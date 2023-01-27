use hashbrown::HashMap;
use spin::{RwLock, Lazy};

use self::rsdt::table::SdtHeader;

pub mod rsdt;
pub mod ebda;

pub static ACPI_TABLES: Lazy<RwLock<HashMap<&str, &SdtHeader>>> = 
    Lazy::new(|| 
        RwLock::new(HashMap::new())
    );