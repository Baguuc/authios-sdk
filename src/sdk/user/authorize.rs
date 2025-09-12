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
            .parse(format!("users/me/permissions/{}", params.permission).as_str())
            // won't error 
            .unwrap();
        
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", params.token)
            .send()
            .await
            .map_err(|_| Error::ServerUnavaible)?;
        
        let status_code = response
            .status()
            .as_u16();
        
        let text = response
            .text()
            .await
            .unwrap_or(String::new());

        return match (status_code, text.as_str()) {
            (200, _) => Ok(true),
            (401, "") => Ok(false),
            (401, "INVALID_TOKEN") => Err(Error::InvalidToken),
            (409, "PERMISSION_NOT_FOUND") => Err(Error::PermissionNotFound),
            (500, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
