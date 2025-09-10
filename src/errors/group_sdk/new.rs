#[derive(thiserror::Error, Debug)]
pub enum GroupSdkNewError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
