//pub mod create;

pub struct PermissionSdk {
    base_url: reqwest::Url
}

impl PermissionSdk {
    pub fn new(base_url: reqwest::Url) -> Result<Self, crate::errors::PermissionSdkNewError> {
        use crate::errors::PermissionSdkNewError as Error;
        
        let base_url = match reqwest::Url::parse(base_url.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(Error::InvalidUrl(error.to_string()))
        };
        
        return Ok(Self { base_url });
    }
}
