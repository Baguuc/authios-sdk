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

impl ResourcePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Create { api_key, service_id, resource_type, permission_name } => crate::Query::ResourcePermissionCreate { 
                api_key,
                service_id,
                resource_type,
                permission_name
            },
            Self::Delete { api_key, service_id, resource_type, permission_name } => crate::Query::ResourcePermissionDelete { 
                api_key,
                service_id,
                resource_type,
                permission_name
            }
        }
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
