#[derive(thiserror::Error, Debug)]
pub enum UserSdkCreateError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
