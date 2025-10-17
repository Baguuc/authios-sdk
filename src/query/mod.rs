mod user;
mod permission;

pub struct QueryBuilder {
    base_url: reqwest::Url
}

impl QueryBuilder {
    pub fn new(base_url: reqwest::Url) -> Self {
        Self { base_url }
    } 

    pub fn user(self) -> user::UserQueryBuilder { user::UserQueryBuilder { base_url: self.base_url } }

    pub fn permission(self, api_key: String) -> permission::PermissionQueryBuilder { permission::PermissionQueryBuilder(api_key) }
}
