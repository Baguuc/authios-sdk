impl crate::UserSdk {
    /// # Sdk::login
    ///
    /// log in as a user and get the session token
    /// 
    pub async fn login(&self, params: crate::params::UserSdkLoginParams) -> Result<String, crate::errors::UserSdkLoginError> {
        use crate::errors::UserSdkLoginError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&params)
            .send()
            .await
            .map_err(|_| Error::ServerUnavaible)?;

        if response.status() == 200 {
            match response.text().await {
                Ok(token) => return Ok(token),
                Err(_) => panic!("Token has to be present in the response")
            };
        }

        if let Ok(text) = response.text().await {
            return match text.as_str() {
                "USER_NOT_FOUND" => Err(Error::UserNotFound),
                "WRONG_PASSWORD" => Err(Error::WrongPassword),
                "CANNOT_GENERATE_TOKEN" => Err(Error::WrongPassword),
                "DATABASE_CONNECTION" => Err(Error::DatabaseConnection),
                // content cannot be different 
                _ => panic!("Wrong error details")
            }
        }
        
        // content has to be present 
        panic!("No error details in the response")
    }
}
