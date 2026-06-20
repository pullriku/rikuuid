use crate::{N_UUID_BYTES, Result, bytes_to_uuid_string, random::random_bytes};

pub fn uuid_v4() -> Result<String> {
    let bytes = uuid_v4_from_random(random_bytes::<N_UUID_BYTES>()?);

    Ok(bytes_to_uuid_string(bytes))
}

fn uuid_v4_from_random(mut bytes: [u8; N_UUID_BYTES]) -> [u8; N_UUID_BYTES] {
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_v4_has_version_4() {
        let uuid = uuid_v4_from_random([0xff; N_UUID_BYTES]);

        assert_eq!(uuid[6] >> 4, 0x4);
    }

    #[test]
    fn uuid_v4_has_rfc_variant() {
        let uuid = uuid_v4_from_random([0xff; N_UUID_BYTES]);

        assert_eq!(uuid[8] >> 6, 0b10);
    }

    #[test]
    fn uuid_v4_preserves_random_bits_except_version_and_variant() {
        let input = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0xaa, 0x77, 0xcc, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];

        let uuid = uuid_v4_from_random(input);

        // bytes[6]: 0xaa -> 0x4a
        assert_eq!(uuid[6], 0x4a);

        // bytes[8]: 0xcc -> 0x8c
        assert_eq!(uuid[8], 0x8c);

        assert_eq!(&uuid[..6], &input[..6]);
        assert_eq!(uuid[7], input[7]);
        assert_eq!(&uuid[9..], &input[9..]);
    }

    #[test]
    fn uuid_v4_string_has_expected_format() {
        let input = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0xaa, 0x77, 0xcc, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];

        let s = bytes_to_uuid_string(uuid_v4_from_random(input));

        assert_eq!(s, "00112233-4455-4a77-8c99-aabbccddeeff");
    }
}
