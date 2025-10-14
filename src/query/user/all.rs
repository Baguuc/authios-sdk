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

pub struct AllUserQueryBuilder;

impl AllUserQueryBuilder {
    pub fn register(self, login: String, password: String) -> AllUserQuery { AllUserQuery::Register { login, password } }
    
    pub fn login(self, login: String, password: String) -> AllUserQuery { AllUserQuery::Login { login, password } }
}
