#[derive(thiserror::Error, Debug)]
pub enum UserSdkUpdatePwdError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    
    #[error("{0}")]
    UrlParse(String),

    #[error("Unauthorized")]
    Unauthorized
}
