use crate::error::UuidError;

pub mod error;
pub mod random;
pub mod v4;
pub mod v7;

pub type Result<T> = std::result::Result<T, UuidError>;

pub const N_UUID_BYTES: usize = 16;

pub(crate) fn bytes_to_uuid_string(bytes: [u8; N_UUID_BYTES]) -> Result<String> {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut buf = [0u8; 36];
    let mut current: usize = 0;

    for (i, byte) in bytes.iter().copied().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            buf[current] = b'-';
            current += 1;
        }

        buf[current] = HEX[(byte >> 4) as usize];
        buf[current + 1] = HEX[(byte & 0x0f) as usize];
        current += 2
    }

    Ok(String::from_utf8(buf.to_vec())?)
}
