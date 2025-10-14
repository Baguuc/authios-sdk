pub enum AllUserQuery {
    Register {
        login: String,
        password: String
    },
    Login {
        login: String,
        password: String
    },
}

impl AllUserQuery {
    pub fn build_query(self) -> crate::Query {
        match self {
            Self::Register { login, password } => crate::Query::AllUserRegister { 
                login,
                password
            },
            Self::Login { login, password } => crate::Query::AllUserLogin { 
                login,
                password
            }
        }
    }
}

pub struct AllUserQueryBuilder;

impl AllUserQueryBuilder {
    pub fn register(self, login: String, password: String) -> AllUserQuery { AllUserQuery::Register { login, password } }
    
    pub fn login(self, login: String, password: String) -> AllUserQuery { AllUserQuery::Login { login, password } }
}
