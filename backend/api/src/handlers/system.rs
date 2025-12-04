use crate::state::AppState;
use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use common::models::{
    MenuPayload, RoleCreateParams, RoleSearchParams, RoleUpdateParams, UserCreateParams,
    UserSearchParams, UserUpdateParams,
};
use common::response::ApiResponse;
use common::utils::Claims;
use service::system;

#[get("/list")]
pub async fn get_user_list(
    state: web::Data<AppState>,
    params: web::Query<UserSearchParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER", "R_ADMIN"]).await {
        return resp;
    }

    match system::get_user_list(&state.db, params.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(list)),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())),
    }
}

#[post("/")]
pub async fn create_user(
    state: web::Data<AppState>,
    payload: web::Json<UserCreateParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER", "R_ADMIN"]).await {
        return resp;
    }

    match system::create_user(&state.db, payload.into_inner()).await {
        Ok(user) => {
            log::info!(
                "[audit] user_add actor={:?} target_id={}",
                get_actor(&req),
                user.id
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(
                user,
                "新增用户成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[put("/")]
pub async fn update_user(
    state: web::Data<AppState>,
    payload: web::Json<UserUpdateParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER", "R_ADMIN"]).await {
        return resp;
    }

    match system::update_user(&state.db, payload.into_inner()).await {
        Ok(user) => {
            log::info!(
                "[audit] user_update actor={:?} target_id={}",
                get_actor(&req),
                user.id
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(
                user,
                "更新用户成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[delete("/{id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<i64>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER", "R_ADMIN"]).await {
        return resp;
    }
    let target_id = path.into_inner();
    match system::delete_user(&state.db, target_id).await {
        Ok(_) => {
            log::info!(
                "[audit] user_delete actor={:?} target_id={}",
                get_actor(&req),
                target_id
            );
            HttpResponse::Ok().json(ApiResponse::<()>::success_with_msg(
                (),
                "删除用户成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[get("/list")]
pub async fn get_role_list(
    state: web::Data<AppState>,
    params: web::Query<RoleSearchParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER", "R_ADMIN"]).await {
        return resp;
    }

    match system::get_role_list(&state.db, params.into_inner()).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(list)),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())),
    }
}

#[post("/")]
pub async fn create_role(
    state: web::Data<AppState>,
    payload: web::Json<RoleCreateParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    match system::create_role(&state.db, payload.into_inner()).await {
        Ok(role) => {
            log::info!(
                "[audit] role_add actor={:?} target_id={}",
                get_actor(&req),
                role.roleId
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(
                role,
                "新增角色成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[put("/")]
pub async fn update_role(
    state: web::Data<AppState>,
    payload: web::Json<RoleUpdateParams>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    match system::update_role(&state.db, payload.into_inner()).await {
        Ok(role) => {
            log::info!(
                "[audit] role_update actor={:?} target_id={}",
                get_actor(&req),
                role.roleId
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(
                role,
                "更新角色成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[delete("/{id}")]
pub async fn delete_role(
    state: web::Data<AppState>,
    path: web::Path<i64>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    let target_id = path.into_inner();
    match system::delete_role(&state.db, target_id).await {
        Ok(_) => {
            log::info!(
                "[audit] role_delete actor={:?} target_id={}",
                get_actor(&req),
                target_id
            );
            HttpResponse::Ok().json(ApiResponse::<()>::success_with_msg(
                (),
                "删除角色成功".to_string(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[get("/v3/system/menus/simple")]
pub async fn get_menu_list(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let roles = match get_request_roles(&state, &req).await {
        Ok(r) => r,
        Err(resp) => return resp,
    };

    match system::get_menu_list(&state.db, &roles).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(list)),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())),
    }
}

#[get("/list")]
pub async fn admin_menu_list(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    match system::get_all_menus(&state.db).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse::success(list)),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())),
    }
}

#[post("/")]
pub async fn create_menu(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<MenuPayload>,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    match system::create_menu(&state.db, payload.into_inner()).await {
        Ok(menu) => {
            log::info!(
                "[audit] menu_add actor={:?} target_id={:?}",
                get_actor(&req),
                menu.id
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(menu, "新增菜单成功".into()))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[put("/")]
pub async fn update_menu(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<MenuPayload>,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    match system::update_menu(&state.db, payload.into_inner()).await {
        Ok(menu) => {
            log::info!(
                "[audit] menu_update actor={:?} target_id={:?}",
                get_actor(&req),
                menu.id
            );
            HttpResponse::Ok().json(ApiResponse::success_with_msg(menu, "更新菜单成功".into()))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[delete("/{id}")]
pub async fn delete_menu(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(resp) = ensure_admin_roles(&state, &req, &["R_SUPER"]).await {
        return resp;
    }

    let target_id = path.into_inner();
    match system::delete_menu(&state.db, target_id).await {
        Ok(_) => {
            log::info!(
                "[audit] menu_delete actor={:?} target_id={}",
                get_actor(&req),
                target_id
            );
            HttpResponse::Ok().json(ApiResponse::<()>::success_with_msg(
                (),
                "删除菜单成功".into(),
            ))
        }
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

/// 从请求中提取用户角色并进行鉴权
async fn ensure_admin_roles(
    state: &web::Data<AppState>,
    req: &HttpRequest,
    allowed: &[&str],
) -> Result<(), HttpResponse> {
    let roles = get_request_roles(state, req).await?;
    let has_permission = roles
        .iter()
        .any(|role| allowed.iter().any(|allowed_role| role == allowed_role));

    if has_permission {
        Ok(())
    } else {
        Err(HttpResponse::Ok().json(ApiResponse::<()>::error(403, "无权访问".to_string())))
    }
}

/// 获取请求中的用户角色，未登录返回 401
async fn get_request_roles(
    state: &web::Data<AppState>,
    req: &HttpRequest,
) -> Result<Vec<String>, HttpResponse> {
    let claims = get_claims(req)?;
    system::get_user_roles(&state.db, claims.uid)
        .await
        .map_err(|err| HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())))
}

fn get_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let extensions = req.extensions();
    extensions
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| HttpResponse::Ok().json(ApiResponse::<()>::error(401, "未登录".to_string())))
}

fn get_actor(req: &HttpRequest) -> Option<String> {
    req.extensions()
        .get::<Claims>()
        .map(|c| format!("uid={},user={}", c.uid, c.sub))
}
