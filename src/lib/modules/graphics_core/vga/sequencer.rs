use super::{PlaneMask, Vga};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SequencerIndex {
    SequencerReset = 0x00,
    ClockingMode = 0x01,
    PlaneMask = 0x02,
    CharFont = 0x03,
    MemoryMode = 0x04,
    CounterReset = 0x05,
}

impl From<SequencerIndex> for u8 {
    fn from(value: SequencerIndex) -> u8 {
        value as u8
    }
}
pub trait VgaSequencer {
    fn read(&mut self, index: SequencerIndex) -> u8;
    fn write(&mut self, index: SequencerIndex, value: u8);
    fn set_plane_mask(&mut self, plane_mask: PlaneMask);
    fn set_index(&mut self, index: SequencerIndex);
}

impl VgaSequencer for Vga {
    fn read(&mut self, index: SequencerIndex) -> u8 {
        self.set_index(index);
        unsafe { self.srx_data.read() }
    }

    /// Writes the `value` to the sequencer, as specified by `index`.
    fn write(&mut self, index: SequencerIndex, value: u8) {
        self.set_index(index);
        unsafe {
            self.srx_data.write(value);
        }
    }

    /// Sets the plane mask of the sequencer controller, as specified by `plane_mask`.
    fn set_plane_mask(&mut self, plane_mask: PlaneMask) {
        let original_value = self.read(SequencerIndex::PlaneMask) & 0xF0;
        self.write(
            SequencerIndex::PlaneMask,
            original_value | u8::from(plane_mask),
        );
    }

    fn set_index(&mut self, index: SequencerIndex) {
        unsafe {
            self.srx_index.write(u8::from(index));
        }
    }
}