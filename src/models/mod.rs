use crate::proto::{self, FormulaBaseEntry, FormulaColorantEntry};
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
    pub bases: Vec<Base>,
    pub colorants: Vec<Colorant>,
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
            bases: vec![],
            colorants: vec![],
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}

// NewFormula is an intermediate data structure that collects information about a formula
// and associated colorants/bases to eventually be formed in the database.
#[derive(Default, Debug, Clone)]
pub struct NewFormula {
    pub id: String,
    pub name: String,
    pub number: Option<String>,
    pub notes: Option<String>,
    pub bases: Vec<FormulaBaseEntry>,
    pub colorants: Vec<FormulaColorantEntry>,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl From<proto::CreateFormulaRequest> for NewFormula {
    fn from(formula: proto::CreateFormulaRequest) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        NewFormula {
            id: nanoid!(10),
            name: formula.name,
            number: Some(formula.number),
            notes: Some(formula.notes),
            bases: formula.bases,
            colorants: formula.colorants,
            created: epoch,
            modified: epoch,
            org_id: formula.org_id,
        }
    }
}

impl From<proto::UpdateFormulaRequest> for NewFormula {
    fn from(formula: proto::UpdateFormulaRequest) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        NewFormula {
            id: nanoid!(10),
            name: formula.name,
            number: Some(formula.number),
            notes: Some(formula.notes),
            bases: formula.bases,
            colorants: formula.colorants,
            created: epoch,
            modified: epoch,
            org_id: formula.org_id,
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
    pub amount: Option<String>,
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
            amount: None,
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
    pub amount: Option<String>,
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
            amount: None,
        }
    }
}
