use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use dotenvy::dotenv;
use env_logger;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::{env, fs};

mod config;
mod handlers;
mod middleware;
use middleware as api_middleware;
mod state;

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db?mode=rwc".to_owned());

    log::info!("Connecting to database at {}", database_url);
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    log::info!("Running migrations...");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db };
    // 确保上传目录存在，避免 actix-files 报错
    let _ = fs::create_dir_all(config::UPLOAD_DIR.as_str());

    log::info!("Starting server at http://127.0.0.1:9123");

    HttpServer::new(move || {
        let cors = {
            let mut builder = Cors::default()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);
            if config::CORS_ALLOW_ORIGINS.is_empty() {
                builder = builder.allow_any_origin();
            } else {
                for origin in config::CORS_ALLOW_ORIGINS.iter() {
                    builder = builder.allowed_origin(origin);
                }
            }
            builder
        };

        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(config::UPLOAD_DIR.clone()))
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .service(
                web::scope("/api")
                    // 公开接口
                    .service(
                        web::scope("/auth")
                            .service(handlers::auth::login)
                            .service(handlers::auth::register)
                            .service(handlers::auth::reset_password),
                    )
                    // 需要认证的接口
                    .service(
                        web::scope("")
                            .wrap(api_middleware::auth::Auth)
                            // 通用
                            .service(handlers::common::upload_wangeditor)
                            .service(handlers::auth::get_user_info)
                            .service(handlers::auth::update_profile)
                            .service(handlers::auth::change_password)
                            .service(handlers::auth::upload_avatar)
                            // 用户
                            .service(
                                web::scope("/user")
                                    .service(handlers::system::get_user_list)
                                    .service(handlers::system::create_user)
                                    .service(handlers::system::update_user)
                                    .service(handlers::system::delete_user),
                            )
                            // 角色
                            .service(
                                web::scope("/role")
                                    .service(handlers::system::get_role_list)
                                    .service(handlers::system::create_role)
                                    .service(handlers::system::update_role)
                                    .service(handlers::system::delete_role),
                            )
                            // 菜单
                            .service(
                                web::scope("/menu")
                                    .service(handlers::system::admin_menu_list)
                                    .service(handlers::system::create_menu)
                                    .service(handlers::system::update_menu)
                                    .service(handlers::system::delete_menu),
                            )
                            // 前端动态路由/菜单树
                            .service(handlers::system::get_menu_list),
                    ),
            )
            .service(Files::new("/uploads", config::UPLOAD_DIR.as_str()).show_files_listing())
    })
    .bind(("127.0.0.1", 9123))?
    .run()
    .await
}
