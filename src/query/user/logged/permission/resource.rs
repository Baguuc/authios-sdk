pub struct LoggedUserResourcePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub token: String
}

impl LoggedUserResourcePermissionQueryBuilder {
    pub async fn list(self, params: crate::requests::LoggedUserListResourcePermissionsRequest) -> crate::responses::LoggedUserListResourcePermissionsResponse {
        use crate::responses::LoggedUserListResourcePermissionsResponse as Response;

        let query = format!(
            "?page_number={}&get_page_number={}&get_service_id={}&get_resource_type={}&get_resource_id={}&get_permission_names={}",
            params.page_number,
            params.get_page_number,
            params.get_service_id,
            params.get_resource_type,
            params.get_resource_id,
            params.get_permission_names
        );

        let url = format!("users/me/permissions/resource/{}/{}{}", params.service_id, params.resource_type, query);
        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(url.as_str())
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
    
    pub async fn check(self, params: crate::requests::LoggedUserCheckResourcePermissionRequest) -> crate::responses::LoggedUserCheckResourcePermissionResponse {
        use crate::responses::LoggedUserCheckResourcePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/me/permissions/resource/{}/{}/{}/{}", params.service_id, params.resource_type, params.resource_id, params.permission_name).as_str())
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
