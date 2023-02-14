use super::Color16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextModeColor(u8);

impl TextModeColor {
    /// Returns a new `TextModeColor` given the specified `foreground`
    /// and `background` color.
    pub const fn new(foreground: Color16, background: Color16) -> TextModeColor {
        TextModeColor((background as u8) << 4 | (foreground as u8))
    }

    /// Sets the background color given the specified `background`;
    pub fn set_background(&mut self, background: Color16) {
        self.0 = (background as u8) << 4 | (self.0 & 0x0F);
    }

    /// Sets the foreground color given the specified `foreground`.
    pub fn set_foreground(&mut self, foreground: Color16) {
        self.0 = foreground as u8;
    }
}