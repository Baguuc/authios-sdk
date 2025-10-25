pub struct SpecificUserResourcePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserResourcePermissionQueryBuilder {
    pub async fn list(self, params: crate::requests::SpecificUserListResourcePermissionsRequest) -> crate::responses::SpecificUserListResourcePermissionsResponse {
        use crate::responses::SpecificUserListResourcePermissionsResponse as Response;

        let query = format!(
            "?page_number={}&get_page_number={}&get_service_id={}&get_resource_type={}&get_resource_id={}&get_permission_names={}",
            params.page_number,
            params.get_page_number,
            params.get_service_id,
            params.get_resource_type,
            params.get_resource_id,
            params.get_permission_names
        );

        let url = format!("users/{}/permissions/resource/{}/{}{}", self.user_id, params.service_id, params.resource_type, query);
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
    
    pub async fn check(self, params: crate::requests::SpecificUserCheckResourcePermissionRequest) -> crate::responses::SpecificUserCheckResourcePermissionResponse {
        use crate::responses::SpecificUserCheckResourcePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/resource/{}/{}/{}/{}", self.user_id, params.service_id, params.resource_type, params.resource_id, params.permission_name).as_str())
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
    
    pub async fn grant(self, params: crate::requests::SpecificUserGrantResourcePermissionRequest) -> crate::responses::SpecificUserGrantResourcePermissionResponse {
        use crate::responses::SpecificUserGrantResourcePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/resource", self.user_id).as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .post(url)
            .header("authorization", format!("Bearer {}", self.api_key))
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
    
    pub async fn revoke(self, params: crate::requests::SpecificUserRevokeResourcePermissionRequest) -> crate::responses::SpecificUserRevokeResourcePermissionResponse {
        use crate::responses::SpecificUserRevokeResourcePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/resource/{}/{}/{}/{}", self.user_id, params.service_id, params.resource_type, params.resource_id, params.permission_name).as_str())
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
