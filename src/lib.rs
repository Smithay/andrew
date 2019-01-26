//! Andrew is a crate for drawing objects
#![warn(missing_docs)]
extern crate line_drawing;
extern crate rusttype;
extern crate walkdir;
extern crate xdg;
extern crate xml;

#[macro_use]
extern crate bitflags;

/// A module that contains functions and objects relating to lines
pub mod line;
/// A module that contains functions and objects relating to shapes
pub mod shapes;
/// A module that contains functions and objects relating to text
pub mod text;

/// The Drawable trait allows object to be drawn to a buffer or canvas
pub trait Drawable {
    /// A function that draws the object to a canvas
    fn draw(&self, canvas: &mut Canvas);
}

/// Describes an endianness (aka byte order)
#[derive(Debug, PartialEq)]
pub enum Endian {
    /// Little Endian
    Little,
    /// Big Endian
    Big,
}

impl Endian {
    /// Returns the native endianness
    pub fn native() -> Endian {
        if cfg!(target_endian = "little") {
            Endian::Little
        } else {
            Endian::Big
        }
    }
}

/// The canvas object acts as a wrapper around a buffer, providing information and functions
/// for drawing
pub struct Canvas<'a> {
    /// A buffer for the canvas to draw to
    pub buffer: &'a mut [u8],
    /// The width in pixels of the canvas
    pub width: usize,
    /// The height in pixels of the canvas
    pub height: usize,
    /// The number of bytes between each line of pixels on the canvas
    pub stride: usize,
    /// The number of bytes contained in each pixel
    pub pixel_size: usize,
    /// The endianness of the canvas
    pub endianness: Endian,
}

impl<'a> Canvas<'a> {
    /// Creates a new canvas object
    pub fn new(
        buffer: &'a mut [u8],
        width: usize,
        height: usize,
        stride: usize,
        endianness: Endian,
    ) -> Canvas<'a> {
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
            endianness,
        }
    }

    /// Draws an object that implements the Drawable trait to the buffer
    pub fn draw<D: Drawable>(&mut self, drawable: &D) {
        drawable.draw(self);
    }

    /// Draws a pixel at the x and y coordinate
    pub fn draw_point(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let base = self.stride * y + self.pixel_size * x;
        if self.endianness == Endian::Little {
            if color[0] == 255 {
                self.buffer[base + 3] = color[0];
                self.buffer[base + 2] = color[1];
                self.buffer[base + 1] = color[2];
                self.buffer[base] = color[3];
            } else {
                for c in 0..3 {
                    let alpha = f32::from(color[0]) / 255.0;
                    let color_diff =
                        (color[3 - c] as isize - self.buffer[base + c] as isize) as f32 * alpha;
                    let new_color = (f32::from(self.buffer[base + c]) + color_diff) as u8;
                    self.buffer[base + c] = new_color as u8;
                }
                self.buffer[base + 3] = 255 as u8;
            }
        } else if color[0] == 255 {
            self.buffer[base] = color[0];
            self.buffer[base + 1] = color[1];
            self.buffer[base + 2] = color[2];
            self.buffer[base + 3] = color[3];
        } else {
            for c in 1..4 {
                let alpha = f32::from(color[0]) / 255.0;
                let color_diff =
                    (color[c] as isize - self.buffer[base + c] as isize) as f32 * alpha;
                let new_color = (f32::from(self.buffer[base + c]) + color_diff) as u8;
                self.buffer[base + c] = new_color as u8;
            }
            self.buffer[base] = 255 as u8;
        }
    }

    /// Clears the entire canvas buffer by zeroing it
    pub fn clear(&mut self) {
        for i in 0..self.width * self.height * 4 {
            self.buffer[i] = 0x00;
        }
    }
}
