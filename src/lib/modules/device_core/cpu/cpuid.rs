use core::arch::asm;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::RwLock;
lazy_static! {
    static ref CACHE: RwLock<HashMap<CpuIdRequest, CpuIdRequestReply>> = RwLock::new(HashMap::new());
}

pub struct CpuId {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}
impl CpuId {
    pub unsafe fn new() -> Option<CpuId> {


        None
    }
    pub unsafe fn available() -> bool {
        let mut result: u32 = 0;
        asm!("
            pushfd
            pop {result}
            mov {result}, {saved_flags}
            xor $0x200000, {result}
            push {result}
            popfd
            pushfd
            pop {result}
            xor {result}, {saved_flags}
            ",
            result = out(reg) result,
            saved_flags = out(reg) _,
            options(nomem, nostack, preserves_flags, att_syntax)
        );
        result != 0
    }
    unsafe fn call_cpuid(&mut self, code: u32) {
        asm!("
            cpuid
            mov ebx, {0}
            ",
            out(reg) self.ebx,
            in("eax") code,
            lateout("eax") self.eax,
            out("ecx") self.ecx,
            out("edx") self.edx,
            options(nomem, nostack, preserves_flags, att_syntax)
        );
    }
    fn perform_request(&mut self, request: CpuIdRequest) {
        unsafe {
            self.call_cpuid(request as u32);
        }
    }
    pub fn get_vendor_string(&mut self) -> [u8; 12] {
        if let Some(CpuIdRequestReply::VendorString(result)) = CACHE.read().get(&CpuIdRequest::GetVendorString) {
            return *result;
        }
        self.perform_request(CpuIdRequest::GetVendorString);
        let mut result = [0u8; 12];
        result[0..4].copy_from_slice(&self.ebx.to_le_bytes());
        result[4..8].copy_from_slice(&self.edx.to_le_bytes());
        result[8..12].copy_from_slice(&self.ecx.to_le_bytes());
        CACHE.write().insert(CpuIdRequest::GetVendorString, CpuIdRequestReply::VendorString(result));
        result
    }
    pub fn get_features(&mut self) -> CpuFeatures {
        if let Some(CpuIdRequestReply::Features(result)) = CACHE.read().get(&CpuIdRequest::GetFeatures) {
            return *result;
        }
        self.perform_request(CpuIdRequest::GetFeatures);
        let c = CpuFeatures::new(self.ecx, self.edx);
        CACHE.write().insert(CpuIdRequest::GetFeatures, CpuIdRequestReply::Features(c));
        c
    }


}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CpuIdRequest {
    GetVendorString,
    GetFeatures,
    GetTlb,
    GetSerial,

    IntelExtended = 0x80000000,
    IntelExtendedFeatures,
    IntelExtendedBrandString,
    IntelExtendedBrandStringMore,
    IntelExtendedBrandStringEnd,
}

pub enum CpuIdRequestReply {
    VendorString([u8; 12]),
    Features(CpuFeatures),
    
}
#[derive(Debug, Copy, Clone)]
pub struct CpuFeatures {
    pub sse3: bool,
    pub pclmul: bool,
    pub dtes64: bool,
    pub monitor: bool,
    pub ds_cpl: bool,
    pub vmx: bool,
    pub smx: bool,
    pub est: bool,
    pub tm2: bool,
    pub ssse3: bool,
    pub cnxt_id: bool,
    pub sdbg: bool,
    pub fma: bool,
    pub cx16: bool,
    pub xtpr: bool,
    pub pdcm: bool,
    pub pcid: bool,
    pub dca: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub x2apic: bool,
    pub movbe: bool,
    pub popcnt: bool,
    pub tsc_deadline: bool,
    pub aes: bool,
    pub xsave: bool,
    pub osxsave: bool,
    pub avx: bool,
    pub f16c: bool,
    pub rdrand: bool,
    pub hypervisor: bool,

