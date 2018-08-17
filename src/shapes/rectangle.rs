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
            if let Some(round_size) = border.3 {
                for i in 0..border.0 {
                    let rounding_space = if i < round_size {
                        round_size
                            - ((round_size as f32).powi(2) - ((round_size - i - 1) as f32).powi(2))
                                .sqrt()
                                .round() as usize
                    } else {
                        0
                    };
                    // Top line
                    if border.2.contains(Sides::TOP) {
                        if border.3.is_some() {
                            if border.2.contains(Sides::LEFT) {}
                            if border.2.contains(Sides::RIGHT) {}
                        }
                        Line::new(
                            (self.pos.0 + rounding_space, self.pos.1 + i),
                            (self.pos.0 + self.size.0 - rounding_space, self.pos.1 + i),
                            border.1,
                            false,
                        ).draw(canvas);
                    }
                    // Bottom line
                    if border.2.contains(Sides::BOTTOM) {
                        if border.3.is_some() {
                            if border.2.contains(Sides::LEFT) {}
                            if border.2.contains(Sides::RIGHT) {}
                        }
                        Line::new(
                            (self.pos.0 + rounding_space, self.pos.1 + self.size.1 - i),
                            (
                                self.pos.0 + self.size.0 - rounding_space,
                                self.pos.1 + self.size.1 - i,
                            ),
                            border.1,
                            false,
                        ).draw(canvas);
                    }
                    // Left line
                    if border.2.contains(Sides::LEFT) {
                        Line::new(
                            (self.pos.0 + i, self.pos.1 + rounding_space),
                            (self.pos.0 + i, self.pos.1 + self.size.1 - rounding_space),
                            border.1,
                            false,
                        ).draw(canvas);
                    }
                    // Right line
                    if border.2.contains(Sides::RIGHT) {
                        Line::new(
                            (self.pos.0 + self.size.0 - i, self.pos.1 + rounding_space),
                            (
                                self.pos.0 + self.size.0 - i,
                                self.pos.1 + self.size.1 - rounding_space,
                            ),
                            border.1,
                            false,
                        ).draw(canvas);
                    }
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
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &mut Canvas) {
        self.draw_borders(canvas);
        self.draw_area(canvas);
    }
}
