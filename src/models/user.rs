/// Represents a single user.
/// This struct do not have any method associated to it, it just models the data.
///
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password_hash: String
}
