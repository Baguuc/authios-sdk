impl crate::GroupSdk {
    /// # Sdk::grant_permission
    ///
    pub async fn grant_permission(&self, params: crate::params::GroupSdkGrantPermissionParams) -> Result<(), crate::errors::GroupSdkGrantPermissionError> {
        use crate::errors::GroupSdkGrantPermissionError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("groups/{}/permissions/{}", params.group_name, params.permission_name).as_str())
            // won't error 
            .unwrap();
        
        let client = reqwest::Client::new();
        let response = client
            .post(url)
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
            (409, "GROUP_NOT_FOUND") => Err(Error::GroupNotFound),
            (409, "Permission_NOT_FOUND") => Err(Error::PermissionNotFound),
            (409, "ALREADY_ADDED") => Err(Error::AlreadyAdded),
            (401, "UNAUTHORIZED") => Err(Error::Unauthorized),
            (409, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
