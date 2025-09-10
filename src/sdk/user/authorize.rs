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
            // won't error 
            .unwrap();
        
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token)
            .send()
            .await
            .map_err(|_| Error::ServerUnavaible)?;
        
        if response.status() == 200 {
            return Ok(true);
        }

        if let Ok(text) = response.text().await {
            return match text.as_str() {
                "UNAUTHORIZED" => Ok(false),
                "INVALID_TOKEN" => Err(Error::InvalidToken),
                "PERMISSION_NOT_FOUND" => Err(Error::PermissionNotFound),
                "DATABASE_CONNECTION" => Err(Error::DatabaseConnection),
                // content cannot be different 
                _ => panic!("Wrong error details")
            }
        }
        
        // content has to be present 
        panic!("No error details in the response")
    }
}
