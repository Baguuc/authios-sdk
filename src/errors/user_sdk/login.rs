#[derive(thiserror::Error, Debug)]
pub enum UserSdkLoginError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("WRONG_PASSWORD")]
    WrongPassword,
    #[error("CANNOT_GENERATE_TOKEN")]
    CannotGenerateToken,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection

}
