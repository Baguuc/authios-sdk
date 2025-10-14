mod permission;

pub enum SpecificUserQuery {
    Update { 
        api_key: String,
        user_id: i32,
        new_login: Option<String>,
        new_password: Option<String>
    },
    GetInfo {
        api_key: String,
        user_id: i32
    },
    Delete {
        api_key: String,
        user_id: i32
    },
}

pub struct SpecificUserQueryBuilder {
    pub api_key: String,
    pub user_id: i32
}

impl SpecificUserQueryBuilder {
    pub fn permissions(self) -> permission::SpecificUserPermissionQueryBuilder {
        permission::SpecificUserPermissionQueryBuilder {
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
    
    pub fn update(self, new_login: Option<String>, new_password: Option<String>) -> SpecificUserQuery { 
        SpecificUserQuery::Update {
            api_key: self.api_key,
            user_id: self.user_id,
            new_login,
            new_password
        }
    }
    
    pub fn delete(self) -> SpecificUserQuery { 
        SpecificUserQuery::Delete {
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
    
    pub fn get_info(self) -> SpecificUserQuery { 
        SpecificUserQuery::GetInfo {
            api_key: self.api_key,
            user_id: self.user_id
        }
    }
}
