enum SpecificUserServicePermissionQuery {
    List {
        api_key: String,
        user_id: i32,
        service_id: String
    },
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

pub struct SpecificUserServicePermissionQueryBuilder {
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserServicePermissionQueryBuilder {
    pub fn list(self, service_id: String) -> SpecificUserServicePermissionQuery {
        SpecificUserServicePermissionQuery::List {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id
        }
    }
    
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
