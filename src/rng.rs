

pub trait Rng {
    fn next(&mut self) -> usize;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
}
