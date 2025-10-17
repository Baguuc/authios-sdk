pub struct SpecificUserResourcePermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserResourcePermissionQueryBuilder {
    pub async fn list(self, params: crate::requests::SpecificUserListResourcePermissionsRequest) -> crate::responses::SpecificUserListResourcePermissionsResponse {
        use crate::responses::SpecificUserListResourcePermissionsResponse as Response;

        let mut url_query = String::new();

        if let Some(page_number) = params.page_number {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}page_number={}", prefix, page_number).as_str());
        }
        
        if let Some(get_page_number) = params.get_page_number {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_page_number={}", prefix, get_page_number).as_str());
        }
        
        if let Some(get_total_page_count) = params.get_total_page_count {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_total_page_count={}", prefix, get_total_page_count).as_str());
        }
        
        if let Some(get_service_id) = params.get_service_id {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_service_id={}", prefix, get_service_id).as_str());
        }
        
        if let Some(get_resource_type) = params.get_resource_type {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_resource_type={}", prefix, get_resource_type).as_str());
        }
        
        if let Some(get_resource_id) = params.get_resource_id {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_resource_id={}", prefix, get_resource_id).as_str());
        }
        
        if let Some(get_permission_names) = params.get_permission_names {
            let prefix = if url_query.len() > 0 { "&" } else { "?" };

            url_query.push_str(format!("{}get_permission_names={}", prefix, get_permission_names).as_str());
        }

        let url = format!("users/{}/permissions/resource/{}/{}{}", self.user_id, params.service_id, params.resource_type, url_query);
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
    
    pub async fn grant(self, params: crate::requests::SpecificUserGrantResourcePermissionRequest) -> crate::responses::SpecificUserCheckResourcePermissionResponse {
        use crate::responses::SpecificUserCheckResourcePermissionResponse as Response;

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
