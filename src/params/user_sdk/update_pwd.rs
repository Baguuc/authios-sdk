#[derive(serde::Serialize)]
pub struct UpdatePwdParams {
    pub token: String,
    pub pwd: String
}
