mod permission;

pub struct SpecificUserQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserQueryBuilder {
    pub fn permissions(self) -> permission::SpecificUserPermissionQueryBuilder {
        permission::SpecificUserPermissionQueryBuilder {
            base_url: self.base_url,
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
    
    pub async fn update(self, params: crate::requests::SpecificUserUpdateRequest) -> crate::responses::SpecificUserUpdateResponse {
        use crate::responses::SpecificUserUpdateResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("users/{}", self.user_id).as_str())
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .patch(url)
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
    
    pub async fn delete(self) -> crate::responses::SpecificUserDeleteResponse { 
        use crate::responses::SpecificUserDeleteResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse(format!("users/{}", self.user_id).as_str())
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
