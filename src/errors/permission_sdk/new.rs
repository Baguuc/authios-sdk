#[derive(thiserror::Error, Debug)]
pub enum PermissionSdkNewError {
    #[error("INVALID_URL:{0}")]
    InvalidUrl(String)
}
