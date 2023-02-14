use core::convert::TryFrom;
use bitflags::bitflags;

bitflags! {
    /// Represents the plane masks of the `SequencerIndex::PlaneMask` register.
    pub struct PlaneMask: u8 {
        /// Represents none of the plane masks of vga memory.
        const NONE = 0b0000_0000;
        /// Represents `Plane0` of vga memory.
        const PLANE0 = 0b0000_0001;
        /// Represents `Plane1` of vga memory.
        const PLANE1 = 0b0000_0010;
        /// Represents `Plane2` of vga memory.
        const PLANE2 = 0b0000_0100;
        /// Represents `Plane3` of vga memory.
        const PLANE3 = 0b0000_1000;
        /// Represents all of the plane masks of vga memory.
        const ALL_PLANES = Self::PLANE0.bits() | Self::PLANE1.bits() | Self::PLANE2.bits() | Self::PLANE3.bits();
    }
}

impl TryFrom<u8> for PlaneMask {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlaneMask::PLANE0),
            1 => Ok(PlaneMask::PLANE1),
            2 => Ok(PlaneMask::PLANE2),
            3 => Ok(PlaneMask::PLANE3),
            _ => Err("PlaneMask only accepts values between 0-3!"),
        }
    }
}

impl From<PlaneMask> for u8 {
    fn from(value: PlaneMask) -> u8 {
        value.bits()
    }
}