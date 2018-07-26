use line::Line;
use Draw;

pub struct Rectangle {
    pub pos: (usize, usize),
    pub size: (usize, usize),
    pub border: Option<(usize, [u8; 4])>,
    pub fill: Option<[u8; 4]>,
}

impl Rectangle {
    fn draw_borders(&self, canvas: &mut[u8], canvas_size: (usize, usize)) {
        if let Some(border) = self.border {
            for i in 0..border.0 {
                // Top line
                Line::new((self.pos.0, self.pos.1 + i), 
                          (self.pos.0 + self.size.0, self.pos.1 + i), border.1)
                    .draw(canvas, canvas_size);
                // Bottom line
                Line::new((self.pos.0, self.pos.1 + self.size.1 - i), 
                          (self.pos.0 + self.size.0, self.pos.1 + self.size.1 - i), border.1)
                    .draw(canvas, canvas_size);
                // Left line
                Line::new((self.pos.0 + i, self.pos.1),
                          (self.pos.0 + i, self.pos.1 + self.size.1), border.1)
                    .draw(canvas, canvas_size);
                // Right line
                Line::new((self.pos.0 + self.size.0 - i, self.pos.1),
                          (self.pos.0 + self.size.0 - i, self.pos.1 + self.size.1), border.1)
                    .draw(canvas, canvas_size);
            }
        }
    }
    
    fn draw_area(&self, canvas: &mut[u8], canvas_size: (usize, usize)) {
        if let Some(fill) = self.fill {
            let (area_pos, area_size) = match self.border {
                Some(border) => {
                    ((self.pos.0 + border.0, self.pos.1 + border.0),
                    (self.size.0 - 2 * border.0, self.size.1 - 2 * border.0))
                },
                None => ((self.pos.0, self.pos.1), (self.size.0, self.size.1))
            };
            for y in area_pos.1..area_pos.1 + area_size.1 + 1 {
                Line::new((area_pos.0, y),
                    (area_pos.0 + area_size.0, y), fill).draw(canvas, canvas_size)
            }
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self, canvas: &mut[u8], canvas_size: (usize, usize)) {
        self.draw_borders(canvas, canvas_size);
        self.draw_area(canvas, canvas_size);
    }
}
