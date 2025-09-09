impl crate::UserSdk {
    /// # Sdk::authorize
    ///
    /// check if user has specified permission
    /// 
    /// Params:
    /// + token - session token retrieved from login, used to authorize the operation
    /// + permission - permission to check
    ///
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (AuthorizeParams::HTTP)
    /// + when the url of the request cannot be created (AuthorizeParams::Url)
    ///
    pub async fn authorize(&self, params: crate::params::UserSdkAuthorizeParams) -> Result<bool, crate::errors::UserSdkAuthorizeError> {
        use crate::errors::UserSdkAuthorizeError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("user/permissions/{}", params.permission).as_str())
            .map_err(|error| Error::Url(error.to_string()))?;
        
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token)
            .send()
            .await?;
        
        if response.status() == 200 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
