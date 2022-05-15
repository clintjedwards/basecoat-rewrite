// We use a re-export pattern here to allow all the proto definitions to be under the same module when referencing.
// so instead of `use proto::basecoat::{something}` it's just `use proto::{something}`.
mod footer;
mod login;
pub use self::footer::*;
pub use self::login::*;
