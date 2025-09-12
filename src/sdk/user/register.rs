impl crate::UserSdk {
    /// # Sdk::login
    ///
    pub async fn register(&self, params: crate::params::UserSdkRegisterParams) -> Result<String, crate::errors::UserSdkRegisterError> {
        use crate::errors::UserSdkRegisterError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/me")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&params)
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
            (200, token) => Ok(token.to_string()),
            (409, "ALREADY_EXIST") => Err(Error::AlreadyExist),
            (500, "CANNOT_HASH_PASSWORD") => Err(Error::CannotHashPassword),
            (500, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
