use crate::proto;
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

impl From<proto::CreateFormulaRequest> for Formula {
    fn from(formula: proto::CreateFormulaRequest) -> Self {
        let mut new_formula = Formula::new(&formula.org_id, &formula.name);

        new_formula.number = Some(formula.number);
        new_formula.notes = Some(formula.notes);
        new_formula.bases = formula.bases.into_iter().map(Base::from).collect();
        new_formula.colorants = formula.colorants.into_iter().map(Colorant::from).collect();

        new_formula
    }
}

impl From<proto::UpdateFormulaRequest> for Formula {
    fn from(formula: proto::UpdateFormulaRequest) -> Self {
        let mut updated_formula = Formula::new(&formula.org_id, &formula.name);

        updated_formula.id = formula.id;
        updated_formula.number = Some(formula.number);
        updated_formula.notes = Some(formula.notes);
        updated_formula.bases = formula.bases.into_iter().map(Base::from).collect();
        updated_formula.colorants = formula.colorants.into_iter().map(Colorant::from).collect();

        updated_formula
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
    pub fn new(org: &str, name: &str, manufacturer: Option<String>) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Base {
            id: nanoid!(10),
            name: name.to_string(),
            manufacturer,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
            amount: None,
        }
    }
}

impl From<proto::Base> for Base {
    fn from(base: proto::Base) -> Self {
        Base {
            id: base.id,
            name: base.name,
            manufacturer: Some(base.manufacturer),
            amount: Some(base.amount),
            created: base.created,
            modified: base.modified,
            org_id: base.org_id,
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
    pub fn new(org: &str, name: &str, manufacturer: Option<String>) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Colorant {
            id: nanoid!(10),
            name: name.to_string(),
            manufacturer,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
            amount: None,
        }
    }
}

impl From<proto::Colorant> for Colorant {
    fn from(colorant: proto::Colorant) -> Self {
        Colorant {
            id: colorant.id,
            name: colorant.name,
            manufacturer: Some(colorant.manufacturer),
            amount: Some(colorant.amount),
            created: colorant.created,
            modified: colorant.modified,
            org_id: colorant.org_id,
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Contractor {
    pub id: String,
    pub name: String,
    pub contact: Option<String>,
    pub created: i64,
    pub modified: i64,
    pub org_id: String,
}

impl Contractor {
    pub fn new(org: &str, name: &str, contact: Option<String>) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Contractor {
            id: nanoid!(10),
            name: name.to_string(),
            contact,
            created: epoch,
            modified: epoch,
            org_id: org.to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub contact: Option<String>,
    pub notes: Option<String>,
    pub created: i64,
    pub modified: i64,
    pub contractor_id: String,
    pub org_id: String,
}

impl Job {
    pub fn new(
        org: &str,
        name: &str,
        contractor_id: &str,
        contact: Option<String>,
        address: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Job {
            id: nanoid!(10),
            name: name.to_string(),
            address,
            contact,
            notes,
            created: epoch,
            modified: epoch,
            contractor_id: contractor_id.to_string(),
            org_id: org.to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone)]
pub struct APIToken {
    pub encrypted_token: String,
    pub created: i64,
    pub duration: i64,
    pub org_id: String,
    pub username: String,
}

impl APIToken {
    pub fn new(
        org_id: &str,
        username: &str,
        duration: std::time::Duration,
        encrypted_token: &str,
    ) -> Self {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        APIToken {
            encrypted_token: encrypted_token.to_string(),
            created: epoch,
            duration: duration.as_secs() as i64,
            org_id: org_id.to_string(),
            username: username.to_string(),
        }
    }
}
