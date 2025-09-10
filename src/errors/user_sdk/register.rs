#[derive(thiserror::Error, Debug)]
pub enum UserSdkRegisterError {
    #[error("SERVER_UNAVAIBLE")]
    ServerUnavaible,
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("CANNOT_HASH_PASSWORD")]
    CannotHashPassword,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
