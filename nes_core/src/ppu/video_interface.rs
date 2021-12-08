pub trait VideoInterface {
    /// Outputs a single pixel to the interface at the specified location.
    fn draw_pixel(&mut self, x: u16, y: u16, color: Color);
    /// Signals to the interface that a full frame has been sent.
    fn end_of_frame(&mut self);
}

pub struct DummyVideo();
impl VideoInterface for DummyVideo {
    fn draw_pixel(&mut self, _x: u16, _y: u16, _color: Color) {}
    fn end_of_frame(&mut self) {}
}

/// Represents an RGB color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);
impl Color {
    pub fn into_tuple(self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}
