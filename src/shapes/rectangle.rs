use line::Line;
use Canvas;
use Drawable;

bitflags! {
    /// The Sides bitflag presents the sides of a rectangle
    pub struct Sides: u32 {
        const TOP = 0b0001;
        const BOTTOM = 0b0010;
        const LEFT = 0b0100;
        const RIGHT = 0b1000;
        const ALL = Self::TOP.bits | Self::BOTTOM.bits | Self::LEFT.bits | Self::RIGHT.bits;
    }
}

enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// The rectangle struct is a drawable object that represents a triangle
pub struct Rectangle {
    /// Position of the top-left corner of rectangle, relative to the buffer
    pub pos: (usize, usize),
    /// The size of the rectangle to be drawn, the border will be contained within this size
    pub size: (usize, usize),
    /// The border that is drawn around the perimeter of the rectangle. It's arguments are
    /// thickness of border, color of border, sides that the border is drawn around, rounding size
    /// of the corners
    pub border: Option<(usize, [u8; 4], Sides, Option<usize>)>,
    /// The color of the fill (area) of the rectangle
    pub fill: Option<[u8; 4]>,
}

impl Rectangle {
    fn draw_borders(&self, canvas: &mut Canvas) {
        if let Some(border) = self.border {
            for i in 0..border.0 {
                // Top line
                if border.2.contains(Sides::TOP) {
                    let (top_pos, top_size) = self.measure_border(Sides::TOP);
                    Line::new(
                        (top_pos.0, top_pos.1 + i),
                        (top_pos.0 + top_size.0, top_pos.1 + i),
                        border.1,
                        false,
                    ).draw(canvas);
                    if border.3.is_some() {
                        if border.2.contains(Sides::LEFT) {
                            self.draw_rounded_corner(canvas, &Corner::TopLeft);
                        }
                        if border.2.contains(Sides::RIGHT) {
                            self.draw_rounded_corner(canvas, &Corner::TopRight);
                        }
                    }
                }
                // Bottom line
                if border.2.contains(Sides::BOTTOM) {
                    let (bottom_pos, bottom_size) = self.measure_border(Sides::BOTTOM);
                    Line::new(
                        (bottom_pos.0, bottom_pos.1 + i),
                        (bottom_pos.0 + bottom_size.0, bottom_pos.1 + i),
                        border.1,
                        false,
                    ).draw(canvas);
                    if border.3.is_some() {
                        if border.2.contains(Sides::LEFT) {
                            self.draw_rounded_corner(canvas, &Corner::BottomLeft);
                        }
                        if border.2.contains(Sides::RIGHT) {
                            self.draw_rounded_corner(canvas, &Corner::BottomRight);
                        }
                    }
                }
                // Left line
                if border.2.contains(Sides::LEFT) {
                    let (left_pos, left_size) = self.measure_border(Sides::LEFT);
                    Line::new(
                        (left_pos.0 + i, left_pos.1),
                        (left_pos.0 + i, left_pos.1 + left_size.1),
                        border.1,
                        false,
                    ).draw(canvas);
                }
                // Right line
                if border.2.contains(Sides::RIGHT) {
                    let (right_pos, right_size) = self.measure_border(Sides::RIGHT);
                    Line::new(
                        (right_pos.0 + i, right_pos.1),
                        (right_pos.0 + i, right_pos.1 + right_size.1),
                        border.1,
                        false,
                    ).draw(canvas);
                }
            }
        }
    }

    fn draw_area(&self, canvas: &mut Canvas) {
        if let Some(fill) = self.fill {
            let (area_pos, area_size) = self.measure_area();
            for y in area_pos.1..area_pos.1 + area_size.1 + 1 {
                Line::new((area_pos.0, y), (area_pos.0 + area_size.0, y), fill, false).draw(canvas)
            }
        }
    }

