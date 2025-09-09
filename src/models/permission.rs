#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Permission {
    pub name: String,
}
