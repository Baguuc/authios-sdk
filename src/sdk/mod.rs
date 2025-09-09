pub mod user;
pub use user::UserSdk;

#[derive(Clone)]
pub struct Sdk {
    base_url: reqwest::Url
}

impl Sdk {
    pub fn new(base_url: String) -> Result<Self, crate::errors::SdkCreateError> { 
        use crate::errors::SdkCreateError as Error;
        
        let base_url = match reqwest::Url::parse(base_url.as_str()) {
            Ok(url) => url,
            Err(error) => return Err(Error::InvalidUrl(error.to_string()))
        };
        
        return Ok(Self { base_url });
    }

    pub fn user(self) -> UserSdk {
        return UserSdk::new(self.base_url).unwrap();
    }
}
