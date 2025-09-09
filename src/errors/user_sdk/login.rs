#[derive(thiserror::Error, Debug)]
pub enum UserSdkLoginError {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
    
    #[error("{0}")]
    Url(String),

    #[error("Unauthorized")]
    Unauthorized
}
