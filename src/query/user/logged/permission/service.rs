pub enum LoggedUserServicePermissionQuery {
    Check {
        token: String,
        service_id: String
    }
}

impl LoggedUserServicePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Check { token, service_id } => crate::Query::LoggedUserCheckServicePermission { 
                token,
                service_id
            }
        }
    }
}

pub struct LoggedUserServicePermissionQueryBuilder(pub String);

impl LoggedUserServicePermissionQueryBuilder {
    pub fn check(self, service_id: String) -> LoggedUserServicePermissionQuery {
        LoggedUserServicePermissionQuery::Check {
            token: self.0,
            service_id
        }
    }
}
