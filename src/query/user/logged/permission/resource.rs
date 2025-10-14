pub enum LoggedUserResourcePermissionQuery {
    List {
        token: String,
        service_id: String,
        resource_type: String
    },
    Check {
        token: String,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    }
}

impl LoggedUserResourcePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::List { token, service_id, resource_type } => crate::Query::LoggedUserListResourcePermission { 
                token,
                service_id,
                resource_type
            },
            Self::Check { token, service_id, resource_type, resource_id, permission_name } => crate::Query::LoggedUserCheckResourcePermission { 
                token,
                service_id,
                resource_type,
                resource_id,
                permission_name
            }
        }
    }
}

pub struct LoggedUserResourcePermissionQueryBuilder(pub String);

impl LoggedUserResourcePermissionQueryBuilder {
    pub fn list(self, service_id: String, resource_type: String) -> LoggedUserResourcePermissionQuery {
        LoggedUserResourcePermissionQuery::List {
            token: self.0,
            service_id,
            resource_type
        }
    }
    
    pub fn check(self, service_id: String, resource_type: String, resource_id: i32, permission_name: String) -> LoggedUserResourcePermissionQuery {
        LoggedUserResourcePermissionQuery::Check {
            token: self.0,
            service_id,
            resource_type,
            resource_id,
            permission_name
        }
    }
}
