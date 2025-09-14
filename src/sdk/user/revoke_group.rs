impl crate::UserSdk {
    /// # Sdk::revoke_group
    ///
    pub async fn revoke_group(&self, params: crate::params::UserSdkRevokeGroupParams) -> Result<(), crate::errors::UserSdkRevokeGroupError> {
        use crate::errors::UserSdkRevokeGroupError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("users/{}/groups/{}", params.user_login, params.group_name).as_str())
            // won't error 
            .unwrap();
        
        let client = reqwest::Client::new();
        let response = client
            .delete(url)
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
            (204, _) => Ok(()),
            (401, "UNAUTHORIZED") => Err(Error::Unauthorized),
            (401, "InvalidToken") => Err(Error::InvalidToken),
            (404, "USER_NOT_FOUND") => Err(Error::UserNotFound),
            (404, "GROUP_NOT_FOUND") => Err(Error::GroupNotFound),
            (409, "NOT_ADDED_YET") => Err(Error::NotAddedYet),
            (503, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
