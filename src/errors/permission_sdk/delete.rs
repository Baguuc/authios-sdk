#[derive(thiserror::Error, Debug)]
pub enum PermissionSdkDeleteError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
