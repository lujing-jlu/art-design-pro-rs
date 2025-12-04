use common::error::AppError;
use common::models::{
    AppRouteRecord, MenuPayload, PaginatedResponse, RoleCreateParams, RoleList, RoleListItem,
    RoleSearchParams, RoleUpdateParams, RouteMeta, UserCreateParams, UserList, UserListItem,
    UserSearchParams, UserUpdateParams,
};
use entity::{menu, role, user, user_role};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use serde_json;
use std::collections::{HashMap, HashSet};

// ---------------- 用户 ----------------

pub async fn get_user_list(
    db: &DatabaseConnection,
    params: UserSearchParams,
) -> Result<UserList, AppError> {
    let current = params.current.unwrap_or(1);
    let size = params.size.unwrap_or(10);

    let mut query = user::Entity::find();

    if let Some(username) = params.userName {
        query = query.filter(user::Column::Username.contains(&username));
    }
    if let Some(phone) = params.userPhone {
        query = query.filter(user::Column::Phone.contains(&phone));
    }
    if let Some(email) = params.userEmail {
        query = query.filter(user::Column::Email.contains(&email));
    }
    if let Some(status) = params.status {
        query = query.filter(user::Column::Status.eq(status));
    }
    if let Some(gender) = params.userGender {
        query = query.filter(user::Column::Gender.eq(gender));
    }
    if let Some(id) = params.id {
        query = query.filter(user::Column::Id.eq(id as i64));
    }

    let paginator = query.paginate(db, size as u64);
    let total = paginator.num_items().await?;
    let records = paginator.fetch_page(current as u64 - 1).await?;

    // 预先查询所有用户的角色映射
    let user_ids: Vec<i64> = records.iter().map(|u| u.id).collect();
    let mut user_roles_map: HashMap<i64, Vec<String>> = HashMap::new();
    if !user_ids.is_empty() {
        let relations = user_role::Entity::find()
            .filter(user_role::Column::UserId.is_in(user_ids.clone()))
            .all(db)
            .await?;

        let role_ids: HashSet<i64> = relations.iter().map(|r| r.role_id).collect();
        let role_map: HashMap<i64, String> = if role_ids.is_empty() {
            HashMap::new()
        } else {
            role::Entity::find()
                .filter(role::Column::Id.is_in(role_ids))
                .all(db)
                .await?
                .into_iter()
                .map(|r| (r.id, r.code))
                .collect()
        };

        for rel in relations {
            let code = role_map
                .get(&rel.role_id)
                .cloned()
                .unwrap_or_else(|| "R_USER".to_string());
            user_roles_map.entry(rel.user_id).or_default().push(code);
        }
    }

    let user_list_items = records
        .into_iter()
        .map(|u| UserListItem {
            id: u.id as u64,
            avatar: u.avatar.unwrap_or_default(),
            status: u.status,
            userName: u.username,
            userGender: u.gender,
            nickName: u.nickname,
            userPhone: u.phone.unwrap_or_default(),
            userEmail: u.email.unwrap_or_default(),
            userRoles: user_roles_map
                .get(&u.id)
                .cloned()
                .unwrap_or_else(|| vec!["R_USER".to_string()]),
            createBy: "system".to_string(),
            createTime: u.created_at.to_string(),
            updateBy: "system".to_string(),
            updateTime: u.updated_at.to_string(),
        })
        .collect();

    Ok(PaginatedResponse {
        total: total as u32,
        size,
        current,
        records: user_list_items,
    })
}

