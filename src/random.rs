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

const BUFLEN: usize = 16;

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn getrandom(buf: *mut c_void, buflen: usize, flags: c_uint) -> isize;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn getentropy(buf: *mut c_void, buflen: usize) -> c_int;
}

#[cfg(target_os = "linux")]
pub fn random_bytes() -> io::Result<[u8; BUFLEN]> {
    let mut buf: MaybeUninit<[u8; BUFLEN]> = MaybeUninit::uninit();
    let ret = unsafe { getrandom(buf.as_mut_ptr().cast(), BUFLEN, 0) };

    if ret == BUFLEN as isize {
        unsafe { Ok(buf.assume_init()) }
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(target_os = "macos")]
pub fn random_bytes() -> io::Result<[u8; BUFLEN]> {
    let mut buf: MaybeUninit<[u8; BUFLEN]> = MaybeUninit::uninit();
    let ret = unsafe { getentropy(buf.as_mut_ptr().cast(), BUFLEN) };

    if ret == 0 {
        unsafe { Ok(buf.assume_init()) }
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn random_bytes_has_no_duplicates_in_many_samples() {
        let mut seen = HashSet::new();

        for _ in 0..1000000 {
            let bytes: [u8; BUFLEN] = random_bytes().unwrap();
            assert!(
                seen.insert(bytes),
                "duplicate random value found: {bytes:?}"
            );
        }
    }

    #[test]
    fn random_bytes_bits_are_roughly_balanced() {
        let samples = 1_000_000;
        let total_bits = samples * BUFLEN * 8;

        let mut ones = 0usize;

        for _ in 0..samples {
            let bytes = random_bytes().unwrap();

            for byte in bytes {
                ones += byte.count_ones() as usize;
            }
        }

        let ratio = ones as f64 / total_bits as f64;

        dbg!(ratio);
        assert!((0.499..=0.501).contains(&ratio), "ones ratio was {ratio}");
    }
}
