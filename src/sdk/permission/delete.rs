impl crate::PermissionSdk {
    pub async fn delete(&self, params: crate::params::PermissionSdkDeleteParams) -> Result<(), crate::errors::PermissionSdkDeleteError> {
        use crate::errors::PermissionSdkDeleteError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("permissions/{}", params.permission_name).as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .delete(url)
            .header("authorization", params.token)
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
            (401, "INVALID_TOKEN") => Err(Error::InvalidToken),
            (404, "NOT_FOUND") => Err(Error::NotFound),
            (503, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
