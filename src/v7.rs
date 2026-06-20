use std::time::{SystemTime, UNIX_EPOCH};

use crate::{N_UUID_BYTES, Result, bytes_to_uuid_string, error::UuidError, random::random_bytes};

const N_TS_BYTES: usize = 6;
const N_RANDOM_BYTES: usize = N_UUID_BYTES - N_TS_BYTES;

pub fn uuid_v7() -> Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| UuidError::ClockBeforeUnixEpoch)?
        .as_millis() as u64;

    bytes_to_uuid_string(uuid_v7_from_parts(
        timestamp,
        random_bytes::<N_RANDOM_BYTES>()?,
    ))
}

fn uuid_v7_from_parts(timestamp: u64, random_bytes: [u8; N_RANDOM_BYTES]) -> [u8; N_UUID_BYTES] {
    let mut bytes = [0u8; N_UUID_BYTES];

    let timestamp_bytes = timestamp.to_be_bytes();
    let ts_skip_bytes = timestamp_bytes.len() - N_TS_BYTES;
    bytes[0..6].copy_from_slice(&timestamp_bytes[ts_skip_bytes..]);

    bytes[6..].copy_from_slice(&random_bytes);

    bytes[6] &= 0x0f;
    bytes[6] |= 0x70;

    bytes[8] &= 0x3f;
    bytes[8] |= 0x80;

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_7() {
        let uuid = uuid_v7_from_parts(0x0123_4567_89ab, [0u8; N_RANDOM_BYTES]);

        assert_eq!(uuid[6] >> 4, 0x7);
    }

    #[test]
    fn variant_is_rfc_4122_variant() {
        let uuid = uuid_v7_from_parts(0x0123_4567_89ab, [0u8; N_RANDOM_BYTES]);

        assert_eq!(uuid[8] >> 6, 0b10);
    }

    #[test]
    fn timestamp_is_written_to_first_48_bits_as_big_endian() {
        let uuid = uuid_v7_from_parts(0x0123_4567_89ab, [0u8; N_RANDOM_BYTES]);

        assert_eq!(&uuid[..6], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab]);
    }

    #[test]
    fn random_bytes_are_written_after_timestamp() {
        let random_bytes = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44];

        let uuid = uuid_v7_from_parts(0x0123_4567_89ab, random_bytes);

        // bytes[6]は上位4bitが 0x7 で、固定
        assert_eq!(uuid[6], 0x7a);

        assert_eq!(uuid[7], 0xbb);

        // bytes[8]はvariantで上位2bitが10で固定
        // 0xcc = 1100_1100 なので、
        // 1000_1100 = 0x8c
        assert_eq!(uuid[8], 0x8c);

        assert_eq!(&uuid[9..], &[0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44]);
    }
    #[test]
    fn uuid_v7_orders_by_timestamp() {
        let random = [0u8; N_RANDOM_BYTES];

        let a = uuid_v7_from_parts(1000, random);
        let b = uuid_v7_from_parts(1001, random);

        assert!(a < b);
    }

    #[test]
    fn uuid_v7_string_orders_by_timestamp() {
        let random = [0u8; N_RANDOM_BYTES];

        let a = bytes_to_uuid_string(uuid_v7_from_parts(1000, random)).unwrap();
        let b = bytes_to_uuid_string(uuid_v7_from_parts(1001, random)).unwrap();

        assert!(a < b);
    }

    #[test]
    fn uuid_v7_string_has_expected_format() {
        let random = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44];

        let s = bytes_to_uuid_string(uuid_v7_from_parts(0x0123_4567_89ab, random)).unwrap();

        assert_eq!(s, "01234567-89ab-7abb-8cdd-eeff11223344");
    }
}
