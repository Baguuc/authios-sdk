impl crate::GroupSdk {
    pub async fn create(&self, params: crate::params::GroupSdkCreateParams) -> Result<(), crate::errors::GroupSdkCreateError> {
        use serde_json::json; 
        use crate::errors::GroupSdkCreateError as Error;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("groups")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("authorization", params.token)
            .json(&json!({ "name": params.group_name }))
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
            (403, "UNAUTHORIZED") => Err(Error::Unauthorized),
            (409, "ALREADY_EXIST") => Err(Error::AlreadyExist),
            (500, "DATABASE_CONNECTION") => Err(Error::DatabaseConnection),
            
            _ => panic!("Invalid status and body combination, cannot parse the response")
        };
    }
}
