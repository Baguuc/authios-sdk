mod user;
mod permission;

pub enum Query {
    AllUserRegister {
        login: String, password: String,
    },
    AllUserLogin {
        login: String,
        password: String,
    },
    LoggedUserUpdate { 
        token: String,
        new_login: Option<String>,
        new_password: Option<String>
    },
    LoggedUserGetInfo {
        token: String
    },
    LoggedUserDelete {
        token: String
    },
    LoggedUserListResourcePermission {
        token: String,
        service_id: String,
        resource_type: String
    },
    LoggedUserCheckResourcePermission {
        token: String,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    LoggedUserCheckServicePermission {
        token: String,
        service_id: String
    },
    SpecificUserUpdate { 
        api_key: String,
        user_id: i32,
        new_login: Option<String>,
        new_password: Option<String>
    },
    SpecificUserGetInfo {
        api_key: String,
        user_id: i32
    },
    SpecificUserDelete {
        api_key: String,
        user_id: i32
    },
    SpecificUserListResourcePermission {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String
    },
    SpecificUserCheckResourcePermission {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    SpecificUserCheckServicePermission {
        api_key: String,
        user_id: i32,
        service_id: String
    },
    SpecificUserGrantResourcePermission {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    SpecificUserRevokeResourcePermission {
        api_key: String,
        user_id: i32,
        service_id: String,
        resource_type: String,
        resource_id: i32,
        permission_name: String
    },
    SpecificUserGrantServicePermission {
        api_key: String,
        user_id: i32,
        service_id: String
    },
    SpecificUserRevokeServicePermission {
        api_key: String,
        user_id: i32,
        service_id: String
    },
    ResourcePermissionCreate {
        api_key: String,
        service_id: String,
        resource_type: String,
        permission_name: String
    },
    ResourcePermissionDelete {
        api_key: String,
        service_id: String,
        resource_type: String,
        permission_name: String
    },
    ServicePermissionCreate {
        api_key: String,
        service_id: String
    },
    ServicePermissionDelete {
        api_key: String,
        service_id: String
    }
}

impl Query {
    pub fn new() -> QueryBuilder { QueryBuilder }
}

pub struct QueryBuilder;

impl QueryBuilder {
    pub fn user(self) -> user::UserQueryBuilder { user::UserQueryBuilder }

    pub fn permission(self, api_key: String) -> permission::PermissionQueryBuilder { permission::PermissionQueryBuilder(api_key) }
}
