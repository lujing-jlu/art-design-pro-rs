use serde::{Deserialize, Serialize};

// --- Common Types ---

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub current: u32,
    pub size: u32,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub records: Vec<T>,
    pub current: u32,
    pub size: u32,
    pub total: u32,
}

// --- Auth Types ---

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub userName: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UpdateProfileParams {
    pub realName: Option<String>,
    pub nickName: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub bio: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ChangePasswordParams {
    pub oldPassword: String,
    pub newPassword: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RegisterParams {
    pub userName: String,
    pub password: String,
    #[serde(default)]
    pub email: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refreshToken: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub buttons: Vec<String>,
    pub roles: Vec<String>,
    pub userId: u64,
    pub userName: String,
    pub email: String,
    pub avatar: Option<String>,
    pub nickName: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub realName: Option<String>,
    pub address: Option<String>,
    pub bio: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ResetPasswordParams {
    pub userName: String,
    #[serde(default)]
    pub newPassword: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub userId: u64,
    pub userName: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub userId: u64,
    pub userName: String,
    pub message: String,
}

// --- System Manage Types ---

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListItem {
    pub id: u64,
    pub avatar: String,
    pub status: String,
    pub userName: String,
    pub userGender: String,
    pub nickName: String,
    pub userPhone: String,
    pub userEmail: String,
    pub userRoles: Vec<String>,
    pub createBy: String,
    pub createTime: String,
    pub updateBy: String,
    pub updateTime: String,
}

pub type UserList = PaginatedResponse<UserListItem>;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserSearchParams {
    pub current: Option<u32>,
    pub size: Option<u32>,
    pub id: Option<u64>,
    pub userName: Option<String>,
    pub userGender: Option<String>,
    pub userPhone: Option<String>,
    pub userEmail: Option<String>,
    pub status: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListItem {
    pub roleId: u64,
    pub roleName: String,
    pub roleCode: String,
    pub description: String,
    pub enabled: bool,
    pub createTime: String,
}

pub type RoleList = PaginatedResponse<RoleListItem>;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RoleSearchParams {
    pub current: Option<u32>,
    pub size: Option<u32>,
    pub roleId: Option<u64>,
    pub roleName: Option<String>,
    pub roleCode: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

// --- System Manage Payloads ---

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserCreateParams {
    pub userName: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>, // '1' enabled, '0' disabled
    pub userGender: Option<String>,
    #[serde(default)]
    pub roleCodes: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserUpdateParams {
    pub id: i64,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub userGender: Option<String>,
    #[serde(default)]
    pub roleCodes: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RoleCreateParams {
    pub roleName: String,
    pub roleCode: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RoleUpdateParams {
    pub roleId: i64,
    pub roleName: String,
    pub roleCode: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

// --- Router Types ---

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RouteMeta {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authList: Option<Vec<AuthItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showBadge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showTextBadge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isHide: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isHideTab: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isIframe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepAlive: Option<bool>,
    // authList omitted for simplicity for now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isFirstLevel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixedTab: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activePath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isFullPage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isAuthButton: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authMark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentPath: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthItem {
    pub title: String,
    pub authMark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppRouteRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub path: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    pub meta: RouteMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<AppRouteRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuPayload {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub parentId: Option<i64>,
    pub path: String,
    pub name: String,
    pub title: String,
    #[serde(default)]
    pub component: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub sort: Option<i32>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub meta: Option<RouteMeta>,
}
