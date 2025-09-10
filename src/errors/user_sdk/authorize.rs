#[derive(thiserror::Error, Debug)]
pub enum UserSdkAuthorizeError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
