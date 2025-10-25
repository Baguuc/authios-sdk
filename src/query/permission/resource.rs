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
    
    /// list users having permission to a resource
    ///
    pub async fn list_users(self, params: crate::requests::ResourcePermissionListUsersRequest) -> crate::responses::ResourcePermissionListUsersResponse {
        use crate::responses::ResourcePermissionListUsersResponse as Response;
        
        let query = format!(
            "?page_number={}&get_page_number={}&get_id={}&get_login={}&get_password_hash={}",
            params.page_number,
            params.get_page_number,
            params.get_id,
            params.get_login,
            params.get_password_hash
        );
        let url = format!("permissions/resource/{}/{}/{}/users{}", params.service_id, params.resource_type, params.resource_id, query);
        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(url.as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .get(url)
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
