use super::AllocatorSettings;

pub struct DynamicRegionAllocator {
    settings: AllocatorSettings,
}
impl DynamicRegionAllocator {
    pub fn new(settings: AllocatorSettings) -> DynamicRegionAllocator {
        DynamicRegionAllocator {
            settings
        }
    }
    pub async fn test() {
        
    }
}