    pub fpu: bool,
    pub vme: bool,
    pub de: bool,
    pub pse: bool,
    pub tsc: bool,
    pub msr: bool,
    pub pae: bool,
    pub mce: bool,
    pub cx8: bool,
    pub apic: bool,
    pub sep: bool,
    pub mtrr: bool,
    pub pge: bool,
    pub mca: bool,
    pub cmov: bool,
    pub pat: bool,
    pub pse_36: bool,
    pub psn: bool,
    pub clfsh: bool,
    pub ds: bool,
    pub acpi: bool,
    pub mmx: bool,
    pub fxsr: bool,
    pub sse: bool,
    pub sse2: bool,
    pub ss: bool,
    pub htt: bool,
    pub tm: bool,
    pub ia64: bool,
    pub pbe: bool,
}

impl CpuFeatures {
    pub fn new(ecx: u32, edx: u32) -> Self {
        Self {
            sse3: ecx & (1 << 0) != 0,
            pclmul: ecx & (1 << 1) != 0,
            dtes64: ecx & (1 << 2) != 0,
            monitor: ecx & (1 << 3) != 0,
            ds_cpl: ecx & (1 << 4) != 0,
            vmx: ecx & (1 << 5) != 0,
            smx: ecx & (1 << 6) != 0,
            est: ecx & (1 << 7) != 0,
            tm2: ecx & (1 << 8) != 0,
            ssse3: ecx & (1 << 9) != 0,
            cnxt_id: ecx & (1 << 10) != 0,
            sdbg: ecx & (1 << 11) != 0,
            fma: ecx & (1 << 12) != 0,
            cx16: ecx & (1 << 13) != 0,
            xtpr: ecx & (1 << 14) != 0,
            pdcm: ecx & (1 << 15) != 0,
            pcid: ecx & (1 << 17) != 0,
            dca: ecx & (1 << 18) != 0,
            sse4_1: ecx & (1 << 19) != 0,
            sse4_2: ecx & (1 << 20) != 0,
            x2apic: ecx & (1 << 21) != 0,
            movbe: ecx & (1 << 22) != 0,
            popcnt: ecx & (1 << 23) != 0,
            tsc_deadline: ecx & (1 << 24) != 0,
            aes: ecx & (1 << 25) != 0,
            xsave: ecx & (1 << 26) != 0,
            osxsave: ecx & (1 << 27) != 0,
            avx: ecx & (1 << 28) != 0,
            f16c: ecx & (1 << 29) != 0,
            rdrand: ecx & (1 << 30) != 0,
            hypervisor: ecx & (1 << 31) != 0,

            fpu: edx & (1 << 0) != 0,
            vme: edx & (1 << 1) != 0,
            de: edx & (1 << 2) != 0,
            pse: edx & (1 << 3) != 0,
            tsc: edx & (1 << 4) != 0,
            msr: edx & (1 << 5) != 0,
            pae: edx & (1 << 6) != 0,
            mce: edx & (1 << 7) != 0,
            cx8: edx & (1 << 8) != 0,
            apic: edx & (1 << 9) != 0,
            sep: edx & (1 << 11) != 0,
            mtrr: edx & (1 << 12) != 0,
            pge: edx & (1 << 13) != 0,
            mca: edx & (1 << 14) != 0,
            cmov: edx & (1 << 15) != 0,
            pat: edx & (1 << 16) != 0,
            pse_36: edx & (1 << 17) != 0,
            psn: edx & (1 << 18) != 0,
            clfsh: edx & (1 << 19) != 0,
            ds: edx & (1 << 21) != 0,
            acpi: edx & (1 << 22) != 0,
            mmx: edx & (1 << 23) != 0,
            fxsr: edx & (1 << 24) != 0,
            sse: edx & (1 << 25) != 0,
            sse2: edx & (1 << 26) != 0,
            ss: edx & (1 << 27) != 0,
            htt: edx & (1 << 28) != 0,
            tm: edx & (1 << 29) != 0,
            ia64: edx & (1 << 30) != 0,
            pbe: edx & (1 << 31) != 0,
        }
    }
}

struct Tlb {
    pub size: u32,
    pub associativity: u32,
    pub entries: u32,
}