impl crate::UserSdk {
    /// # Sdk::login
    ///
    /// log in as a user and get the session token
    /// 
    pub async fn login(&self, params: crate::params::UserSdkLoginParams) -> Result<String, crate::errors::UserSdkLoginError> {
        use crate::errors::UserSdkLoginError as Error;

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
            (401, "WRONG_PASSWORD") => Err(Error::WrongPassword),
            (404, "USER_NOT_FOUND") => Err(Error::UserNotFound),
            (503, "CANNOT_GENERATE_TOKEN") => Err(Error::CannotGenerateToken),
            (503, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
