mod user;
mod permission;

pub struct QueryBuilder;

impl QueryBuilder {
    pub fn user() -> user::UserQueryBuilder { user::UserQueryBuilder }

    pub fn permission(api_key: String) -> permission::PermissionQueryBuilder { permission::PermissionQueryBuilder(api_key) }
}
