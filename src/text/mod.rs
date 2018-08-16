use rusttype::{point, Font, Scale, VMetrics};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use Canvas;
use Drawable;

pub struct Text<'a> {
    pub pos: (usize, usize),
    pub color: [u8; 4],
    pub text: String,
    pub font: Font<'a>,
    pub scale: Scale,
    pub v_metrics: VMetrics,
}

impl<'a> Text<'a> {
    pub fn new<P: Into<PathBuf>, T: Into<String>>(
        pos: (usize, usize),
        color: [u8; 4],
        font_path: P,
        height: f32,
        width_scale: f32,
        text: T,
    ) -> Text<'a> {
        let text = text.into();
        // Create font
        let mut font_data: Vec<u8> = Vec::new();
        let mut font_file = File::open(font_path.into()).expect("Could not open font file");
        font_file
            .read_to_end(&mut font_data)
            .expect("Could not read font file");
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
                    let x = x as usize + self.pos.0;
                    let y = y as usize + self.pos.1;

                    let mut color = self.color;
                    color[3] = (f32::from(color[3]) * v) as u8;
                    canvas.draw_point(
                        x + bounding_box.min.x as usize,
                        y + bounding_box.min.y as usize,
                        color,
                    );
                });
            }
        }
    }

    pub fn get_width(&self) -> usize {
        let glyphs: Vec<_> = self
            .font
            .layout(&self.text, self.scale, point(0.0, self.v_metrics.ascent))
            .collect();
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as usize
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw(&self, canvas: &mut Canvas) {
        self.draw_text(canvas);
    }
}
