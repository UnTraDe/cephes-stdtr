mod cephes;

#[inline(always)]
pub fn stdtr(k: i32, t: f64) -> f64 {
    unsafe { cephes::stdtr(k, t) }
}
