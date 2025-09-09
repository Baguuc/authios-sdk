impl crate::UserSdk {
    /// # Sdk::login
    ///
    /// log in as a user and get the session token
    /// 
    /// Params:
    /// + login - login of user to log in as
    /// + pwd - password to authenticate the user
    /// 
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (LoginError::HTTP)
    /// + when the url of the request cannot be created (LoginError::Url)
    /// + when the login and password doesn't match (LoginError::Unauthorized)
    /// 
    pub async fn login(&self, params: crate::params::UserSdkLoginParams) -> Result<String, crate::errors::UserSdkLoginError> {
        use crate::errors::UserSdkLoginError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
            .map_err(|error| Error::Url(error.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&params)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(Error::Unauthorized)
        }

        let token = response
            .text()
            .await?;
        
        return Ok(token);
    }
}
