#[derive(serde::Serialize)]
pub struct RevokeGroupParams {
    pub user_login: String,
    pub group_name: String,
    pub token: String
}
