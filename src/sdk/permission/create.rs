impl crate::PermissionSdk {
    pub async fn create(&self, params: crate::params::PermissionSdkCreateParams) -> Result<(), crate::errors::PermissionSdkCreateError> {
        use serde_json::json; 
        use crate::errors::PermissionSdkCreateError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("permissions")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("authorization", params.token)
            .json(&json!({ "name": params.permission_name }))
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
            (201, _) => Ok(()),
            (401, "UNAUTHORIZED") => Err(Error::Unauthorized),
            (401, "INVALID_TOKEN") => Err(Error::InvalidToken),
            (409, "ALREADY_EXIST") => Err(Error::AlreadyExist),
            (409, "ROOT_GROUP_NOT_FOUND") => Err(Error::RootGroupNotFound),
            (503, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
