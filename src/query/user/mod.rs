mod all;
mod logged;
mod specific;
mod root;

pub struct UserQueryBuilder {
    pub base_url: reqwest::Url
}

impl UserQueryBuilder {
    pub fn all(self) -> all::AllUserQueryBuilder { 
        all::AllUserQueryBuilder { base_url: self.base_url } 
    }

    pub fn logged(self, token: String) -> logged::LoggedUserQueryBuilder {
        logged::LoggedUserQueryBuilder { base_url: self.base_url, token }
    }

    pub fn specific(self, api_key: String, user_id: i32) -> specific::SpecificUserQueryBuilder {
        specific::SpecificUserQueryBuilder { base_url: self.base_url, api_key, user_id }
    }
    
    pub fn root(self, root_password: String) -> root::RootUserQueryBuilder {
        root::RootUserQueryBuilder { base_url: self.base_url, root_password }
    }
}
