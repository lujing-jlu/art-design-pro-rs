use once_cell::sync::Lazy;
use std::env;

/// 上传目录（可通过环境变量 UPLOAD_DIR 配置）
pub static UPLOAD_DIR: Lazy<String> =
    Lazy::new(|| env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string()));

/// 允许的 CORS 来源（逗号分隔），空则允许所有
pub static CORS_ALLOW_ORIGINS: Lazy<Vec<String>> = Lazy::new(|| {
    env::var("CORS_ALLOW_ORIGINS")
        .map(|val| {
            val.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
});
