mod resource;
mod service;

pub struct PermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String
}

impl PermissionQueryBuilder {
    pub fn resource(self) -> resource::ResourcePermissionQueryBuilder {
        resource::ResourcePermissionQueryBuilder {
            base_url: self.base_url,
            api_key: self.api_key
        }
    }
    
    pub fn service(self) -> service::ServicePermissionQueryBuilder {
        service::ServicePermissionQueryBuilder {
            base_url: self.base_url,
            api_key: self.api_key
        }
    }
}
