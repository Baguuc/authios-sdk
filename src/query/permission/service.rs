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

impl ServicePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Create { api_key, service_id } => crate::Query::ServicePermissionCreate { 
                api_key,
                service_id
            },
            Self::Delete { api_key, service_id } => crate::Query::ServicePermissionDelete { 
                api_key,
                service_id
            }
        }
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
