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
            state: 0,
            created: user.created,
            modified: user.modified,
            org_id: user.org_id,
        }
    }
}
