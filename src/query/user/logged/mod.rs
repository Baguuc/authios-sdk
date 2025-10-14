mod permission;

pub enum LoggedUserQuery {
    Update { 
        token: String,
        new_login: Option<String>,
        new_password: Option<String>
    },
    GetInfo {
        token: String
    },
    Delete {
        token: String
    }
}

impl LoggedUserQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Update { token, new_login, new_password } => crate::Query::LoggedUserUpdate { 
                token,
                new_login,
                new_password
            },
            Self::GetInfo { token } => crate::Query::LoggedUserGetInfo { token },
            Self::Delete { token } => crate::Query::LoggedUserDelete {  token }
        }
    }
}

pub struct LoggedUserQueryBuilder(pub String);

impl LoggedUserQueryBuilder {
    pub fn permissions(self) -> permission::LoggedUserPermissionQueryBuilder { permission::LoggedUserPermissionQueryBuilder(self.0) }
    
    pub fn update(self, new_login: Option<String>, new_password: Option<String>) -> LoggedUserQuery { 
        LoggedUserQuery::Update {
            token: self.0,
            new_login,
            new_password
        }
    }
    
    pub fn delete(self) -> LoggedUserQuery { 
        LoggedUserQuery::Delete {
            token: self.0
        }
    }
    
    pub fn get_info(self) -> LoggedUserQuery { 
        LoggedUserQuery::GetInfo {
            token: self.0
        }
    }
}
