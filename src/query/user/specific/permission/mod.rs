mod resource;
mod service;

pub struct SpecificUserPermissionQueryBuilder {
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserPermissionQueryBuilder {
    pub fn resource(self) -> resource::SpecificUserResourcePermissionQueryBuilder {
        resource::SpecificUserResourcePermissionQueryBuilder {
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
    
    pub fn service(self) -> service::SpecificUserServicePermissionQueryBuilder {
        service::SpecificUserServicePermissionQueryBuilder {
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
}
