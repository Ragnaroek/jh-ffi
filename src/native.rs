
use libc::{c_int};

extern "C" {
    pub fn jh_hash(hashbitlen: c_int, data: *const u8, databitlen: u64, hashval: *mut u8) -> c_int;
}
