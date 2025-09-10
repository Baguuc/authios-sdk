impl crate::UserSdk {
    /// # Sdk::grant_group
    ///
    pub async fn grant_group(&self, params: crate::params::UserSdkGrantGroupParams) -> Result<(), crate::errors::UserSdkGrantGroupError> {
        use crate::errors::UserSdkGrantGroupError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("users/{}/groups/{}", params.user_login, params.group_name).as_str())
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
            (200, _) => Ok(()),
            (409, "USER_NOT_FOUND") => Err(Error::UserNotFound),
            (409, "GROUP_NOT_FOUND") => Err(Error::GroupNotFound),
            (409, "ALREADY_ADDED") => Err(Error::AlreadyAdded),
            (401, "UNAUTHORIZED") => Err(Error::Unauthorized),
            (409, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
