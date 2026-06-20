#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("rikuuid currently supports only Linux and macOS.");

#[cfg(target_os = "macos")]
use std::{
    ffi::{c_int, c_void},
    io,
    mem::MaybeUninit,
};
#[cfg(target_os = "linux")]
use std::{
    ffi::{c_uint, c_void},
    io,
    mem::MaybeUninit,
};

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn getrandom(buf: *mut c_void, buflen: usize, flags: c_uint) -> isize;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn getentropy(buf: *mut c_void, buflen: usize) -> c_int;
}

#[cfg(target_os = "linux")]
pub(crate) fn random_bytes_16() -> io::Result<[u8; 16]> {
    let mut buf: MaybeUninit<[u8; 16]> = MaybeUninit::uninit();
    let ret = unsafe { getrandom(buf.as_mut_ptr().cast(), 16, 0) };

    if ret == 16 as isize {
        unsafe { Ok(buf.assume_init()) }
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn random_bytes_16() -> io::Result<[u8; 16]> {
    let mut buf: MaybeUninit<[u8; 16]> = MaybeUninit::uninit();
    let ret = unsafe { getentropy(buf.as_mut_ptr().cast(), 16) };

    if ret == 0 {
        unsafe { Ok(buf.assume_init()) }
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::N_UUID_BYTES;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn random_bytes_has_no_duplicates_in_many_samples() {
        let mut seen = HashSet::new();

        for _ in 0..1000000 {
            let bytes: [u8; N_UUID_BYTES] = random_bytes_16().unwrap();
            assert!(
                seen.insert(bytes),
                "duplicate random value found: {bytes:?}"
            );
        }
    }

    #[test]
    #[ignore = "heavy test"]
    fn random_bytes_bits_are_roughly_balanced() {
        let samples = 100_000;
        let total_bits = samples * N_UUID_BYTES * 8;

        let mut ones = 0usize;

        for _ in 0..samples {
            let bytes = random_bytes_16().unwrap();

            for byte in bytes {
                ones += byte.count_ones() as usize;
            }
        }

        let ratio = ones as f64 / total_bits as f64;

        dbg!(ratio);
        assert!((0.499..=0.501).contains(&ratio), "ones ratio was {ratio}");
    }
}
