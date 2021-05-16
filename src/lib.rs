use std::iter::Chain;

mod qit {
    const byte_match: [u8; 8] = [0b0, 0b1, 0b11, 0b111, 0b1111, 0b1_1111, 0b11_1111, 0b111_1111];


    fn bit_match(match_count: usize, left: &[u8], right: &[u8]) -> bool {
        let bytes = match_count / 8;
        let bits = match_count % 8;
        let lefti = left.len() - bytes;
        let righti = right.len() - bytes;

        let tailend = bits == 0 || left[lefti - 1] & byte_match[bits] == right[righti - 1] & byte_match[bits];
        tailend && left[lefti..] == right[righti..]
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const example_bs_0 : [u8; 5] = [0x43, 0x33, 0xF8, 0x00, 0xFA];
        const example_bs_1 : [u8; 5] = [0x43, 0x33, 0xF8, 0x80, 0xFA];
        // Room for improvement

        #[test]
        fn bit_match_same_string() {
            assert!(bit_match(40, &example_bs_0, &example_bs_0))
        }

        #[test]
        fn bit_match_same_end() {
            assert!(bit_match(15, &example_bs_0, &example_bs_1))
        }

        #[test]
        fn bit_match_different_end() {
            assert!(!bit_match(16, &example_bs_0, &example_bs_1))
        }

        #[test]
        fn bit_match_different_end_2() {
            assert!(!bit_match(17, &example_bs_0, &[0x01, 0x00, 0xFA]))
        }

        #[test]
        fn bit_match_only_same_at_last_bit() {
            assert!(bit_match(1, &[0xFE], &[0x0]))
        }

        #[test]
        fn bit_match_matches_one_byte() {
            let one_byte = [0b1111];
            assert!(bit_match(8, &one_byte, &one_byte))
        }

        #[should_panic]
        #[test]
        fn bit_match_matches_too_many_bits() {
            let one_byte = [0b1111];
            assert!(bit_match(9, &one_byte, &one_byte))
        }

    }

}
