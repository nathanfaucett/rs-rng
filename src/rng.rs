use core::usize;


pub const INV_MAX_F32: f32 = 1f32 / usize::MAX as f32;
pub const INV_MAX_F64: f64 = 1f64 / usize::MAX as f64;


pub trait Rng {
    fn next(&mut self) -> usize;

    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next()
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.next() as u64
    }

    #[inline(always)]
    fn next_f32(&mut self) -> f32 {
        self.next() as f32 * INV_MAX_F32
    }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 * INV_MAX_F64
    }
}
