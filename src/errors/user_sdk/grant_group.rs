#[derive(thiserror::Error, Debug)]
pub enum UserSdkGrantGroupError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
