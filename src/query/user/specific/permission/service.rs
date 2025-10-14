pub enum SpecificUserServicePermissionQuery {
    Check {
        api_key: String,
        user_id: i32,
        service_id: String
    },
    Grant {
        api_key: String,
        user_id: i32,
        service_id: String
    },
    Revoke {
        api_key: String,
        user_id: i32,
        service_id: String
    }
}

impl SpecificUserServicePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Check { api_key, user_id, service_id } => crate::Query::SpecificUserCheckServicePermission { 
                api_key,
                user_id,
                service_id
            },
            Self::Grant { api_key, user_id, service_id } => crate::Query::SpecificUserGrantServicePermission { 
                api_key,
                user_id,
                service_id
            },
            Self::Revoke { api_key, user_id, service_id } => crate::Query::SpecificUserRevokeServicePermission { 
                api_key,
                user_id,
                service_id
            }
        }
    }
}

pub struct SpecificUserServicePermissionQueryBuilder {
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserServicePermissionQueryBuilder {
    pub fn check(self, service_id: String) -> SpecificUserServicePermissionQuery {
        SpecificUserServicePermissionQuery::Check {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id
        }
    }
    
    pub fn grant(self, service_id: String) -> SpecificUserServicePermissionQuery {
        SpecificUserServicePermissionQuery::Grant {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id
        }
    }
    
    pub fn revoke(self, service_id: String) -> SpecificUserServicePermissionQuery {
        SpecificUserServicePermissionQuery::Revoke {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id
        }
    }
}
