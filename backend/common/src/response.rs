use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_msg(data: T, msg: String) -> Self {
        Self {
            code: 200,
            msg,
            data: Some(data),
        }
    }

    pub fn error(code: i32, msg: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            msg,
            data: None,
        }
    }
}
