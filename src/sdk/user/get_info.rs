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

        if response.status() == 200 {
            match response.json().await {
                Ok(user) => return Ok(user),
                Err(_) => panic!("User object has to be present in the response")
            };
        }

        if let Ok(text) = response.text().await {
            return match text.as_str() {
                "INVALID_TOKEN" => Err(Error::InvalidToken),
                "DATABASE_CONNECTION" => Err(Error::DatabaseConnection),
                // content cannot be different 
                _ => panic!("Wrong error details")
            }
        }
        
        // content has to be present 
        panic!("No error details in the response")
    }
}
