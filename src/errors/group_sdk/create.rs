#[derive(thiserror::Error, Debug)]
pub enum GroupSdkCreateError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
