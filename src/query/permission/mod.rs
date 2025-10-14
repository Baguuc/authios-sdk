mod resource;
mod service;

pub struct PermissionQueryBuilder(pub String);

impl PermissionQueryBuilder {
    pub fn resource(self) -> resource::ResourcePermissionQueryBuilder {
        resource::ResourcePermissionQueryBuilder(self.0)
    }
    
    pub fn service(self) -> resource::ResourcePermissionQueryBuilder {
        resource::ResourcePermissionQueryBuilder(self.0)
    }
}
