#[macro_use]
extern crate bitflags;

pub mod line;
pub mod shape;

/// The Drawable trait allows object to be drawn to a buffer or canvas
pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas);
}

/// The canvas object acts as a wrapper around the buffer, providing information
/// about the buffer for drawing
pub struct Canvas<'a> {
    pub buffer: &'a mut [u8],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_size: usize,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u8], width: usize, height: usize, stride: usize) -> Canvas<'a> {
        assert!(stride % width == 0, "Incorrect Dimensions - Stride is not a multiple of width");
        assert!(buffer.len() == stride * height);
        let pixel_size = stride / width;
        Canvas {
            buffer,
            width,
            height,
            stride,
            pixel_size
        }
    }

    pub fn draw<D: Drawable>(&mut self, drawable: D) {
        drawable.draw(self);
    }
}
