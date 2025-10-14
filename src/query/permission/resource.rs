pub enum ResourcePermissionQuery {
    Create {
        api_key: String,
        service_id: String,
        resource_type: String,
        permission_name: String
    },
    Delete {
        api_key: String,
        service_id: String,
        resource_type: String,
        permission_name: String
    }
}

pub struct ResourcePermissionQueryBuilder(pub String);

impl ResourcePermissionQueryBuilder {
    pub fn create(self, service_id: String, resource_type: String, permission_name: String) -> ResourcePermissionQuery {
        ResourcePermissionQuery::Create {
            api_key: self.0,
            service_id,
            resource_type,
            permission_name
        }
    }
    
    pub fn delete(self, service_id: String, resource_type: String, permission_name: String) -> ResourcePermissionQuery {
        ResourcePermissionQuery::Delete {
            api_key: self.0,
            service_id,
            resource_type,
            permission_name
        }
    }
}
