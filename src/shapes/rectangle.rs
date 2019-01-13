use line::Line;
use Canvas;
use Drawable;

bitflags! {
    /// The Sides bitflag presents the sides of a rectangle
    pub struct Sides: u32 {
        /// The top side of the rectangle
        const TOP = 0b0001;
        /// The bottom side of the rectangle
        const BOTTOM = 0b0010;
        /// The left side of the rectangle
        const LEFT = 0b0100;
        /// The right side of the rectangle
        const RIGHT = 0b1000;
        /// All sides of the rectangle
        const ALL = Self::TOP.bits | Self::BOTTOM.bits | Self::LEFT.bits | Self::RIGHT.bits;
    }
}

/// A drawable object that represents a rectangle
pub struct Rectangle {
    /// Position of the top-left corner of rectangle
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
    /// Creates a new Rectangle object
    pub fn new(
        pos: (usize, usize),
        size: (usize, usize),
        border: Option<(usize, [u8; 4], Sides, Option<usize>)>,
        fill: Option<[u8; 4]>,
    ) -> Rectangle {
        Rectangle {
            pos,
            size,
            border,
            fill,
        }
    }

    fn draw_borders(&self, canvas: &mut Canvas) {
        if let Some(border) = self.border {
            for i in 0..border.0 {
                let rounding_space = if let Some(round_size) = border.3 {
                    if i < round_size {
                        round_size
                            - ((round_size as f32).powi(2) - ((round_size - i - 1) as f32).powi(2))
                                .sqrt()
                                .round() as usize
                    } else {
                        0
                    }
                } else {
                    0
                };

                // Top line
                if border.2.contains(Sides::TOP) && canvas.width > rounding_space * 2 {
                    Line::new(
                        (self.pos.0 + rounding_space, self.pos.1 + i),
                        (self.pos.0 + self.size.0 - rounding_space, self.pos.1 + i),
                        border.1,
                        false,
                    )
                    .draw(canvas);
                }
                // Bottom line
                if border.2.contains(Sides::BOTTOM) && canvas.width > rounding_space * 2 {
                    Line::new(
                        (self.pos.0 + rounding_space, self.pos.1 + self.size.1 - i),
                        (
                            self.pos.0 + self.size.0 - rounding_space,
                            self.pos.1 + self.size.1 - i,
                        ),
                        border.1,
                        false,
                    )
                    .draw(canvas);
                }
                // Left line
                if border.2.contains(Sides::LEFT) && canvas.height > rounding_space * 2 {
                    Line::new(
                        (self.pos.0 + i, self.pos.1 + rounding_space),
                        (self.pos.0 + i, self.pos.1 + self.size.1 - rounding_space),
                        border.1,
                        false,
                    )
                    .draw(canvas);
                }
                // Right line
                if border.2.contains(Sides::RIGHT) && canvas.height > rounding_space * 2 {
                    Line::new(
                        (self.pos.0 + self.size.0 - i, self.pos.1 + rounding_space),
                        (
                            self.pos.0 + self.size.0 - i,
                            self.pos.1 + self.size.1 - rounding_space,
                        ),
                        border.1,
                        false,
                    )
                    .draw(canvas);
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
