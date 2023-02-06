#![allow(non_snake_case)]

use hashbrown::HashMap;
use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref PORT_CONTROLLER: Mutex<PortController> = Mutex::new(PortController::new());
}


pub struct PortController {
    

    Ox3C0: Port<u8>,
    Ox3C1: Port<u8>,

    Ox3C2: Port<u8>, // assure bit 0 is 1 to enable some other ports
    Ox3CC: Port<u8>, 

    Ox3C4: Port<u8>, // W
    Ox3C5: Port<u8>, // R
    Ox3CE: Port<u8>, // W
    Ox3CF: Port<u8>, // R
    Ox3D4: Port<u8>, // W
    Ox3D5: Port<u8>, // R

    Ox3C6: Port<u8>,
    
    Ox3C7: Port<u8>,
    Ox3C8: Port<u8>,
    Ox3C9: Port<u8>,
}

impl PortController {
    pub fn new() -> PortController {
        PortController {
            Ox3C0: Port::new(0x3C0),
            Ox3C1: Port::new(0x3C1),
            Ox3C2: Port::new(0x3C2),
            Ox3CC: Port::new(0x3CC),
            Ox3C4: Port::new(0x3C4),
            Ox3C5: Port::new(0x3C5),
            Ox3CE: Port::new(0x3CE),
            Ox3CF: Port::new(0x3CF),
            Ox3D4: Port::new(0x3D4),
            Ox3D5: Port::new(0x3D5),
            Ox3C6: Port::new(0x3C6),
            Ox3C7: Port::new(0x3C7),
            Ox3C8: Port::new(0x3C8),
            Ox3C9: Port::new(0x3C9),
        }
    }
    pub fn write(&mut self, port: u16, value: u8) { unsafe {
        match port {
            0x3C0 => self.Ox3C0.write(value),
            0x3C1 => self.Ox3C1.write(value),
            0x3C2 => self.Ox3C2.write(value),
            0x3CC => self.Ox3CC.write(value),
            0x3C4 => self.Ox3C4.write(value),
            0x3C5 => self.Ox3C5.write(value),
            0x3CE => self.Ox3CE.write(value),
            0x3CF => self.Ox3CF.write(value),
            0x3D4 => self.Ox3D4.write(value),
            0x3D5 => self.Ox3D5.write(value),
            0x3C6 => self.Ox3C6.write(value),
            0x3C7 => self.Ox3C7.write(value),
            0x3C8 => self.Ox3C8.write(value),
            0x3C9 => self.Ox3C9.write(value),
            _ => panic!("Invalid port: {}", port),
        }
    }}
    pub fn read(&mut self, port: u16) -> u8 { unsafe {
        match port {
            0x3C0 => self.Ox3C0.read(),
            0x3C1 => self.Ox3C1.read(),
            0x3C2 => self.Ox3C2.read(),
            0x3CC => self.Ox3CC.read(),
            0x3C4 => self.Ox3C4.read(),
            0x3C5 => self.Ox3C5.read(),
            0x3CE => self.Ox3CE.read(),
            0x3CF => self.Ox3CF.read(),
            0x3D4 => self.Ox3D4.read(),
            0x3D5 => self.Ox3D5.read(),
            0x3C6 => self.Ox3C6.read(),
            0x3C7 => self.Ox3C7.read(),
            0x3C8 => self.Ox3C8.read(),
            0x3C9 => self.Ox3C9.read(),
            _ => panic!("Invalid port: {}", port),
        }
    }}
}