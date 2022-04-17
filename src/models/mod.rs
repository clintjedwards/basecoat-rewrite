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
            id: nanoid!(),
            name: name.to_string(),
            created: epoch,
            modified: epoch,
        }
    }
}

#[derive(sqlx::Type, Debug, Clone)]
pub enum AccountStatus {
    Unknown,
    Active,
    Disabled,
}

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub hash: String,
    pub state: AccountStatus,
    pub created: i64,
    pub modified: i64,
}
