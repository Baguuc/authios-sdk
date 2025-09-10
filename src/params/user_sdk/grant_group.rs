#[derive(serde::Serialize)]
pub struct GrantGroupParams {
    pub user_login: String,
    pub group_name: String,
    pub token: String
}
