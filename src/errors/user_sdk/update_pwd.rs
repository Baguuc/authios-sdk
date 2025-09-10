#[derive(thiserror::Error, Debug)]
pub enum UserSdkUpdatePwdError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("CANNOT_HASH")]
    CannotHash,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
