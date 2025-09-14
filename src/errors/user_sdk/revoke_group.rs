#[derive(thiserror::Error, Debug)]
pub enum UserSdkRevokeGroupError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("GROUP_NOT_FOUND")]
    GroupNotFound,
    #[error("NOT_ADDED_YET")]
    NotAddedYet,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
