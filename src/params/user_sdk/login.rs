#[derive(serde::Serialize)]
pub struct LoginParams {
    pub login: String,
    pub pwd: String
}
