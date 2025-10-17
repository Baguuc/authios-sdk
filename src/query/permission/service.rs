pub struct ServicePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String
}

impl ServicePermissionQueryBuilder {
    /// create a service permission
    ///
    pub async fn create(self, params: crate::requests::ServicePermissionCreateRequest) -> crate::responses::ServicePermissionCreateResponse {
        use crate::responses::ServicePermissionCreateResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("permissions/service")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .json(&params)
            .header("authorization", format!("Bearer {}", self.api_key))
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
    
    /// delete a service permission
    ///
    pub async fn delete(self, params: crate::requests::ServicePermissionDeleteRequest) -> crate::responses::ServicePermissionDeleteResponse {
        use crate::responses::ServicePermissionDeleteResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("permissions/service/{}", params.service_id).as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .delete(url)
            .header("authorization", format!("Bearer {}", self.api_key))
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
