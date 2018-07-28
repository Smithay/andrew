mod bresenham;
use Draw;

use std::cmp::{min, max};

pub struct Line {
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub color: [u8; 4],
}

impl Line {
    pub fn new(pt1: (usize, usize), pt2: (usize, usize), color: [u8; 4]) -> Line {
        Line { pt1, pt2, color }
    }
}

impl Draw for Line {
    fn draw(&self, canvas: &mut [u8], canvas_size: (usize, usize)) {
        // Pt1.x will always be smaller then Pt2.x
        let (pt1, pt2) = match self.pt1.0 > self.pt2.0 {
            true => (self.pt2, self.pt1),
            false => (self.pt1, self.pt2),
        };
        if pt1.0 == pt2.0 {
            // Straight vertical line
            let (min_y, max_y) = match pt2.1 > pt1.1 {
                true => (pt1.1, pt2.1),
                false => (pt2.1, pt1.1)
            };
            for i in min_y..max_y + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * i + pt1.0) + c] = self.color[c];
                }
            }
        } else if pt1.1 == pt2.1 {
            // Straight horizontal line
            for i in pt1.0..pt2.0 + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * pt1.1 + i) + c] = self.color[c];
                }
            }
        } else {
            // Angled line
            for pt in bresenham::BresenhamLine::new(pt1, pt2).pts {
                println!("{}, {}", pt.0, pt.1);
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * pt.1 + pt.0) + c] = self.color[c];
                }
            }
        }
    }
}
