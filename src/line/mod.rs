mod bresenham;
use Canvas;
use Drawable;

pub struct Line {
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub color: [u8; 4],
}

impl Line {
    pub fn new(pt1: (usize, usize), pt2: (usize, usize), color: [u8; 4]) -> Line {
        let mut color = color;
        color.reverse();
        Line { pt1, pt2, color }
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas) {
        // Pt1.x will always be smaller then Pt2.x
        let (pt1, pt2) = match self.pt1.0 > self.pt2.0 {
            true => (self.pt2, self.pt1),
            false => (self.pt1, self.pt2),
        };
        if pt1.0 == pt2.0 {
            // Straight vertical line
            let (min_y, max_y) = match pt2.1 > pt1.1 {
                true => (pt1.1, pt2.1),
                false => (pt2.1, pt1.1),
            };
            for i in min_y..max_y {
                for c in 0..4 {
                    canvas.buffer[canvas.stride * i + canvas.pixel_size * pt1.0 + c] = self.color[c];
                }
            }
        } else if pt1.1 == pt2.1 {
            // Straight horizontal line
            for i in pt1.0..pt2.0  {
                for c in 0..4 {
                    canvas.buffer[canvas.stride * pt1.1 + canvas.pixel_size * i + c] = self.color[c];
                }
            }
        } else {
            // Angled line
            for pt in bresenham::BresenhamLine::new(pt1, pt2).pts {
                for c in 0..4 {
                    canvas.buffer[canvas.stride * pt.1 + canvas.pixel_size * pt.0 + c] = self.color[c];
                }
            }
        }
    }
}
