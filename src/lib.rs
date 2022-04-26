mod cephes;

#[cfg(not(feature = "single"))]
#[inline(always)]
pub fn stdtr(k: i32, t: f64) -> f64 {
    unsafe { cephes::stdtr(k, t) }
}

#[cfg(feature = "single")]
#[inline(always)]
pub fn stdtrf(k: i32, t: f32) -> f32 {
    unsafe { cephes::stdtrf(k, t) }
}
