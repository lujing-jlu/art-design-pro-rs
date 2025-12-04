use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{DatabaseBackend, Statement, Value};
use sea_orm_migration::SchemaManagerConnection;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 基础表
        create_role_table(manager).await?;
        create_user_table(manager).await?;
        create_user_role_table(manager).await?;
        add_gender_column_if_missing(manager).await?;
        add_profile_columns_if_missing(manager).await?;
        create_menu_table(manager).await?;

        // 种子数据
        let db = manager.get_connection();
        seed_roles(db).await?;
        seed_users(db).await?;
        seed_user_roles(db).await?;
        seed_menus(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysMenu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SysUserRole::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SysUser::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SysRole::Table).to_owned())
            .await
    }
}

async fn create_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(SysUser::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(SysUser::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(SysUser::Username)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(SysUser::Password).string().not_null())
                .col(ColumnDef::new(SysUser::Nickname).string().not_null())
                .col(ColumnDef::new(SysUser::Email).string())
                .col(ColumnDef::new(SysUser::Phone).string())
                .col(
                    ColumnDef::new(SysUser::Status)
                        .string()
                        .not_null()
                        .default("1"),
                )
                .col(
                    ColumnDef::new(SysUser::Gender)
                        .string()
                        .not_null()
                        .default("男"),
                )
                .col(ColumnDef::new(SysUser::Avatar).string())
                .col(
                    ColumnDef::new(SysUser::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(SysUser::UpdatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .to_owned(),
        )
        .await
}

async fn create_role_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(SysRole::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(SysRole::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(SysRole::Code)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(SysRole::Name).string().not_null())
                .col(ColumnDef::new(SysRole::Description).string())
                .col(
                    ColumnDef::new(SysRole::Status)
                        .string()
                        .not_null()
                        .default("1"),
                )
                .col(
                    ColumnDef::new(SysRole::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .to_owned(),
        )
        .await
}

async fn create_user_role_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(SysUserRole::Table)
                .if_not_exists()
                .col(ColumnDef::new(SysUserRole::UserId).big_integer().not_null())
                .col(ColumnDef::new(SysUserRole::RoleId).big_integer().not_null())
                .primary_key(
                    Index::create()
                        .col(SysUserRole::UserId)
                        .col(SysUserRole::RoleId),
                )
                .to_owned(),
        )
        .await
}

async fn add_gender_column_if_missing(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    // 尝试添加列，若已存在会报错，忽略即可
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "ALTER TABLE sys_user ADD COLUMN gender TEXT NOT NULL DEFAULT '男'".to_string(),
        ))
        .await;
    Ok(())
}

async fn add_profile_columns_if_missing(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    // 添加用户详细信息字段
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "ALTER TABLE sys_user ADD COLUMN real_name TEXT".to_string(),
        ))
        .await;
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "ALTER TABLE sys_user ADD COLUMN address TEXT".to_string(),
        ))
        .await;
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "ALTER TABLE sys_user ADD COLUMN bio TEXT".to_string(),
        ))
        .await;
    Ok(())
}

async fn create_menu_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(SysMenu::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(SysMenu::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(SysMenu::ParentId).big_integer())
                .col(ColumnDef::new(SysMenu::Path).string().not_null())
                .col(ColumnDef::new(SysMenu::Name).string().not_null())
                .col(ColumnDef::new(SysMenu::Component).string())
                .col(ColumnDef::new(SysMenu::Title).string().not_null())
                .col(ColumnDef::new(SysMenu::Icon).string())
                .col(
                    ColumnDef::new(SysMenu::Sort)
                        .integer()
                        .not_null()
                        .default(1),
                )
                .col(
                    ColumnDef::new(SysMenu::Type)
                        .string()
                        .not_null()
                        .default("menu"),
                )
                .col(ColumnDef::new(SysMenu::Meta).text())
                .col(
                    ColumnDef::new(SysMenu::Status)
                        .string()
                        .not_null()
                        .default("1"),
                )
                .col(
                    ColumnDef::new(SysMenu::CreatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(SysMenu::UpdatedAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .to_owned(),
        )
        .await
}

async fn seed_roles(db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    let roles = vec![
        (
            1,
            "R_SUPER",
            "Super Administrator",
            "Super Administrator with full access",
        ),
        (2, "R_ADMIN", "Administrator", "System Administrator"),
        (3, "R_USER", "User", "Normal User"),
    ];

    for (id, code, name, desc) in roles {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT INTO sys_role (id, code, name, description, status, created_at) VALUES (?, ?, ?, ?, ?, datetime('now'))"#,
            vec![id.into(), code.into(), name.into(), desc.into(), "1".into()],
        ))
        .await?;
    }
    Ok(())
}

