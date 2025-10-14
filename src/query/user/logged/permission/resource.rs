enum LoggedUserResourcePermissionQuery {
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
