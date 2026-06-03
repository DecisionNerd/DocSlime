//! Command implementations. Each command orchestrates the template inventory
//! ([`crate::templates`]) and the write chokepoint ([`crate::scaffold`]).

pub mod add;
pub mod init;
pub mod list;
