use alloc::{vec::Vec, string::String};

struct StringBuilder(String);

impl StringBuilder {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl From<&str> for StringBuilder {
    fn from(some: &str) -> Self {
        Self(some.to_string())
    }
}

impl From<String> for StringBuilder {
    fn from(some: String) -> Self {
        Self(some)
    }
}

impl Into<String> for StringBuilder {
    fn into(self) -> String {
        self.0
    }
}

impl core::fmt::Write for StringBuilder {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
