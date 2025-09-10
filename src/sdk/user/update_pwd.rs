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
        
        let status_code = response
            .status()
            .as_u16();
        
        let text = response
            .text()
            .await
            .unwrap_or(String::new());

        return match (status_code, text.as_str()) {
            (200, _) => Ok(()),
            (401, "INVALID_TOKEN") => Err(Error::InvalidToken),
            (500, "CANNOT_HASH_PWD") => Err(Error::CannotHash),
            (500, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
