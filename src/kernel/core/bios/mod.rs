use hashbrown::HashMap;
use spin::{RwLock, Lazy};

use self::acpi::tables::SdtHeader;


pub mod ebda;
pub mod acpi;
pub static ACPI_TABLES: Lazy<RwLock<HashMap<&str, &SdtHeader>>> = 
    Lazy::new(|| 
        RwLock::new(HashMap::new())
    );