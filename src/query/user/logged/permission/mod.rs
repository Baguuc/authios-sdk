mod resource;
mod service;

pub struct LoggedUserPermissionQueryBuilder {
    pub base_url: reqwest::Url,
    pub token: String
}

impl LoggedUserPermissionQueryBuilder {
    pub fn resource(self) -> resource::LoggedUserResourcePermissionQueryBuilder { resource::LoggedUserResourcePermissionQueryBuilder { base_url: self.base_url, token: self.token } }
    
    pub fn service(self) -> service::LoggedUserServicePermissionQueryBuilder { service::LoggedUserServicePermissionQueryBuilder { base_url: self.base_url, token: self.token } }
}
