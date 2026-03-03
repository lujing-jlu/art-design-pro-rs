use common::utils::hash_password;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, Statement, Value};

pub async fn initialize_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    create_tables(db).await?;
    seed_roles(db).await?;
    seed_users(db).await?;
    seed_user_roles(db).await?;
    seed_menus(db).await?;
    Ok(())
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.execute(sql(
        r#"
        CREATE TABLE IF NOT EXISTS sys_role (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT '1',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    ))
    .await?;

    db.execute(sql(
        r#"
        CREATE TABLE IF NOT EXISTS sys_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            nickname TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            status TEXT NOT NULL DEFAULT '1',
            gender TEXT NOT NULL DEFAULT '男',
            avatar TEXT,
            real_name TEXT,
            address TEXT,
            bio TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    ))
    .await?;

    db.execute(sql(
        r#"
        CREATE TABLE IF NOT EXISTS sys_user_role (
            user_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            PRIMARY KEY (user_id, role_id),
            FOREIGN KEY (user_id) REFERENCES sys_user(id) ON UPDATE CASCADE ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES sys_role(id) ON UPDATE CASCADE ON DELETE CASCADE
        )
        "#,
    ))
    .await?;

    db.execute(sql(
        r#"
        CREATE TABLE IF NOT EXISTS sys_menu (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parent_id INTEGER,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            component TEXT,
            title TEXT NOT NULL,
            icon TEXT,
            sort INTEGER NOT NULL DEFAULT 1,
            type TEXT NOT NULL DEFAULT 'menu',
            meta TEXT,
            status TEXT NOT NULL DEFAULT '1',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    ))
    .await?;

    Ok(())
}

async fn seed_roles(db: &DatabaseConnection) -> Result<(), DbErr> {
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
            r#"INSERT OR IGNORE INTO sys_role (id, code, name, description, status, created_at) VALUES (?, ?, ?, ?, ?, datetime('now'))"#,
            vec![id.into(), code.into(), name.into(), desc.into(), "1".into()],
        ))
        .await?;
    }

    Ok(())
}

async fn seed_users(db: &DatabaseConnection) -> Result<(), DbErr> {
    let users = vec![(1, "Super", "男"), (2, "Admin", "女"), (3, "User", "男")];

    for (id, username, gender) in users {
        let password = hash_password("123456").map_err(|e| DbErr::Custom(e.to_string()))?;

        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT OR IGNORE INTO sys_user (id, username, password, nickname, status, gender, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"#,
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

async fn seed_user_roles(db: &DatabaseConnection) -> Result<(), DbErr> {
    let relations = vec![(1, 1), (2, 2), (3, 3)];

    for (user_id, role_id) in relations {
        db.execute(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"INSERT OR IGNORE INTO sys_user_role (user_id, role_id) VALUES (?, ?)"#,
            vec![user_id.into(), role_id.into()],
        ))
        .await?;
    }

    Ok(())
}

async fn seed_menus(db: &DatabaseConnection) -> Result<(), DbErr> {
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
            r#"INSERT OR IGNORE INTO sys_menu (id, parent_id, path, name, component, title, icon, sort, type, meta, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, '1', datetime('now'), datetime('now'))"#,
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

fn sql(raw: &str) -> Statement {
    Statement::from_string(DatabaseBackend::Sqlite, raw.to_string())
}

