#[derive(thiserror::Error, Debug)]
pub enum SdkNewError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
