pub struct AllUserQueryBuilder {
    pub base_url: reqwest::Url
}

impl AllUserQueryBuilder {
    /// log in as a user and get the session token using POST /users/me route.
    /// 
    pub async fn login(self: &Self, params: crate::requests::AllUserLoginRequest) -> crate::responses::AllUserLoginResponse {
        use crate::responses::AllUserLoginResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/me")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .json(&params)
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

    /// register a user using POST /users route.
    /// 
    pub async fn register(self: &Self, params: crate::requests::AllUserRegisterRequest) -> crate::responses::AllUserRegisterResponse {
        use crate::responses::AllUserRegisterResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .json(&params)
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
