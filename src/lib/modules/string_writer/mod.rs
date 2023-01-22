use alloc::{vec::Vec, string::String};


struct StringBuilder {
    buffer: Vec<u8>,
}

impl StringBuilder {
    pub fn new ( ) -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}
impl From<&str> for StringBuilder {
    fn from(some: &str) -> Self {
        Self {
            buffer: some.as_bytes().to_vec(),
        }
    }
}
impl From<String> for StringBuilder {
    fn from(some: String) -> Self {
        Self {
            buffer: some.as_bytes().to_vec(),
        }
    }
}
impl Into<String> for StringBuilder {
    fn into(self) -> String {
        String::from_utf8(self.buffer).unwrap()
    }
}
impl core::fmt::Write for StringBuilder {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.buffer.extend_from_slice(s.as_bytes());
        Ok(())
    }
}