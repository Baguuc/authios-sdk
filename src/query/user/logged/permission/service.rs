pub struct LoggedUserServicePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub token: String
}

impl LoggedUserServicePermissionQueryBuilder {
    pub async fn check(self, params: crate::requests::LoggedUserCheckServicePermissionRequest) -> crate::responses::LoggedUserCheckServicePermissionResponse {
        use crate::responses::LoggedUserCheckServicePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/me/permissions/service/{}", params.service_id).as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .get(url)
            .header("authorization", format!("Bearer {}", self.token))
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
