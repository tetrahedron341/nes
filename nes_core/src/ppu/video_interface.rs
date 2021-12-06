pub trait VideoInterface {
    /// Outputs a single pixel to the interface at the specified location.
    fn draw_pixel(&self, x: u16, y: u16, color: Color);
    /// Signals to the interface that a full frame has been sent.
    fn end_of_frame(&self);
}

impl<V: VideoInterface> VideoInterface for &V {
    fn draw_pixel(&self, x: u16, y: u16, color: Color) {
        (**self).draw_pixel(x, y, color)
    }
    fn end_of_frame(&self) {
        (**self).end_of_frame()
    }
}

pub struct DummyVideo();
impl VideoInterface for DummyVideo {
    fn draw_pixel(&self, _x: u16, _y: u16, _color: Color) {}
    fn end_of_frame(&self) {}
}

/// Represents an RGB color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);
impl Color {
    pub fn into_tuple(self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }
}
