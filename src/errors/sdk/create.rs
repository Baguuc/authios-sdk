#[derive(thiserror::Error, Debug)]
pub enum SdkCreateError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
