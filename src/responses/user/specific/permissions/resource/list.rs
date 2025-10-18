/// represents one of possible responses returned from listing a user's resource permissions (GET /users/{user_id}/permissions/resource/{service_id}/{resource_type})
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum SpecificUserListResourcePermissionsResponse {
    /// ok, permissions returned
    Ok {
        list: ListData
    },
    /// the user with specified id is not found
    UserNotFound,
    /// the provided api key is invalid
    Unauthorized,
    /// server requested is not authios (invalid response returned)
    ServerNotAuthios,
    /// server unavailable when requested
    ServerUnavailable
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListData {
    pub page_number: Option<u32>,
    pub total_page_count: Option<u32>,
    pub permissions: Vec<UserPermissionData>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserPermissionData {
    pub service_id: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i32>,
    pub permissions: Option<Vec<String>>,
}
