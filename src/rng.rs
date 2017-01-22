use core::usize;


pub const MAX_USIZE: usize = usize::MAX;


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
        self.next() as f32 / (MAX_USIZE as f32)
    }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / (MAX_USIZE as f64)
    }
}
