impl crate::UserSdk {
    /// # Sdk::list_permissions
    ///
    pub async fn list_permissions(&self, params: crate::params::UserSdkListPermissionsParams) -> Result<Vec<String>, crate::errors::UserSdkListPermissionsError> {
        use crate::errors::UserSdkListPermissionsError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/me/permissions")
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
            (200, text) => Ok(serde_json::from_str(text).unwrap()),
            (401, "INVALID_TOKEN") => Err(Error::InvalidToken),
            (503, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the reponse")
        };
    }
}
