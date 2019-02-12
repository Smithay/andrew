/// A module that contains functions and objects relating to fontconfig
pub mod fontconfig;

use rusttype::{point, Font, Scale, SharedBytes, VMetrics};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use Canvas;
use Drawable;

/// A drawable object that represents text
pub struct Text<'a> {
    /// The position of the text on the canvas
    pub pos: (usize, usize),
    /// The color of the text
    pub color: [u8; 4],
    /// The text that is rendered to the canvas on draw
    pub text: String,
    /// The font used in rendering the text
    pub font: Font<'a>,
    /// The scale that is applied to the text
    pub scale: Scale,
    /// The vertical metrics of the text
    pub v_metrics: VMetrics,
}

/// Loads a font file into a `Vec<u8>`
pub fn load_font_file<P: Into<PathBuf>>(path: P) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let mut file = File::open(path.into()).expect("Could not open font file");
    file.read_to_end(&mut data)
        .expect("Could not read font file");
    data
}

impl<'a> Text<'a> {
    /// Creates a new Text object
    pub fn new<P: Into<SharedBytes<'a>>, T: Into<String>>(
        pos: (usize, usize),
        color: [u8; 4],
        font_data: P,
        height: f32,
        width_scale: f32,
        text: T,
    ) -> Text<'a> {
        let text = text.into();
        // Create font
        let font = Font::from_bytes(font_data).expect("Error constructing Font");
        // Create scale
        let scale = Scale {
            x: height * width_scale,
            y: height,
        };
        // Create needed metrics
        let v_metrics = font.v_metrics(scale);
        Text {
            pos,
            color,
            text: text.clone(),
            scale,
            v_metrics,
            font,
        }
    }

    fn draw_text(&self, canvas: &mut Canvas) {
        let glyphs: Vec<_> = self
            .font
            .layout(&self.text, self.scale, point(0.0, self.v_metrics.ascent))
            .collect();
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = ((x as usize + self.pos.0) as i32 + bounding_box.min.x) as usize;
                    let y = ((y as usize + self.pos.1) as i32 + bounding_box.min.y) as usize;

                    if x < canvas.width && y < canvas.height {
                        let mut color = self.color;
                        color[0] = (f32::from(color[0]) * v) as u8;
                        canvas.draw_point(x, y, color);
                    }
                });
            }
        }
    }

    /// Calculates the width in pixels of the text
    pub fn get_width(&self) -> usize {
        let glyphs: Vec<_> = self
            .font
            .layout(&self.text, self.scale, point(0.0, self.v_metrics.ascent))
            .collect();
        let min_x = glyphs
            .first()
            .map(|g| {
                if let Some(bb) = g.pixel_bounding_box() {
                    bb.min.x
                } else {
                    g.position().x as i32
                }
            })
            .unwrap_or(0);
        let max_x = glyphs
            .last()
            .map(|g| {
                if let Some(bb) = g.pixel_bounding_box() {
                    bb.max.x
                } else {
                    (g.position().x + g.unpositioned().h_metrics().advance_width) as i32
                }
            })
            .unwrap_or(0);
        (max_x - min_x) as usize
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw(&self, canvas: &mut Canvas) {
        self.draw_text(canvas);
    }
}
