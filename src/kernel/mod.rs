mod main;

pub(crate) mod core; // a lot of things will need core
pub(self) mod internal;
pub(self) mod shell;
pub use main::main;
pub(self) use shell::{PS1, PS1_LEN};