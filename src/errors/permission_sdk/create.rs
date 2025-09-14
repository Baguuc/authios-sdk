#[derive(thiserror::Error, Debug)]
pub enum PermissionSdkCreateError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("ROOT_GROUP_NOT_FOUND")]
    RootGroupNotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
