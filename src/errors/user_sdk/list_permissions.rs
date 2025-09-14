#[derive(thiserror::Error, Debug)]
pub enum UserSdkListPermissionsError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
