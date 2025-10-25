/// represents one of possible responses returned from listing users having access to a particular
/// resource (GET
/// /permissions/resource/{service_id}/{resource_type}/{resource_id}/users)
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ResourcePermissionListUsersResponse {
    /// ok
    Ok {
        page: PageData
    },
    /// permission matching provided criteria is not found
    PermissionNotFound,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}
        
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PageData {
    page_number: Option<u32>,
    users: Option<Vec<UserData>>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData {
    id: Option<i32>,
    login: Option<String>,
    password_hash: Option<String>
}

