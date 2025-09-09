#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct User {
    pub login: String,
    pub pwd: String,
    pub groups: Vec<String>
}
