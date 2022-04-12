// We use a re-export pattern here to allow all the proto definitions to be under the same module when referencing.
// so instead of `use proto::basecoat::{something}` it's just `use proto::{something}`.
mod basecoat;
pub use self::basecoat::*;

mod basecoat_message;
pub use self::basecoat_message::*;

mod basecoat_transport;
pub use self::basecoat_transport::*;
