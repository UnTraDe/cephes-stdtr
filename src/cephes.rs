use libc::{c_double, c_float, c_int};

#[cfg(not(feature = "single"))]
extern "C" {
    pub fn stdtr(k: c_int, t: c_double) -> c_double;
}

// used internally by Cephes
#[cfg(not(feature = "single"))]
#[no_mangle]
extern "C" fn isfinite(v: c_double) -> c_int {
    v.is_finite() as _
}

#[cfg(feature = "single")]
extern "C" {
    pub fn stdtrf(k: c_int, t: c_float) -> c_float;
}
