pub mod login;
pub mod get_info;
pub mod authorize;
pub mod update_pwd;
pub mod grant_group;
pub mod revoke_group;

pub struct UserSdk {
    base_url: reqwest::Url
}

impl UserSdk {
    pub fn new(base_url: reqwest::Url) -> Result<Self, crate::errors::UserSdkNewError> {
        use crate::errors::UserSdkNewError as Error;
        
        let base_url = match reqwest::Url::parse(base_url.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(Error::InvalidUrl(error.to_string()))
        };
        
        return Ok(Self { base_url });
    }
}
