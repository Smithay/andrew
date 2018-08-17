use line_drawing::Bresenham;
use line_drawing::XiaolinWu;

use Canvas;
use Drawable;

/// A drawable object that represents a line
pub struct Line {
    /// The first point of the line
    pub pt1: (usize, usize),
    /// The second point of the line
    pub pt2: (usize, usize),
    /// The color of the line
    pub color: [u8; 4],
    /// Decides whether the line will be antialiased
    pub antialiased: bool,
}

impl Line {
    /// Creates a new Line object
    pub fn new(
        pt1: (usize, usize),
        pt2: (usize, usize),
        color: [u8; 4],
        antialiased: bool,
    ) -> Line {
        Line {
            pt1,
            pt2,
            color,
            antialiased,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas) {
        if !self.antialiased || self.pt1.0 == self.pt2.0 || self.pt1.1 == self.pt2.1 {
            // Angled line without antialias
            for (x, y) in Bresenham::new(
                (self.pt1.0 as isize, self.pt1.1 as isize),
                (self.pt2.0 as isize, self.pt2.1 as isize),
            ) {
                if x < canvas.width as isize && y < canvas.height as isize {
                    canvas.draw_point(x as usize, y as usize, self.color)
                }
            }
        } else {
            // Angled line with antialias
            for ((x, y), coverage) in XiaolinWu::<f32, isize>::new(
                (self.pt1.0 as f32, self.pt1.1 as f32),
                (self.pt2.0 as f32, self.pt2.1 as f32),
            ) {
                if x < canvas.width as isize && y < canvas.height as isize {
                    let mut color = self.color;
                    color[3] = (f32::from(color[3]) * coverage) as u8;
                    canvas.draw_point(x as usize, y as usize, color)
                }
            }
        }
    }
}
