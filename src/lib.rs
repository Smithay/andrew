pub mod line;
pub mod shape;
pub use line::Line;

pub trait Draw {
    fn draw(&self, canvas: &mut[u8], canvas_size: (usize, usize));
}
