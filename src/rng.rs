use core::{u32, u64};


pub const MAX_U32: u32 = u32::MAX;
pub const MAX_U64: u64 = u64::MAX;


pub trait Rng {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn next(&mut self) -> usize {
        self.next_u32() as usize
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn next(&mut self) -> usize {
        self.next_u64() as usize
    }

    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next()
    }

    #[inline(always)]
    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / (MAX_U32 as f32)
    }
    #[inline(always)]
    fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / (MAX_U64 as f64)
    }
}
