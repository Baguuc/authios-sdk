#[derive(thiserror::Error, Debug)]
pub enum PermissionSdkDeleteError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
