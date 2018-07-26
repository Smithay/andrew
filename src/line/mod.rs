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
            for i in 0..(self.pt2.1 - self.pt1.1) + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * i + self.pt1.0) + c] = self.color[c];
                }
            }
        } else if self.pt1.1 == self.pt2.1 {
            // Straight horizontal line 
            for i in 0..(self.pt2.0 - self.pt1.0) + 1 {
                for c in 0..4 {
                    canvas[4 * (canvas_size.0 * self.pt1.1 + self.pt1.0 + i) + c] = self.color[c];
                }
            }
        } else {
            // Angled line
            let gradient = (self.pt2.1 - self.pt1.1) as f32 / (self.pt2.0 - self.pt1.0) as f32;  
            println!("gradient = {}", gradient);
        }
    }
}
