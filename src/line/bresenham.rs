use std::cmp::{max, min};

pub struct BresenhamLine {
    pub pts: Vec<(usize, usize)>,
}

impl BresenhamLine {
    pub fn new(pt1: (usize, usize), pt2: (usize, usize)) -> Self {
        Self {
            pts: Self::plot(pt1, pt2)   
        }
    }

    fn plot(pt1: (usize, usize), pt2: (usize, usize)) -> Vec<(usize, usize)> {
        let mut pts: Vec<(usize, usize)> = Vec::new();
        let (delta_x, delta_y) = (pt2.0 - pt1.0,
             max(pt1.1, pt2.1) - min(pt1.1, pt2.1));
        if delta_x >= delta_y {
            // Octant 1 & 2
            let y_inc = delta_y as f32 / delta_x as f32;
            let mut y = pt1.1 as f32;
            for x in pt1.0..pt2.0 {
                match pt2.1 > pt1.1 {
                    true => y += y_inc,
                    false => y -= y_inc
                }
                pts.push((x, y.round() as usize));
            }
        } else {
            // Octant 0 & 4
            let x_inc = delta_x as f32 / delta_y as f32;
            let mut x = pt1.0 as f32;
            if pt2.1 > pt1.1 {
                for y in min(pt1.1, pt2.1)..max(pt1.1, pt2.1) {
                    pts.push((x.round() as usize, y));
                    x += x_inc;
                }
                pts.push((x.round() as usize, pt1.1));
            } else {
                for y in (min(pt1.1, pt2.1)..max(pt1.1, pt2.1)).rev() {
                    pts.push((x.round() as usize, y));
                    x += x_inc;
                }
                pts.push((x.round() as usize, pt1.1));
            }
        }
        pts
    }
}
