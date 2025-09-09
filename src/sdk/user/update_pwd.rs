impl crate::UserSdk {
    /// # Sdk::update_pwd
    ///
    /// update user password
    ///
    /// Params:
    /// + token - session token retrieved from login, used to authorize the operation
    /// + pwd - new password to set
    ///
    /// Errors:
    /// + when the HTTP request cannot be sent to the API (UpdatePwdError::HTTP)
    /// + when the url of the request cannot be created (UpdatePwdError::Url)
    /// + when provided token is invalid (UpdatePwdError::Unauthorized) 
    ///
    pub async fn update_pwd(&self, params: crate::params::UserSdkUpdatePwdParams) -> Result<(), crate::errors::UserSdkUpdatePwdError> {
        use crate::errors::UserSdkUpdatePwdError as Error;

        let result = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user/pwd");

        let url = match result {
            Ok(url) => url,
            Err(error) => return Err(Error::UrlParse(error.to_string()))
        };

        let client = reqwest::Client::new();
        let response = client
            .patch(url)
            .json(&params)
            .header("Authorization", params.token)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(Error::Unauthorized)
        }
        
        return Ok(());
    }
}
