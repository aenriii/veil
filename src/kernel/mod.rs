mod main;

pub(crate) mod core; // a lot of things will need core
pub(self) mod internal;
#[cfg(not(feature = "async_core"))]
pub(self) mod shell;
#[cfg(feature = "async_core")]
pub(self) mod async_shell;
pub use main::main;