    fn draw_rounded_corner(&self, canvas: &mut Canvas, corner: &Corner) {
        let round_size = self.border.unwrap().3.unwrap();
        match corner {
            Corner::TopLeft => {
                for y in self.pos.1..self.pos.1 + round_size {
                    let circle_width = round_size
                        - ((round_size as f32).powi(2)
                            - ((round_size - (y - self.pos.1)) as f32).powi(2))
                            .sqrt() as usize;
                    Line::new(
                        (self.pos.0 + circle_width, y),
                        (self.pos.0 + round_size, y),
                        self.border.unwrap().1,
                        false,
                    ).draw(canvas);
                }
            }
            Corner::TopRight => {
                for y in self.pos.1..self.pos.1 + round_size {
                    let circle_width = round_size
                        - ((round_size as f32).powi(2)
                            - ((round_size - (y - self.pos.1)) as f32).powi(2))
                            .sqrt() as usize;
                    Line::new(
                        (self.pos.0 + self.size.0 - self.border.unwrap().0, y),
                        (self.pos.0 + self.size.0 - circle_width, y),
                        self.border.unwrap().1,
                        false,
                    ).draw(canvas);
                }
            }
            Corner::BottomLeft => {
                for y in self.pos.1 + self.size.1 - round_size - 1..self.pos.1 + self.size.1 {
                    let circle_width = round_size
                        - ((round_size as f32).powi(2)
                            - ((y - (self.pos.1 + self.size.1 - round_size - 1)) as f32).powi(2))
                            .sqrt() as usize;
                    Line::new(
                        (self.pos.0 + circle_width, y),
                        (self.pos.0 + round_size, y),
                        self.border.unwrap().1,
                        false,
                    ).draw(canvas);
                }
            }
            Corner::BottomRight => {
                for y in self.pos.1 + self.size.1 - round_size - 1..self.pos.1 + self.size.1 {
                    let circle_width = round_size
                        - ((round_size as f32).powi(2)
                            - ((y - (self.pos.1 + self.size.1 - round_size - 1)) as f32).powi(2))
                            .sqrt() as usize;
                    Line::new(
                        (self.pos.0 + self.size.0 - self.border.unwrap().0, y),
                        (self.pos.0 + self.size.0 - circle_width, y),
                        self.border.unwrap().1,
                        false,
                    ).draw(canvas);
                }
            }
        }
    }

    fn measure_area(&self) -> ((usize, usize), (usize, usize)) {
        let (mut area_pos, mut area_size) = (self.pos, self.size);
        if let Some(border) = self.border {
            if border.2.contains(Sides::TOP) {
                area_pos.1 += border.0;
                area_size.1 -= border.0;
            }
            if border.2.contains(Sides::BOTTOM) {
                area_size.1 -= border.0;
            }
            if border.2.contains(Sides::LEFT) {
                area_pos.0 += border.0;
                area_size.0 -= border.0;
            }
            if border.2.contains(Sides::RIGHT) {
                area_size.0 -= border.0;
            }
        }
        (area_pos, area_size)
    }

    fn measure_border(&self, side: Sides) -> ((usize, usize), (usize, usize)) {
        let (mut border_pos, mut border_size) = ((0, 0), (0, 0));
        if let Some(border) = self.border {
            match side {
                Sides::TOP => {
                    border_pos = self.pos;
                    border_size = (self.size.0, border.0);
                    if let Some(round_size) = border.3 {
                        if border.2.contains(Sides::LEFT) {
                            border_pos.0 += round_size;
                            border_size.0 -= round_size;
                        }
                        if border.2.contains(Sides::RIGHT) {
                            border_size.0 -= round_size;
                        }
                    }
                }
                Sides::BOTTOM => {
                    border_pos = (self.pos.0, self.pos.1 + self.size.1 - border.0);
                    border_size = (self.size.0, border.0);
                    if let Some(round_size) = border.3 {
                        if border.2.contains(Sides::LEFT) {
                            border_pos.0 += round_size;
                            border_size.0 -= round_size;
                        }
                        if border.2.contains(Sides::RIGHT) {
                            border_size.0 -= round_size;
                        }
                    }
                }
                Sides::LEFT => {
                    border_pos = self.pos;
                    border_size = (border.0, self.size.1);
                    if let Some(round_size) = border.3 {
                        if border.2.contains(Sides::TOP) {
                            border_pos.1 += round_size;
                            border_size.1 -= round_size;
                        }
                        if border.2.contains(Sides::BOTTOM) {
                            border_size.1 -= round_size;
                        }
                    }
                }
                Sides::RIGHT => {
                    border_pos = (self.pos.0 + self.size.0 - border.0, self.pos.1);
                    border_size = (border.0, self.size.1);
                    if let Some(round_size) = border.3 {
                        if border.2.contains(Sides::TOP) {
                            border_pos.1 += round_size;
                            border_size.1 -= round_size;
                        }
                        if border.2.contains(Sides::BOTTOM) {
                            border_size.1 -= round_size;
                        }
                    }
                }
                _ => (),
            }
        }
        (border_pos, border_size)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &mut Canvas) {
        self.draw_borders(canvas);
        self.draw_area(canvas);
    }
}
