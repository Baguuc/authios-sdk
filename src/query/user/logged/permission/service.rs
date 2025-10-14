enum LoggedUserServicePermissionQuery {
    List {
        token: String,
        service_id: String
    },
    Check {
        token: String,
        service_id: String
    }
}

pub struct LoggedUserServicePermissionQueryBuilder(pub String);

impl LoggedUserServicePermissionQueryBuilder {
    pub fn list(self, service_id: String) -> LoggedUserServicePermissionQuery {
        LoggedUserServicePermissionQuery::List {
            token: self.0,
            service_id
        }
    }
    
    pub fn check(self, service_id: String) -> LoggedUserServicePermissionQuery {
        LoggedUserServicePermissionQuery::Check {
            token: self.0,
            service_id
        }
    }
}
