const BITS: [u8; 8] = [
    0b0, 0b1, 0b11, 0b111, 0b1111, 0b1_1111, 0b11_1111, 0b111_1111,
];

fn pad_bits(bytes: &[u8], size: usize) -> Vec<u8> {
    size.checked_sub(bytes.len())
        .map(|padding| {
            let mut result = vec![0; padding];
            result.extend_from_slice(bytes);
            result
        })
        .unwrap_or(Vec::from(bytes))
}

/// Tests that two byte slices match X bits at the end.
///
/// If the byte slices don't contain enough bits to reach `match_count` bits,
/// they are padded with 0's. This is done largely to deal with
/// `hex::decode("000")` returning an empty slice.
///
/// # Examples
///
/// ```rust, ignore
/// use dit::dit_core::work::bit_match;
/// let left = [0b1111_1111u8, 0b01010101];
/// let right = [0b11u8, 0b10101010];
/// assert!(bit_match(10, left, right));
/// // The 11th bit from the right doesn't match
/// assert!(!bit_match(11, left, right));
/// ```
pub fn bit_match(match_count: usize, left: &[u8], right: &[u8]) -> bool {
    let bytes = match_count / 8;
    let bitc = match_count % 8;
    let left = pad_bits(left, bytes + 1);
    let right = pad_bits(right, bytes + 1);
    let lefti = left.len() - bytes;
    let righti = right.len() - bytes;
    let tailend = bitc == 0 || left[lefti - 1] & BITS[bitc] == right[righti - 1] & BITS[bitc];
    tailend && left[lefti..] == right[righti..]
}

#[cfg(test)]
mod test {
    use super::*;

    const BYTES_0: [u8; 5] = [0x43, 0x33, 0xF8, 0x00, 0xFA];
    const BYTES_1: [u8; 5] = [0x43, 0x33, 0xF8, 0x80, 0xFA];
    const ONE_BYTE: [u8; 1] = [0b10_0000];
    // Room for improvement

    #[test]
    fn bit_match_same_string() {
        assert!(bit_match(40, &BYTES_0, &BYTES_0))
    }

    #[test]
    fn bit_match_same_end() {
        assert!(bit_match(15, &BYTES_0, &BYTES_1))
    }

    #[test]
    fn bit_match_different_end() {
        assert!(!bit_match(16, &BYTES_0, &BYTES_1))
    }

    #[test]
    fn bit_match_different_end_2() {
        assert!(!bit_match(17, &BYTES_0, &[0x01, 0x00, 0xFA]))
    }

    #[test]
    fn bit_match_only_same_at_last_bit() {
        assert!(bit_match(1, &[0xFE], &[0x0]))
    }

    #[test]
    fn bit_match_matches_one_byte() {
        assert!(bit_match(8, &ONE_BYTE, &ONE_BYTE))
    }

    #[test]
    fn bit_match_matching_smaller_than_threshold() {
        assert!(bit_match(9, &ONE_BYTE, &ONE_BYTE))
    }

    #[test]
    fn bit_match_matching_with_extra_zeroes() {
        let extra_zero = [0b0, 0b10_0000];
        assert!(bit_match(9, &extra_zero, &ONE_BYTE))
    }

    #[test]
    fn bit_match_matching_no_bytes_are_treated_like_zero() {
        assert!(bit_match(5, &[], &ONE_BYTE))
    }
}
