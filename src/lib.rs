extern crate line_drawing;
extern crate rusttype;
#[macro_use]
extern crate bitflags;

pub mod line;
pub mod shapes;
pub mod text;

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
        assert!(
            stride % width == 0,
            "Incorrect Dimensions - Stride is not a multiple of width"
        );
        assert!(buffer.len() == stride * height);
        let pixel_size = stride / width;
        Canvas {
            buffer,
            width,
            height,
            stride,
            pixel_size,
        }
    }

    pub fn draw<D: Drawable>(&mut self, drawable: &D) {
        drawable.draw(self);
    }

    pub fn draw_point(&mut self, x: usize, y: usize, color: [u8; 4]) {
        for c in 0..3 {
            let alpha = f32::from(color[3]) / 255.0;
            let color_diff = (color[c] as isize
                - self.buffer[self.stride * y + self.pixel_size * x + c] as isize)
                as f32 * alpha;
            let new_color = (f32::from(self.buffer[self.stride * y + self.pixel_size * x + c])
                + color_diff) as u8;
            self.buffer[self.stride * y + self.pixel_size * x + c] = new_color as u8;
        }
        self.buffer[self.stride * y + self.pixel_size * x + 3] = 255 as u8;
    }

    pub fn clear(&mut self) {
        for i in 0..self.width * self.height {
            for c in 0..3 {
                self.buffer[i + c] = 0x00;
            }
            self.buffer[i + 3] = 0xFF;
        }
    }
}
