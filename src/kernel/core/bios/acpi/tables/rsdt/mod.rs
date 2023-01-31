use core::str::{from_utf8, from_utf8_unchecked};

use x86_64::{align_up, PhysAddr};

use crate::{err, kernel::core::{mem::util::phys_to_virt_addr}, print, println, serial_log};
use self::{pointer::{Rdsp, Rdsp2}};

use crate::kernel::core::bios::{ebda, ACPI_TABLES};

pub mod pointer;
pub mod table;
mod find;

pub const RSDP_SIGNATURE: &'static [u8; 8] = b"RSD PTR ";

static mut rdsp_ptr: Option<*mut Rdsp> = None;

pub use find::find_rsdt;