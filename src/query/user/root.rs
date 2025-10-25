pub struct RootUserQueryBuilder {
    pub base_url: reqwest::Url,
    pub root_password: String
}

impl RootUserQueryBuilder {
    /// check root user password with GET /users/root/password route.
    /// 
    pub async fn check_password(self: &Self) -> crate::responses::RootUserCheckPasswordResponse {
        use crate::responses::RootUserCheckPasswordResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/root/password")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .get(url)
            .header("authorization", format!("Bearer {}", self.root_password.clone()))
            .send()
            .await;

        let http_response = match result {
            Ok(http_response) => http_response,
            Err(_) => return Response::ServerUnavailable
        };

        let deserialized = http_response
            .json()
            .await;

        match deserialized {
            Ok(r) => r,
            // when the response is invalid we know that the fetched route is not authios
            Err(_) => Response::ServerNotAuthios
        }
    }
}
