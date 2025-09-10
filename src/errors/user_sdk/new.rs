#[derive(thiserror::Error, Debug)]
pub enum UserSdkNewError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
