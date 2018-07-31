pub trait Color {
    fn draw(&self, buffer: &mut [u8], pos: usize);
}

pub struct Argb8888 {
    a: u8,
    r: u8,
    g: u8,
    b: u8
}

impl Color for Argb8888 {
    fn draw(&self, buffer: &mut [u8], pos: usize) {
        buffer[pos + 0] = self.a;
        buffer[pos + 1] = self.r;
        buffer[pos + 2] = self.g;
        buffer[pos + 3] = self.b;
    }
}
