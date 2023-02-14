#[derive(Debug, Clone, Copy)]
pub enum FrameBuffer {
    GraphicsMode = 0xa0000,
    TextMode = 0xb8000,
    MonoMode = 0xb0000,
}

impl From<u8> for FrameBuffer {
    fn from(val: u8) -> Self {
        match val {
            0 => FrameBuffer::GraphicsMode,
            1 => FrameBuffer::TextMode,
            2 => FrameBuffer::MonoMode,
            _ => panic!("Invalid FrameBuffer mode {}", val),
        }
    }   
}
impl From<FrameBuffer> for u32 {
    fn from(val: FrameBuffer) -> Self {
        match val {
            FrameBuffer::GraphicsMode => 0xa0000,
            FrameBuffer::TextMode => 0xb8000,
            FrameBuffer::MonoMode => 0xb0000,
        }
    }
}