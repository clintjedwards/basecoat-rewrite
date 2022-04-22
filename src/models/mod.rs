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

#[derive(Default, Debug, Clone)]
pub struct Formula {
    pub id: String,
    pub name: String,
    pub number: Option<String>,
    pub notes: Option<String>,
    pub bases: Option<Vec<Base>>,
    pub colorants: Option<Vec<Colorant>>,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl Formula {
    pub fn new(org: &str, name: &str) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Formula {
            id: nanoid!(10),
            name: name.to_string(),
            number: None,
            notes: None,
            bases: None,
            colorants: None,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Base {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl Base {
    pub fn new(org: &str, name: &str) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Base {
            id: nanoid!(10),
            name: name.to_string(),
            manufacturer: None,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Colorant {
    pub id: String,
    pub name: String,
    pub manufacturer: Option<String>,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl Colorant {
    pub fn new(org: &str, name: &str) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Colorant {
            id: nanoid!(10),
            name: name.to_string(),
            manufacturer: None,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}
