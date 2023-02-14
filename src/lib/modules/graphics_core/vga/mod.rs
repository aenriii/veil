/*
    https://github.com/rust-osdev/vga was used as a reference for this module.
    All knowledge of VGA is attributed to the authors of that crate.
*/


mod sequencer;
mod grx_control;

mod obj;
pub(self) use obj::vga_ports::*;
pub(self) use obj::plane_mask::*;
pub use obj::frame_buffer::*;
pub use obj::video_mode::*;

mod color;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::{PortReadOnly, PortWriteOnly, Port};

// lazy_static!{
//     pub static ref VGA: Mutex<Vga> = Mutex::new(Vga::new());
// }
#[derive(Debug)]
pub struct Vga {
    // Registers

    // general
    st00_read: PortReadOnly<u8>,
    st01_read_cga: PortReadOnly<u8>,
    st01_read_mda: PortReadOnly<u8>,
    fcr_read: PortReadOnly<u8>,
    fcr_write_cga: PortWriteOnly<u8>,
    fcr_write_mda: PortWriteOnly<u8>,
    msr_read: PortReadOnly<u8>,
    msr_write: PortWriteOnly<u8>,

    // crtc controller
    crx_index_cga: Port<u8>,
    crx_index_mda: Port<u8>,
    crx_data_cga: Port<u8>,
    crx_data_mda: Port<u8>,

    // graphics controller
    grx_index: Port<u8>,
    grx_data: Port<u8>,

    // attribute controller
    arx_index: Port<u8>,
    arx_data: Port<u8>,

    // color palette

    color_data_port: Port<u8>,
    color_index_read_port: Port<u8>,
    color_index_write_port: Port<u8>,

    // sequencer
    srx_index: Port<u8>,
    srx_data: Port<u8>,

    last_video_mode: Option<VideoMode>
}