async fn seed_users(db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    let users = vec![(1, "Super", "男"), (2, "Admin", "女"), (3, "User", "男")];

    for (id, username, gender) in users {
        let password =
            common::utils::hash_password("123456").map_err(|e| DbErr::Custom(e.to_string()))?;

        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT INTO sys_user (id, username, password, nickname, status, gender, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
            vec![
                id.into(),
                username.into(),
                password.into(),
                username.into(),
                "1".into(),
                gender.into(),
            ],
        ))
        .await?;
    }
    Ok(())
}

async fn seed_user_roles(db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    let relations = vec![(1, 1), (2, 2), (3, 3)];

    for (user_id, role_id) in relations {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT INTO sys_user_role (user_id, role_id) VALUES (?, ?)"#,
            vec![user_id.into(), role_id.into()],
        ))
        .await?;
    }
    Ok(())
}

async fn seed_menus(db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    // 以 sort 确保显示顺序，meta 序列化为 JSON
    let menus: Vec<(
        i64,
        Option<i64>,
        &str,
        &str,
        Option<&str>,
        &str,
        Option<&str>,
        i32,
        &str,
        &str,
    )> = vec![
        // Dashboard
        (
            1,
            None,
            "/dashboard",
            "Dashboard",
            Some("/index/index"),
            "menus.dashboard.title",
            Some("ri:pie-chart-line"),
            1,
            "menu",
            r#"{"roles":["R_SUPER","R_ADMIN","R_USER"]}"#,
        ),
        (
            2,
            Some(1),
            "console",
            "Console",
            Some("/dashboard/console"),
            "menus.dashboard.console",
            None,
            1,
            "menu",
            r#"{"keepAlive":false,"fixedTab":true}"#,
        ),
        // System
        (
            10,
            None,
            "/system",
            "System",
            Some("/index/index"),
            "menus.system.title",
            Some("ri:user-3-line"),
            10,
            "menu",
            r#"{"roles":["R_SUPER","R_ADMIN"]}"#,
        ),
        (
            11,
            Some(10),
            "user",
            "User",
            Some("/system/user"),
            "menus.system.user",
            None,
            1,
            "menu",
            r#"{"keepAlive":true,"roles":["R_SUPER","R_ADMIN"]}"#,
        ),
        (
            12,
            Some(10),
            "role",
            "Role",
            Some("/system/role"),
            "menus.system.role",
            None,
            2,
            "menu",
            r#"{"keepAlive":true,"roles":["R_SUPER"]}"#,
        ),
        (
            13,
            Some(10),
            "user-center",
            "UserCenter",
            Some("/system/user-center"),
            "menus.system.userCenter",
            None,
            3,
            "menu",
            r#"{"keepAlive":true,"isHide":true,"isHideTab":true}"#,
        ),
        (
            14,
            Some(10),
            "menu",
            "Menus",
            Some("/system/menu"),
            "menus.system.menu",
            None,
            4,
            "menu",
            r#"{"keepAlive":true,"roles":["R_SUPER"]}"#,
        ),
        // Result
        (
            20,
            None,
            "/result",
            "Result",
            Some("/index/index"),
            "menus.result.title",
            Some("ri:checkbox-circle-line"),
            20,
            "menu",
            r#"{}"#,
        ),
        (
            21,
            Some(20),
            "success",
            "ResultSuccess",
            Some("/result/success"),
            "menus.result.success",
            Some("ri:checkbox-circle-line"),
            1,
            "menu",
            r#"{"keepAlive":true}"#,
        ),
        (
            22,
            Some(20),
            "fail",
            "ResultFail",
            Some("/result/fail"),
            "menus.result.fail",
            Some("ri:close-circle-line"),
            2,
            "menu",
            r#"{"keepAlive":true}"#,
        ),
        // Exception
        (
            30,
            None,
            "/exception",
            "Exception",
            Some("/index/index"),
            "menus.exception.title",
            Some("ri:error-warning-line"),
            30,
            "menu",
            r#"{}"#,
        ),
        (
            31,
            Some(30),
            "403",
            "Exception403",
            Some("/exception/403"),
            "menus.exception.forbidden",
            None,
            1,
            "menu",
            r#"{"keepAlive":true,"isHideTab":true,"isFullPage":true}"#,
        ),
        (
            32,
            Some(30),
            "404",
            "Exception404",
            Some("/exception/404"),
            "menus.exception.notFound",
            None,
            2,
            "menu",
            r#"{"keepAlive":true,"isHideTab":true,"isFullPage":true}"#,
        ),
        (
            33,
            Some(30),
            "500",
            "Exception500",
            Some("/exception/500"),
            "menus.exception.serverError",
            None,
            3,
            "menu",
            r#"{"keepAlive":true,"isHideTab":true,"isFullPage":true}"#,
        ),
        // Menu auth buttons
        (
            40,
            Some(14),
            "menu_add",
            "MenuAdd",
            None,
            "新增",
            None,
            1,
            "button",
            r#"{"isAuthButton":true,"authMark":"add","parentPath":"/system/menu"}"#,
        ),
        (
            41,
            Some(14),
            "menu_edit",
            "MenuEdit",
            None,
            "编辑",
            None,
            2,
            "button",
            r#"{"isAuthButton":true,"authMark":"edit","parentPath":"/system/menu"}"#,
        ),
        (
            42,
            Some(14),
            "menu_delete",
            "MenuDelete",
            None,
            "删除",
            None,
            3,
            "button",
            r#"{"isAuthButton":true,"authMark":"delete","parentPath":"/system/menu"}"#,
        ),
    ];

    for (id, parent_id, path, name, component, title, icon, sort, menu_type, meta) in menus {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT INTO sys_menu (id, parent_id, path, name, component, title, icon, sort, type, meta, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '1', datetime('now'), datetime('now'))"#,
            vec![
                id.into(),
                Value::BigInt(parent_id),
                path.into(),
                name.into(),
                Value::String(component.map(|v| Box::new(v.to_string()))),
                title.into(),
                Value::String(icon.map(|v| Box::new(v.to_string()))),
                sort.into(),
                menu_type.into(),
                meta.into(),
            ],
        ))
        .await?;
    }
    Ok(())
}

#[derive(Iden)]
enum SysUser {
    Table,
    Id,
    Username,
    Password,
    Nickname,
    Email,
    Phone,
    Status,
    Gender,
    Avatar,
    RealName,
    Address,
    Bio,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum SysRole {
    Table,
    Id,
    Code,
    Name,
    Description,
    Status,
    CreatedAt,
}

#[derive(Iden)]
enum SysUserRole {
    Table,
    UserId,
    RoleId,
}

#[derive(Iden)]
enum SysMenu {
    Table,
    Id,
    ParentId,
    Path,
    Name,
    Component,
    Title,
    Icon,
    Sort,
    Type,
    Meta,
    Status,
    CreatedAt,
    UpdatedAt,
}