pub async fn create_user(
    db: &DatabaseConnection,
    params: UserCreateParams,
) -> Result<UserListItem, AppError> {
    let txn = db.begin().await?;

    if user::Entity::find()
        .filter(user::Column::Username.eq(&params.userName))
        .one(&txn)
        .await?
        .is_some()
    {
        return Err(AppError::Validation("用户名已存在".into()));
    }

    let password_hash = common::utils::hash_password("123456")?;

    let new_user = user::ActiveModel {
        username: Set(params.userName.clone()),
        password: Set(password_hash),
        nickname: Set(params.userName.clone()),
        email: Set(params.email.clone()),
        phone: Set(params.phone.clone()),
        status: Set(params.status.unwrap_or_else(|| "1".to_string())),
        gender: Set(params.userGender.unwrap_or_else(|| "男".to_string())),
        avatar: Set(None),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    bind_roles_by_codes(&txn, new_user.id, params.roleCodes).await?;
    txn.commit().await?;

    Ok(UserListItem {
        id: new_user.id as u64,
        avatar: new_user.avatar.unwrap_or_default(),
        status: new_user.status,
        userName: new_user.username,
        userGender: "1".to_string(),
        nickName: new_user.nickname,
        userPhone: new_user.phone.unwrap_or_default(),
        userEmail: new_user.email.unwrap_or_default(),
        userRoles: get_role_codes_for_user(db, new_user.id).await?,
        createBy: "system".to_string(),
        createTime: new_user.created_at.to_string(),
        updateBy: "system".to_string(),
        updateTime: new_user.updated_at.to_string(),
    })
}

pub async fn update_user(
    db: &DatabaseConnection,
    params: UserUpdateParams,
) -> Result<UserListItem, AppError> {
    let txn = db.begin().await?;

    let model = user::Entity::find_by_id(params.id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".into()))?;

    let mut active: user::ActiveModel = model.into();
    if let Some(phone) = params.phone {
        active.phone = Set(Some(phone));
    }
    if let Some(email) = params.email {
        active.email = Set(Some(email));
    }
    if let Some(status) = params.status {
        active.status = Set(status);
    }
    if let Some(gender) = params.userGender {
        active.gender = Set(gender);
    }

    let updated = active.update(&txn).await?;

    user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(updated.id))
        .exec(&txn)
        .await?;
    bind_roles_by_codes(&txn, updated.id, params.roleCodes).await?;

    txn.commit().await?;

    Ok(UserListItem {
        id: updated.id as u64,
        avatar: updated.avatar.unwrap_or_default(),
        status: updated.status,
        userName: updated.username,
        userGender: "1".to_string(),
        nickName: updated.nickname,
        userPhone: updated.phone.unwrap_or_default(),
        userEmail: updated.email.unwrap_or_default(),
        userRoles: get_role_codes_for_user(db, updated.id).await?,
        createBy: "system".to_string(),
        createTime: updated.created_at.to_string(),
        updateBy: "system".to_string(),
        updateTime: updated.updated_at.to_string(),
    })
}

pub async fn delete_user(db: &DatabaseConnection, user_id: i64) -> Result<(), AppError> {
    let txn = db.begin().await?;
    user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(user_id))
        .exec(&txn)
        .await?;
    let res = user::Entity::delete_by_id(user_id).exec(&txn).await?;
    if res.rows_affected == 0 {
        return Err(AppError::NotFound("用户不存在".into()));
    }
    txn.commit().await?;
    Ok(())
}

// ---------------- 角色 ----------------

pub async fn get_role_list(
    db: &DatabaseConnection,
    params: RoleSearchParams,
) -> Result<RoleList, AppError> {
    let current = params.current.unwrap_or(1);
    let size = params.size.unwrap_or(10);

    let mut query = role::Entity::find();

    if let Some(name) = params.roleName {
        query = query.filter(role::Column::Name.contains(&name));
    }
    if let Some(code) = params.roleCode {
        query = query.filter(role::Column::Code.contains(&code));
    }
    if let Some(enabled) = params.enabled {
        query = query.filter(role::Column::Status.eq(if enabled { "1" } else { "0" }));
    }

    let paginator = query.paginate(db, size as u64);
    let total = paginator.num_items().await?;
    let records = paginator.fetch_page(current as u64 - 1).await?;

    let role_list_items = records
        .into_iter()
        .map(|r| RoleListItem {
            roleId: r.id as u64,
            roleName: r.name,
            roleCode: r.code,
            description: r.description.unwrap_or_default(),
            enabled: r.status == "1",
            createTime: r.created_at.to_string(),
        })
        .collect();

    Ok(PaginatedResponse {
        total: total as u32,
        size,
        current,
        records: role_list_items,
    })
}

pub async fn create_role(
    db: &DatabaseConnection,
    params: RoleCreateParams,
) -> Result<RoleListItem, AppError> {
    if role::Entity::find()
        .filter(role::Column::Code.eq(&params.roleCode))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::Validation("角色编码已存在".into()));
    }

    let model = role::ActiveModel {
        code: Set(params.roleCode.clone()),
        name: Set(params.roleName.clone()),
        description: Set(params.description.clone()),
        status: Set(if params.enabled.unwrap_or(true) {
            "1".into()
        } else {
            "0".into()
        }),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(RoleListItem {
        roleId: model.id as u64,
        roleName: model.name,
        roleCode: model.code,
        description: model.description.unwrap_or_default(),
        enabled: model.status == "1",
        createTime: model.created_at.to_string(),
    })
}

