pub mod create;
pub mod delete;

pub struct GroupSdk {
    base_url: reqwest::Url
}

impl GroupSdk {
    pub fn new(base_url: reqwest::Url) -> Result<Self, crate::errors::GroupSdkNewError> {
        use crate::errors::GroupSdkNewError as Error;
        
        let base_url = match reqwest::Url::parse(base_url.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(Error::InvalidUrl(error.to_string()))
        };
        
        return Ok(Self { base_url });
    }
}
