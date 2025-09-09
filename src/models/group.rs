#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}
