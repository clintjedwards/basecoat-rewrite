use bcrypt::{hash, DEFAULT_COST};
use nanoid::nanoid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub modified: i64,
}

impl Organization {
    pub fn new(name: &str) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Organization {
            id: nanoid!(10),
            name: name.to_string(),
            created: epoch,
            modified: epoch,
        }
    }
}

#[derive(sqlx::Type, Debug, Clone)]
pub enum UserState {
    Unknown,
    Active,
    Disabled,
}

impl Default for UserState {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub hash: String,
    pub state: UserState,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl User {
    pub fn new(org: &str, name: &str, password: &str) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let hashed = hash(password, DEFAULT_COST).unwrap();

        User {
            id: nanoid!(10),
            name: name.to_string(),
            hash: hashed,
            state: UserState::Active,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}
