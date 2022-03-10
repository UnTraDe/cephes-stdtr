use libc::{c_double, c_int};

extern "C" {
    pub fn stdtr(k: c_int, t: c_double) -> c_double;
}

// used internally by Cephes
#[no_mangle]
extern "C" fn isfinite(v: c_double) -> c_int {
    v.is_finite() as _
}
