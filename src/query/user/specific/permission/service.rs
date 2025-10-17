pub struct SpecificUserServicePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserServicePermissionQueryBuilder {
    pub async fn check(self, params: crate::requests::SpecificUserCheckServicePermissionRequest) -> crate::responses::SpecificUserCheckServicePermissionResponse {
        use crate::responses::SpecificUserCheckServicePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/service/{}", self.base_url, params.service_id).as_str())
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
    
    pub async fn grant(self, params: crate::requests::SpecificUserGrantServicePermissionRequest) -> crate::responses::SpecificUserCheckServicePermissionResponse {
        use crate::responses::SpecificUserCheckServicePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/service", self.user_id).as_str())
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
    
    pub async fn revoke(self, params: crate::requests::SpecificUserRevokeServicePermissionRequest) -> crate::responses::SpecificUserRevokeServicePermissionResponse {
        use crate::responses::SpecificUserRevokeServicePermissionResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("/users/{}/permissions/service/{}", self.user_id, params.service_id).as_str())
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
