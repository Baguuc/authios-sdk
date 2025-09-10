#[derive(thiserror::Error, Debug)]
pub enum GroupSdkGrantPermissionError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("PERMISSION_NOT_FOUND")]
    PermissionNotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
