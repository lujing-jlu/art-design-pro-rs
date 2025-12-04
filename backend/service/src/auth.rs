use common::error::AppError;
use common::models::{
    ChangePasswordParams, LoginParams, LoginResponse, RegisterParams, RegisterResponse,
    ResetPasswordParams, ResetPasswordResponse, UpdateProfileParams, UserInfo,
};
use common::utils::{generate_token, hash_password, verify_password};
use entity::{role, user, user_role};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::collections::HashSet;
use std::sync::Mutex;
use std::time::{Duration, Instant};

// 简单登录防暴力破解：同一用户名失败 5 次内，每次失败计数，超限锁定 5 分钟
static LOGIN_THROTTLE: Lazy<Mutex<std::collections::HashMap<String, (u32, Instant)>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));
const MAX_FAILED_ATTEMPTS: u32 = 5;
const LOCK_DURATION: Duration = Duration::from_secs(300);

pub async fn login(
    db: &DatabaseConnection,
    params: LoginParams,
) -> Result<LoginResponse, AppError> {
    enforce_login_throttle(&params.userName)?;

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&params.userName))
        .one(db)
        .await?
        .ok_or(AppError::AuthError("User not found".to_string()))?;

    if !verify_password(&params.password, &user.password)? {
        record_failed_login(&params.userName);
        return Err(AppError::AuthError("Invalid password".to_string()));
    }

    clear_login_failures(&params.userName);

    let token = generate_token(&user.username, user.id)?;
    let refresh_token = generate_token(&user.username, user.id)?; // For simplicity, same logic

    Ok(LoginResponse {
        token,
        refreshToken: refresh_token,
    })
}

pub async fn get_user_info(db: &DatabaseConnection, user_id: i64) -> Result<UserInfo, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    // Fetch role ids then fetch roles
    let role_ids: Vec<i64> = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|ur| ur.role_id)
        .collect();

    let role_codes: HashSet<String> = if role_ids.is_empty() {
        HashSet::new()
    } else {
        role::Entity::find()
            .filter(role::Column::Id.is_in(role_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|r| r.code)
            .collect()
    };

    // Default to R_USER when no explicit relation found
    let mut roles: Vec<String> = if role_codes.is_empty() {
        vec!["R_USER".to_string()]
    } else {
        role_codes.into_iter().collect()
    };
    roles.sort();

    // Button permissions derived from roles (demo)
    let mut buttons: Vec<String> = Vec::new();
    if roles.iter().any(|r| r == "R_SUPER" || r == "R_ADMIN") {
        buttons.extend_from_slice(&[
            "UserAdd".to_string(),
            "UserEdit".to_string(),
            "UserDelete".to_string(),
        ]);
    } else {
        buttons.push("UserView".to_string());
    }

    Ok(UserInfo {
        buttons,
        roles,
        userId: user.id as u64,
        userName: user.username,
        email: user.email.clone().unwrap_or_default(),
        avatar: user.avatar.clone(),
        nickName: Some(user.nickname),
        phone: user.phone.clone(),
        gender: Some(user.gender),
        realName: user.real_name.clone(),
        address: user.address.clone(),
        bio: user.bio.clone(),
    })
}

pub async fn register(
    db: &DatabaseConnection,
    params: RegisterParams,
) -> Result<RegisterResponse, AppError> {
    validate_password_strength(&params.password)?;
    // Check if user exists
    let exists = user::Entity::find()
        .filter(user::Column::Username.eq(&params.userName))
        .one(db)
        .await?;

    if exists.is_some() {
        return Err(AppError::AuthError("User already exists".to_string()));
    }

    let password_hash = hash_password(&params.password)?;

    let new_user = user::ActiveModel {
        username: Set(params.userName.clone()),
        password: Set(password_hash),
        nickname: Set(params.userName.clone()),
        email: Set(params.email.clone()),
        phone: Set(None),
        status: Set("1".to_string()),
        avatar: Set(None),
        ..Default::default()
    };

    let inserted = new_user.insert(db).await?;

    // Bind default role R_USER if exists
    if let Some(role_model) = role::Entity::find()
        .filter(role::Column::Code.eq("R_USER"))
        .one(db)
        .await?
    {
        let rel = user_role::ActiveModel {
            user_id: Set(inserted.id),
            role_id: Set(role_model.id),
        };
        let _ = rel.insert(db).await;
    }

    Ok(RegisterResponse {
        userId: inserted.id as u64,
        userName: inserted.username,
    })
}

