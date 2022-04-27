use crate::models;
use crate::proto;
use std::convert::From;

// We create From implementations so that converting between proto and models is easy.

impl From<models::Organization> for proto::Organization {
    fn from(org: models::Organization) -> Self {
        proto::Organization {
            id: org.id,
            name: org.name,
            created: org.created,
            modified: org.modified,
        }
    }
}

impl From<models::User> for proto::User {
    fn from(user: models::User) -> Self {
        proto::User {
            id: user.id,
            state: user.state as i32,
            created: user.created,
            modified: user.modified,
            org_id: user.org_id,
        }
    }
}

impl From<models::Formula> for proto::Formula {
    fn from(formula: models::Formula) -> Self {
        proto::Formula {
            id: formula.id,
            name: formula.name,
            number: formula.number.unwrap_or_default(),
            notes: formula.notes.unwrap_or_default(),
            bases: formula.bases.into_iter().map(proto::Base::from).collect(),
            colorants: formula
                .colorants
                .into_iter()
                .map(proto::Colorant::from)
                .collect(),
            created: formula.created,
            modified: formula.modified,
            org_id: formula.org_id,
        }
    }
}

impl From<models::Base> for proto::Base {
    fn from(base: models::Base) -> Self {
        proto::Base {
            id: base.id,
            name: base.name,
            manufacturer: base.manufacturer.unwrap_or_default(),
            created: base.created,
            modified: base.modified,
            org_id: base.org_id,
            amount: base.amount.unwrap_or_default(),
        }
    }
}

impl From<models::Colorant> for proto::Colorant {
    fn from(colorant: models::Colorant) -> Self {
        proto::Colorant {
            id: colorant.id,
            name: colorant.name,
            manufacturer: colorant.manufacturer.unwrap_or_default(),
            created: colorant.created,
            modified: colorant.modified,
            org_id: colorant.org_id,
            amount: colorant.amount.unwrap_or_default(),
        }
    }
}

impl From<models::Contractor> for proto::Contractor {
    fn from(contractor: models::Contractor) -> Self {
        proto::Contractor {
            id: contractor.id,
            name: contractor.name,
            contact: contractor.contact.unwrap_or_default(),
            created: contractor.created,
            modified: contractor.modified,
            org_id: contractor.org_id,
        }
    }
}

impl From<models::Job> for proto::Job {
    fn from(job: models::Job) -> Self {
        proto::Job {
            id: job.id,
            name: job.name,
            address: job.address.unwrap_or_default(),
            contact: job.contact.unwrap_or_default(),
            notes: job.notes.unwrap_or_default(),
            created: job.created,
            modified: job.modified,
            contractor_id: job.contractor_id,
            org_id: job.org_id,
        }
    }
}
