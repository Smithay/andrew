mod bresenham;
use Draw;

pub struct Line {
    pub pt1: (usize, usize),
    pub pt2: (usize, usize),
    pub color: [u8; 4],
}

impl Line {
    pub fn new(pt1: (usize, usize), pt2: (usize, usize), color: [u8; 4]) -> Line {
        Line {
            pt1,
            pt2,
            color
        }
    }
}

impl Draw for Line {
    fn draw(&self, canvas: &mut[u8], canvas_size: (usize, usize)) {
        if self.pt1.0 == self.pt2.0 {
            // Straight vertical line
            let (pt1, pt2) = if self.pt1.1 > self.pt2.1 {
                (self.pt2, self.pt1)
            } else {
                (self.pt1, self.pt2)
            };
            for i in pt1.1..pt2.1 + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * i + pt1.0) + c] = self.color[c];
                }
            }
        } else if self.pt1.1 == self.pt2.1 {
            // Straight horizontal line 
            let (pt1, pt2) = if self.pt1.0 > self.pt2.0 {
                (self.pt2, self.pt1)
            } else {
                (self.pt1, self.pt2)
            };
            for i in pt1.0..pt2.0 + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * pt1.1 + i) + c] = self.color[c];
                }
            }
        } else {
            // Angled line
            for pt in bresenham::BresenhamLine::new(self.pt1, self.pt2).pts {
                println!("{}, {}", pt.0, pt.1);
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * pt.1 + pt.0) + c] = self.color[c];
                }
            }
        }
    }
}
