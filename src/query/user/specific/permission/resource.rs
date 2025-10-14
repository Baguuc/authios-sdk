pub enum SpecificUserResourcePermissionQuery {
    List {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String
    },
    Check {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    Grant {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    Revoke {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    }
}

impl SpecificUserResourcePermissionQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::List { api_key, user_id, service_id, resource_type } => crate::Query::SpecificUserListResourcePermission { 
                api_key,
                user_id,
                service_id,
                resource_type
            },
            Self::Check { api_key, user_id, service_id, resource_type, resource_id, permission_name } => crate::Query::SpecificUserCheckResourcePermission { 
                api_key,
                user_id,
                service_id,
                resource_type,
                resource_id,
                permission_name
            },
            Self::Grant { api_key, user_id, service_id, resource_type, resource_id, permission_name } => crate::Query::SpecificUserGrantResourcePermission { 
                api_key,
                user_id,
                service_id,
                resource_type,
                resource_id,
                permission_name
            },
            Self::Revoke { api_key, user_id, service_id, resource_type, resource_id, permission_name } => crate::Query::SpecificUserRevokeResourcePermission { 
                api_key,
                user_id,
                service_id,
                resource_type,
                resource_id,
                permission_name
            }
        }
    }
}

pub struct SpecificUserResourcePermissionQueryBuilder {
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserResourcePermissionQueryBuilder {
    pub fn list(self, service_id: String, resource_type: String) -> SpecificUserResourcePermissionQuery {
        SpecificUserResourcePermissionQuery::List {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id,
            resource_type
        }
    }
    
    pub fn check(self, service_id: String, resource_type: String, resource_id: i32, permission_name: String) -> SpecificUserResourcePermissionQuery {
        SpecificUserResourcePermissionQuery::Check {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id,
            resource_type,
            resource_id,
            permission_name
        }
    }
    
    pub fn grant(self, service_id: String, resource_type: String, resource_id: i32, permission_name: String) -> SpecificUserResourcePermissionQuery {
        SpecificUserResourcePermissionQuery::Grant {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id,
            resource_type,
            resource_id,
            permission_name
        }
    }
    
    pub fn revoke(self, service_id: String, resource_type: String, resource_id: i32, permission_name: String) -> SpecificUserResourcePermissionQuery {
        SpecificUserResourcePermissionQuery::Revoke {
            api_key: self.api_key,
            user_id: self.user_id,
            service_id,
            resource_type,
            resource_id,
            permission_name
        }
    }
}
