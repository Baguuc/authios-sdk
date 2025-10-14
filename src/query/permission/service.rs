pub enum ServicePermissionQuery {
    Create {
        api_key: String,
        service_id: String
    },
    Delete {
        api_key: String,
        service_id: String
    }
}

pub struct ServicePermissionQueryBuilder(pub String);

impl ServicePermissionQueryBuilder {
    pub fn create(self, service_id: String) -> ServicePermissionQuery {
        ServicePermissionQuery::Create {
            api_key: self.0,
            service_id
        }
    }
    
    pub fn delete(self, service_id: String) -> ServicePermissionQuery {
        ServicePermissionQuery::Delete {
            api_key: self.0,
            service_id
        }
    }
}
