impl crate::UserSdk {
    /// # Sdk::update_pwd
    ///
    /// update user password
    ///
    pub async fn update_pwd(&self, params: crate::params::UserSdkUpdatePwdParams) -> Result<(), crate::errors::UserSdkUpdatePwdError> {
        use crate::errors::UserSdkUpdatePwdError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("user/pwd")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .patch(url)
            .json(&params)
            .header("Authorization", params.token)
            .send()
            .await
            .map_err(|_| Error::ServerUnavaible)?;
        
        if response.status() == 200 {
            return Ok(());
        }

        if let Ok(text) = response.text().await {
            return match text.as_str() {
                "INVALID_TOKEN" => Err(Error::InvalidToken),
                "CANNOT_HASH_PASSWORD" => Err(Error::CannotHash),
                "DATABASE_CONNECTION" => Err(Error::DatabaseConnection),
                // content cannot be different 
                _ => panic!("Wrong error details")
            }
        }
        
        // content has to be present 
        panic!("No error details in the response")
    }
}
