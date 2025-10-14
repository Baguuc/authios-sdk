mod all;
mod logged;
mod specific;

pub struct UserQueryBuilder;

impl UserQueryBuilder {
    pub fn all(self) -> all::AllUserQueryBuilder { all::AllUserQueryBuilder }
    pub fn logged(self, token: String) -> logged::LoggedUserQueryBuilder { logged::LoggedUserQueryBuilder(token) }
    pub fn specific(self, api_key: String, user_id: i32) -> specific::SpecificUserQueryBuilder { specific::SpecificUserQueryBuilder { api_key, user_id } }
}
