#[derive(thiserror::Error, Debug)]
pub enum UserSdkGetInfoError {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
    
    #[error("{0}")]
    Url(String),

    #[error("Unauthorized")]
    Unauthorized
}