pub async fn update_role(
    db: &DatabaseConnection,
    params: RoleUpdateParams,
) -> Result<RoleListItem, AppError> {
    let model = role::Entity::find_by_id(params.roleId)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("角色不存在".into()))?;

    let mut active: role::ActiveModel = model.into();
    active.name = Set(params.roleName.clone());
    active.code = Set(params.roleCode.clone());
    active.description = Set(params.description.clone());
    active.status = Set(if params.enabled.unwrap_or(true) {
        "1".into()
    } else {
        "0".into()
    });

    let updated = active.update(db).await?;

    Ok(RoleListItem {
        roleId: updated.id as u64,
        roleName: updated.name,
        roleCode: updated.code,
        description: updated.description.unwrap_or_default(),
        enabled: updated.status == "1",
        createTime: updated.created_at.to_string(),
    })
}

pub async fn delete_role(db: &DatabaseConnection, role_id: i64) -> Result<(), AppError> {
    let txn = db.begin().await?;
    user_role::Entity::delete_many()
        .filter(user_role::Column::RoleId.eq(role_id))
        .exec(&txn)
        .await?;
    let res = role::Entity::delete_by_id(role_id).exec(&txn).await?;
    if res.rows_affected == 0 {
        return Err(AppError::NotFound("角色不存在".into()));
    }
    txn.commit().await?;
    Ok(())
}

// ---------------- 菜单 ----------------

pub async fn get_menu_list(
    db: &DatabaseConnection,
    roles: &[String],
) -> Result<Vec<AppRouteRecord>, AppError> {
    let routes = load_menu_tree(db).await?;
    Ok(filter_menu_by_roles(routes, roles))
}

/// 管理端读取菜单（不做角色过滤）
pub async fn get_all_menus(db: &DatabaseConnection) -> Result<Vec<AppRouteRecord>, AppError> {
    load_menu_tree(db).await
}

/// 新建菜单/按钮
pub async fn create_menu(
    db: &DatabaseConnection,
    payload: MenuPayload,
) -> Result<AppRouteRecord, AppError> {
    let meta = payload.meta.clone().unwrap_or(RouteMeta {
        title: payload.title.clone(),
        ..Default::default()
    });
    let meta_json = serde_json::to_string(&meta)
        .map_err(|e| AppError::Validation(format!("meta 序列化失败: {}", e)))?;

    if menu::Entity::find()
        .filter(menu::Column::Path.eq(&payload.path))
        .filter(menu::Column::ParentId.eq(payload.parentId))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::Validation("同级菜单 path 已存在".into()));
    }

    let model = menu::ActiveModel {
        parent_id: Set(payload.parentId),
        path: Set(payload.path.clone()),
        name: Set(payload.name.clone()),
        component: Set(payload.component.clone()),
        title: Set(payload.title.clone()),
        icon: Set(payload.icon.clone()),
        sort: Set(payload.sort.unwrap_or(1)),
        r#type: Set(payload.r#type.unwrap_or_else(|| "menu".to_string())),
        meta: Set(Some(meta_json)),
        status: Set("1".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    menu_record_to_route(&model)
}

/// 更新菜单/按钮
pub async fn update_menu(
    db: &DatabaseConnection,
    payload: MenuPayload,
) -> Result<AppRouteRecord, AppError> {
    let id = payload
        .id
        .ok_or_else(|| AppError::Validation("缺少 id".into()))?;
    let mut model = menu::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("菜单不存在".into()))?;

    model.parent_id = payload.parentId;
    model.path = payload.path.clone();
    model.name = payload.name.clone();
    model.component = payload.component.clone();
    model.title = payload.title.clone();
    model.icon = payload.icon.clone();
    model.sort = payload.sort.unwrap_or(1);
    model.r#type = payload.r#type.unwrap_or_else(|| "menu".to_string());
    let meta = payload.meta.clone().unwrap_or(RouteMeta {
        title: payload.title.clone(),
        ..Default::default()
    });
    model.meta = Some(
        serde_json::to_string(&meta)
            .map_err(|e| AppError::Validation(format!("meta 序列化失败: {}", e)))?,
    );

    let active: menu::ActiveModel = model.into();
    let updated = active.update(db).await?;

    menu_record_to_route(&updated)
}

/// 删除菜单（包含子节点）
pub async fn delete_menu(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let ids = collect_descendant_ids(db, id).await?;
    for delete_id in ids {
        menu::Entity::delete_by_id(delete_id).exec(db).await?;
    }
    Ok(())
}

// ---------------- 辅助函数 ----------------

async fn bind_roles_by_codes(
    db: &impl ConnectionTrait,
    user_id: i64,
    codes: Vec<String>,
) -> Result<(), AppError> {
    if codes.is_empty() {
        return Ok(());
    }

    let role_models = role::Entity::find()
        .filter(role::Column::Code.is_in(codes))
        .all(db)
        .await?;

    for r in role_models {
        user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(r.id),
        }
        .insert(db)
        .await?;
    }
    Ok(())
}

