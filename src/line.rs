use line_drawing::Bresenham;
use line_drawing::XiaolinWu;

use std::cmp::{max, min};

use Canvas;
use Drawable;

pub struct Line {
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub color: [u8; 4],
    pub antialiased: bool,
}

impl Line {
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
        // Make it so pt2.x is always bigger then pt1.x
        let (pt1, pt2) = if self.pt1.0 > self.pt2.0 {
            (self.pt2, self.pt1)
        } else {
            (self.pt1, self.pt2)
        };
        if pt1.0 == pt2.0 {
            // Straight vertical line
            let (min_y, max_y) = if pt2.1 > pt1.1 {
                (pt1.1, pt2.1)
            } else {
                (pt2.1, pt1.1)
            };
            if pt1.0 < canvas.width {
                for i in min_y..min(max_y, canvas.height) {
                    canvas.draw_point(pt1.0, i, self.color);
                }
            }
        } else if pt1.1 == pt2.1 {
            // Straight horizontal line
            if max(pt1.1, pt2.1) < canvas.height {
                for i in pt1.0..min(pt2.0, canvas.width) {
                    canvas.draw_point(i, pt1.1, self.color);
                }
            }
        } else if self.antialiased {
            // Angled line with antialias
            for ((x, y), coverage) in XiaolinWu::<f32, isize>::new(
                (pt1.0 as f32, pt1.1 as f32),
                (pt2.0 as f32, pt2.1 as f32),
            ) {
                if x < canvas.width as isize && y < canvas.height as isize {
                    let mut color = self.color;
                    color[3] = (f32::from(color[3]) * coverage) as u8;
                    canvas.draw_point(x as usize, y as usize, color);
                }
            }
        } else {
            // Angled line without antialias
            for (x, y) in Bresenham::new(
                (pt1.0 as isize, pt1.1 as isize),
                (pt2.0 as isize, pt2.1 as isize),
            ) {
                if x < canvas.width as isize && y < canvas.height as isize {
                    canvas.draw_point(x as usize, y as usize, self.color)
                }
            }
        }
    }
}
