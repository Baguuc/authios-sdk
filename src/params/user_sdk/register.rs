#[derive(serde::Serialize)]
pub struct RegisterParams {
    pub login: String,
    pub pwd: String
}