pub async fn get_role_codes_for_user(
    db: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<String>, AppError> {
    let ids: Vec<i64> = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .into_iter()
        .map(|r| r.role_id)
        .collect();

    if ids.is_empty() {
        return Ok(vec![]);
    }

    let mut codes: Vec<String> = role::Entity::find()
        .filter(role::Column::Id.is_in(ids))
        .all(db)
        .await?
        .into_iter()
        .map(|r| r.code)
        .collect();
    codes.sort();
    Ok(codes)
}

/// 获取用户的角色编码列表，若无绑定则默认返回 R_USER
pub async fn get_user_roles(
    db: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<String>, AppError> {
    let mut codes = get_role_codes_for_user(db, user_id).await?;
    if codes.is_empty() {
        codes.push("R_USER".to_string());
    }
    Ok(codes)
}

/// 按角色过滤菜单树
fn filter_menu_by_roles(menu: Vec<AppRouteRecord>, roles: &[String]) -> Vec<AppRouteRecord> {
    menu.into_iter()
        .filter_map(|mut item| {
            let children = item
                .children
                .take()
                .map(|child| filter_menu_by_roles(child, roles));

            let self_allowed = match &item.meta.roles {
                Some(required) => required.iter().any(|code| roles.contains(code)),
                None => true,
            };

            let has_children = children.as_ref().map(|c| !c.is_empty()).unwrap_or(false);

            if self_allowed || has_children {
                item.children = children.filter(|c| !c.is_empty());
                Some(item)
            } else {
                None
            }
        })
        .collect()
}

/// 从数据库加载完整菜单树
async fn load_menu_tree(db: &DatabaseConnection) -> Result<Vec<AppRouteRecord>, AppError> {
    let records = menu::Entity::find()
        .order_by_asc(menu::Column::Sort)
        .order_by_asc(menu::Column::Id)
        .all(db)
        .await?;

    let mut nodes: HashMap<i64, AppRouteRecord> = HashMap::new();
    let mut parents: HashMap<i64, Option<i64>> = HashMap::new();

    for record in records {
        parents.insert(record.id, record.parent_id);
        let node = menu_record_to_route(&record)?;
        nodes.insert(record.id, node);
    }

    // 组装父子关系
    let mut result_map: HashMap<i64, AppRouteRecord> = nodes.clone();
    for (id, node) in nodes {
        if let Some(Some(parent_id)) = parents.get(&id) {
            if let Some(parent) = result_map.get_mut(parent_id) {
                let mut children = parent.children.take().unwrap_or_default();
                children.push(node);
                parent.children = Some(children);
            }
        }
    }

    let mut roots: Vec<AppRouteRecord> = result_map
        .into_iter()
        .filter(|(id, _)| parents.get(id).map(|p| p.is_none()).unwrap_or(true))
        .map(|(_, node)| node)
        .collect();

    roots.sort_by_key(|r| r.meta.sort.unwrap_or(0));
    for root in roots.iter_mut() {
        sort_children_recursively(root);
    }

    Ok(roots)
}

fn sort_children_recursively(node: &mut AppRouteRecord) {
    if let Some(children) = node.children.as_mut() {
        children.sort_by_key(|c| c.meta.sort.unwrap_or(0));
        for child in children.iter_mut() {
            sort_children_recursively(child);
        }
    }
}

fn menu_record_to_route(model: &menu::Model) -> Result<AppRouteRecord, AppError> {
    let mut meta: RouteMeta = if let Some(meta_json) = &model.meta {
        serde_json::from_str(meta_json).unwrap_or_default()
    } else {
        RouteMeta {
            title: model.title.clone(),
            ..Default::default()
        }
    };

    meta.title = if meta.title.is_empty() {
        model.title.clone()
    } else {
        meta.title
    };
    meta.icon = meta.icon.or(model.icon.clone());
    meta.sort = Some(model.sort);

    Ok(AppRouteRecord {
        id: Some(model.id),
        path: model.path.clone(),
        name: model.name.clone(),
        redirect: None,
        meta,
        children: None,
        component: model.component.clone(),
    })
}

async fn collect_descendant_ids(db: &DatabaseConnection, id: i64) -> Result<Vec<i64>, AppError> {
    let mut result = Vec::new();
    let mut stack = vec![id];

    while let Some(current) = stack.pop() {
        result.push(current);
        let children = menu::Entity::find()
            .filter(menu::Column::ParentId.eq(current))
            .all(db)
            .await?;
        for child in children {
            stack.push(child.id);
        }
    }

    Ok(result)
}
