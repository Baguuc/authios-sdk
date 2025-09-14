#[derive(thiserror::Error, Debug)]
pub enum GroupSdkRevokePermissionError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("ALREADY_ADDED")]
    NotAddedYet,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
