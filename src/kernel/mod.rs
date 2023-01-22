mod main;

pub(super) mod core;
pub(self) mod internal;
pub(self) mod shell;
pub use main::main;
pub(self) use shell::{PS1, PS1_LEN};