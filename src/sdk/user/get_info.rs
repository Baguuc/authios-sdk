impl crate::UserSdk {
    /// # Sdk::get_info
    ///
    /// get info about a user by token
    ///
    /// Params:
    /// + token - session token retrieved from login, used to authorize the operation
    ///
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (InfoError::HTTP)
    /// + when the url of the request cannot be created (InfoError::Url)
    /// + when provided token is invalid (InfoError::Unauthorized)
    ///
    pub async fn get_info(&self, params: crate::params::UserSdkGetInfoParams) -> Result<crate::models::User, crate::errors::UserSdkGetInfoError> {
        use crate::errors::UserSdkGetInfoError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
            .map_err(|error| Error::Url(error.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(Error::Unauthorized)
        }

        let user: crate::models::User = response
            .json()
            .await?;
        
        return Ok(user);
    }
}