pub async fn reset_password(
    db: &DatabaseConnection,
    params: ResetPasswordParams,
) -> Result<ResetPasswordResponse, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&params.userName))
        .one(db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let new_password = params
        .newPassword
        .clone()
        .unwrap_or_else(|| "123456Aa".to_string());
    validate_password_strength(&new_password)?;
    let password_hash = hash_password(&new_password)?;

    let mut active_model: user::ActiveModel = user.into();
    active_model.password = Set(password_hash);
    let updated = active_model.update(db).await?;

    Ok(ResetPasswordResponse {
        userId: updated.id as u64,
        userName: updated.username,
        message: format!("密码已重置为{}", new_password),
    })
}

fn validate_password_strength(pwd: &str) -> Result<(), AppError> {
    if pwd.len() < 8 {
        return Err(AppError::Validation(
            "密码长度至少 8 位，需包含字母和数字".into(),
        ));
    }
    let re = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d).+$").unwrap();
    if !re.is_match(pwd) {
        return Err(AppError::Validation("密码需同时包含字母和数字".into()));
    }
    Ok(())
}

fn enforce_login_throttle(username: &str) -> Result<(), AppError> {
    let mut map = LOGIN_THROTTLE.lock().unwrap();
    if let Some((count, since)) = map.get(username) {
        if *count >= MAX_FAILED_ATTEMPTS && since.elapsed() < LOCK_DURATION {
            return Err(AppError::AuthError("尝试过多，请 5 分钟后再试".to_string()));
        }
        if since.elapsed() >= LOCK_DURATION {
            map.remove(username);
        }
    }
    Ok(())
}

fn record_failed_login(username: &str) {
    let mut map = LOGIN_THROTTLE.lock().unwrap();
    let entry = map
        .entry(username.to_string())
        .or_insert((0, Instant::now()));
    if entry.1.elapsed() >= LOCK_DURATION {
        *entry = (1, Instant::now());
    } else {
        entry.0 += 1;
    }
}

fn clear_login_failures(username: &str) {
    let mut map = LOGIN_THROTTLE.lock().unwrap();
    map.remove(username);
}

/// 更新当前用户个人信息
pub async fn update_profile(
    db: &DatabaseConnection,
    user_id: i64,
    params: UpdateProfileParams,
) -> Result<UserInfo, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound("用户不存在".into()))?;

    let mut active: user::ActiveModel = user.into();

    if let Some(real_name) = params.realName {
        active.real_name = Set(Some(real_name));
    }
    if let Some(nick_name) = params.nickName {
        active.nickname = Set(nick_name);
    }
    if let Some(email) = params.email {
        active.email = Set(Some(email));
    }
    if let Some(phone) = params.phone {
        active.phone = Set(Some(phone));
    }
    if let Some(gender) = params.gender {
        active.gender = Set(gender);
    }
    if let Some(address) = params.address {
        active.address = Set(Some(address));
    }
    if let Some(bio) = params.bio {
        active.bio = Set(Some(bio));
    }

    let updated = active.update(db).await?;

    // 返回更新后的用户信息
    get_user_info(db, updated.id).await
}

/// 修改当前用户密码
pub async fn change_password(
    db: &DatabaseConnection,
    user_id: i64,
    params: ChangePasswordParams,
) -> Result<(), AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound("用户不存在".into()))?;

    // 验证旧密码
    if !verify_password(&params.oldPassword, &user.password)? {
        return Err(AppError::AuthError("当前密码不正确".into()));
    }

    // 验证新密码强度
    validate_password_strength(&params.newPassword)?;

    // 哈希新密码
    let new_password_hash = hash_password(&params.newPassword)?;

    // 更新密码
    let mut active: user::ActiveModel = user.into();
    active.password = Set(new_password_hash);
    active.update(db).await?;

    Ok(())
}

/// 更新用户头像
pub async fn update_avatar(
    db: &DatabaseConnection,
    user_id: i64,
    avatar_url: String,
) -> Result<UserInfo, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound("用户不存在".into()))?;

    let mut active: user::ActiveModel = user.into();
    active.avatar = Set(Some(avatar_url));
    let updated = active.update(db).await?;

    get_user_info(db, updated.id).await
}
