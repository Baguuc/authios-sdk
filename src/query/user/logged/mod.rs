mod permission;

pub struct LoggedUserQueryBuilder {
    pub base_url: reqwest::Url,
    pub token: String
}

impl LoggedUserQueryBuilder {
    pub fn permissions(self) -> permission::LoggedUserPermissionQueryBuilder {
        permission::LoggedUserPermissionQueryBuilder { base_url: self.base_url, token: self.token }
    }
    
    pub async fn update(self, params: crate::requests::LoggedUserUpdateRequest) -> crate::responses::LoggedUserUpdateResponse {
        use crate::responses::LoggedUserUpdateResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/me")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .patch(url)
            .header("authorization", format!("Bearer {}", self.token))
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
    
    pub async fn delete(self) -> crate::responses::LoggedUserDeleteResponse {
        use crate::responses::LoggedUserDeleteResponse as Response;

        let url = reqwest::Url::options()
            .base_url(Some(&self.base_url))
            .parse("users/me")
            // won't error
            .unwrap();

        let client = reqwest::Client::new();
        let result = client
            .delete(url)
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
    
    pub async fn get_info(self, params: crate::requests::LoggedUserGetInfoRequest) -> crate::responses::LoggedUserGetInfoResponse {
        use crate::responses::LoggedUserGetInfoResponse as Response;

        let query = format!(
            "?get_id={}&get_login={}&get_password_hash={}",
            params.get_id,
            params.get_login,
            params.get_password_hash
        );
        
        let url = format!("users/me{}", query);
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
}
