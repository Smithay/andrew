use std::cmp::{max, min};

use Canvas;
use Drawable;
use Endian;

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
        if !self.antialiased {
            if self.pt1.0 == self.pt2.0 && self.pt1.0 < canvas.width {
                let min_y = min(self.pt1.1, self.pt2.1);
                let max_y = min(max(self.pt1.1, self.pt2.1), canvas.height - 1);
                for y in min_y..=max_y {
                    canvas.draw_point(self.pt1.0, y, self.color)
                }
            } else if self.pt1.1 == self.pt2.1 && self.pt1.1 < canvas.height {
                let min_x = min(self.pt1.0, self.pt2.0);
                let max_x = min(max(self.pt1.0, self.pt2.0), canvas.width - 1);
                for x in min_x..=max_x {
                    canvas.draw_point(x, self.pt1.1, self.color)
                }
            } else {
                // Angled line without antialias
                for (x, y) in bresenham(
                    self.pt1.0 as isize,
                    self.pt1.1 as isize,
                    self.pt2.0 as isize,
                    self.pt2.1 as isize,
                ) {
                    if x < canvas.width && y < canvas.height {
                        canvas.draw_point(x, y, self.color)
                    }
                }
            }
        } else {
            // Angled line with antialias
            for (x, y, coverage) in xiaolin_wu(
                self.pt1.0 as f32,
                self.pt1.1 as f32,
                self.pt2.0 as f32,
                self.pt2.1 as f32,
            ) {
                if x < canvas.width && y < canvas.height {
                    let mut color = self.color;
                    let base = canvas.stride * y + canvas.pixel_size * x;
                    if coverage != 1.0 {
                        if canvas.endianness == Endian::Little {
                            color[1] = (canvas.buffer[base + 2] as f32 * (1.0 - coverage)
                                + color[1] as f32 * coverage)
                                as u8;
                            color[2] = (canvas.buffer[base + 1] as f32 * (1.0 - coverage)
                                + color[2] as f32 * coverage)
                                as u8;
                            color[3] = (canvas.buffer[base] as f32 * (1.0 - coverage)
                                + color[3] as f32 * coverage)
                                as u8;
                        } else {
                            color[1] = (canvas.buffer[base + 1] as f32 * (1.0 - coverage)
                                + color[1] as f32 * coverage)
                                as u8;
                            color[2] = (canvas.buffer[base + 2] as f32 * (1.0 - coverage)
                                + color[2] as f32 * coverage)
                                as u8;
                            color[3] = (canvas.buffer[base + 3] as f32 * (1.0 - coverage)
                                + color[3] as f32 * coverage)
                                as u8;
                        }
                    }
                    canvas.draw_point(x as usize, y as usize, color)
                }
            }
        }
    }
}

fn bresenham(mut x0: isize, mut y0: isize, x1: isize, y1: isize) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -((y1 - y0).abs());
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push((x0 as usize, y0 as usize));
        if x0 == x1 && y0 == y1 {
            break;
        };
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    points
}

fn xiaolin_wu(mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32) -> Vec<(usize, usize, f32)> {
    let mut points: Vec<(usize, usize, f32)> = Vec::new();
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx == 0.0 {
        1.0
    } else {
        dy as f32 / dx as f32
    };

    let mut intery = y0 + gradient;
    points.push((x0 as usize, y0 as usize, 1.0));
    points.push((x1 as usize, y1 as usize, 1.0));
    if steep {
        for x in x0 as usize + 1..=x1 as usize - 1 {
            points.push((intery as usize, x, 1.0 - intery.fract()));
            points.push((intery as usize + 1, x, intery.fract()));
            intery = intery + gradient;
        }
    } else {
        for x in x0 as usize + 1..=x1 as usize - 1 {
            points.push((x, intery as usize, 1.0 - intery.fract()));
            points.push((x, intery as usize + 1, intery.fract()));
            intery = intery + gradient;
        }
    }
    points
}
