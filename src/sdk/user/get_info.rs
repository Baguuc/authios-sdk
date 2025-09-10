impl crate::UserSdk {
    /// # Sdk::get_info
    ///
    /// get info about a user by token
    ///
    pub async fn get_info(&self, params: crate::params::UserSdkGetInfoParams) -> Result<crate::models::User, crate::errors::UserSdkGetInfoError> {
        use crate::errors::UserSdkGetInfoError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
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
            (500, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the reponse")
        };
    }
}
