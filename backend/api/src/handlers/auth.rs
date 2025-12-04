use crate::state::AppState;
use actix_web::{get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use common::models::{LoginParams, RegisterParams, ResetPasswordParams};
use common::response::ApiResponse;
use service::auth;

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Json<LoginParams>,
) -> impl Responder {
    let client_ip = req
        .peer_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|| "unknown".into());

    match auth::login(&state.db, params.into_inner()).await {
        Ok(response) => {
            log::info!("[audit] login success user={} ip={}", req.path(), client_ip);
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(err) => {
            log::warn!(
                "[audit] login failed ip={} reason={}",
                client_ip,
                err.to_string()
            );
            HttpResponse::Ok().json(ApiResponse::<()>::error(401, err.to_string()))
        }
    }
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    params: web::Json<RegisterParams>,
) -> impl Responder {
    match auth::register(&state.db, params.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success_with_msg(
            response,
            "注册成功".to_string(),
        )),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[post("/forget-password")]
pub async fn reset_password(
    state: web::Data<AppState>,
    params: web::Json<ResetPasswordParams>,
) -> impl Responder {
    match auth::reset_password(&state.db, params.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success_with_msg(
            response,
            "密码已重置".to_string(),
        )),
        Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
    }
}

#[get("/user/info")]
pub async fn get_user_info(
    state: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // Extract user_id from claims inserted by middleware
    let extensions = req.extensions();
    let claims = extensions.get::<common::utils::Claims>();

    if let Some(claims) = claims {
        match auth::get_user_info(&state.db, claims.uid).await {
            Ok(info) => HttpResponse::Ok().json(ApiResponse::success(info)),
            Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(404, err.to_string())),
        }
    } else {
        HttpResponse::Ok().json(ApiResponse::<()>::error(401, "Unauthorized".to_string()))
    }
}

#[put("/user/profile")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Json<common::models::UpdateProfileParams>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<common::utils::Claims>();

    if let Some(claims) = claims {
        match auth::update_profile(&state.db, claims.uid, params.into_inner()).await {
            Ok(info) => {
                HttpResponse::Ok().json(ApiResponse::success_with_msg(info, "更新成功".to_string()))
            }
            Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
        }
    } else {
        HttpResponse::Ok().json(ApiResponse::<()>::error(401, "未登录".to_string()))
    }
}

#[put("/user/password")]
pub async fn change_password(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Json<common::models::ChangePasswordParams>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<common::utils::Claims>();

    if let Some(claims) = claims {
        match auth::change_password(&state.db, claims.uid, params.into_inner()).await {
            Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success_with_msg(
                (),
                "密码修改成功".to_string(),
            )),
            Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(400, err.to_string())),
        }
    } else {
        HttpResponse::Ok().json(ApiResponse::<()>::error(401, "未登录".to_string()))
    }
}

#[post("/user/avatar")]
pub async fn upload_avatar(
    state: web::Data<AppState>,
    req: HttpRequest,
    upload_dir: web::Data<String>,
    mut payload: actix_multipart::Multipart,
) -> impl Responder {
    use futures::StreamExt;
    use std::{fs, io::Write, path::PathBuf};

    let extensions = req.extensions();
    let claims = extensions.get::<common::utils::Claims>();

    let user_id = match claims {
        Some(c) => c.uid,
        None => {
            return HttpResponse::Ok().json(ApiResponse::<()>::error(401, "未登录".to_string()))
        }
    };

    let upload_dir = PathBuf::from(upload_dir.get_ref().as_str());
    if let Err(err) = fs::create_dir_all(&upload_dir) {
        return HttpResponse::Ok().json(ApiResponse::<()>::error(
            500,
            format!("创建上传目录失败: {}", err),
        ));
    }

    let mut saved_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                return HttpResponse::Ok().json(ApiResponse::<()>::error(
                    400,
                    format!("读取上传流失败: {}", e),
                ));
            }
        };

        let filename = field
            .content_disposition()
            .get_filename()
            .map(|f| {
                // 生成唯一文件名
                let ext = std::path::Path::new(f)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jpg");
                format!(
                    "avatar_{}_{}.{}",
                    user_id,
                    chrono::Utc::now().timestamp(),
                    ext
                )
            })
            .unwrap_or_else(|| format!("avatar_{}.jpg", user_id));

        let filepath = upload_dir.join(&filename);
        let mut f = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(err) => {
                return HttpResponse::Ok().json(ApiResponse::<()>::error(
                    500,
                    format!("保存文件失败: {}", err),
                ));
            }
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(e) => {
                    return HttpResponse::Ok().json(ApiResponse::<()>::error(
                        400,
                        format!("读取文件块失败: {}", e),
                    ));
                }
            };
            if let Err(err) = f.write_all(&data) {
                return HttpResponse::Ok().json(ApiResponse::<()>::error(
                    500,
                    format!("写入文件失败: {}", err),
                ));
            }
        }

        let url = format!("/uploads/{}", filename);
        saved_url = Some(url);
        break;
    }

    if let Some(url) = saved_url {
        match auth::update_avatar(&state.db, user_id, url.clone()).await {
            Ok(info) => HttpResponse::Ok().json(ApiResponse::success_with_msg(
                info,
                "头像上传成功".to_string(),
            )),
            Err(err) => HttpResponse::Ok().json(ApiResponse::<()>::error(500, err.to_string())),
        }
    } else {
        HttpResponse::Ok().json(ApiResponse::<()>::error(400, "未收到文件".to_string()))
    }
}
