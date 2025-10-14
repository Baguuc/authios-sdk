mod resource;
mod service;

pub struct LoggedUserPermissionQueryBuilder(pub String);

impl LoggedUserPermissionQueryBuilder {
    pub fn resource(self) -> resource::LoggedUserResourcePermissionQueryBuilder { resource::LoggedUserResourcePermissionQueryBuilder(self.0) }
    
    pub fn service(self) -> service::LoggedUserServicePermissionQueryBuilder { service::LoggedUserServicePermissionQueryBuilder(self.0) }
}
