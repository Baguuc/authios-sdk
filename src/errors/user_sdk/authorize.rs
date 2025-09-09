#[derive(thiserror::Error, Debug)]
pub enum UserSdkAuthorizeError {
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
    
    #[error("{0}")]
    Url(String),
}

