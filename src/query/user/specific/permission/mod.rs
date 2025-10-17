mod resource;
mod service;

pub struct SpecificUserPermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserPermissionQueryBuilder {
    pub fn resource(self) -> resource::SpecificUserResourcePermissionQueryBuilder {
        resource::SpecificUserResourcePermissionQueryBuilder {
            base_url: self.base_url,
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
    
    pub fn service(self) -> service::SpecificUserServicePermissionQueryBuilder {
        service::SpecificUserServicePermissionQueryBuilder {
            base_url: self.base_url,
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
}
