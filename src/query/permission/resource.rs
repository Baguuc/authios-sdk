pub struct ResourcePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String 
}

impl ResourcePermissionQueryBuilder {
    /// create a resource permission
    ///
    pub async fn create(self, params: crate::requests::ResourcePermissionCreateRequest) -> crate::responses::ResourcePermissionCreateResponse {
        use crate::responses::ResourcePermissionCreateResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("permissions/resource")
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
    
    /// delete a resource permission
    ///
    pub async fn delete(self, params: crate::requests::ResourcePermissionDeleteRequest) -> crate::responses::ResourcePermissionDeleteResponse {
        use crate::responses::ResourcePermissionDeleteResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("permissions/resource/{}/{}/{}", params.service_id, params.resource_type, params.permission_name).as_str())
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
