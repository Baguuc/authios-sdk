#[derive(serde::Serialize)]
pub struct AuthorizeParams {
    pub token: String,
    pub permission: String
}
