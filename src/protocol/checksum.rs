/// Calculate the SHDLC checksum for a frame slice.
/// Checksum is the inverted sum of all bytes modulo 256: `(!sum) as u8`.
#[inline]
pub fn calculate_checksum(data: &[u8]) -> u8 {
    let sum: u32 = data.iter().map(|&b| b as u32).sum();
    (!sum) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_all_zeros() {
        assert_eq!(calculate_checksum(&[0x00, 0x00, 0x00]), 0xFF);
    }

    #[test]
    fn test_checksum_sample() {
        // [0x00, 0x00, 0x00, 0x00] -> sum = 0 -> !0 = 0xFF
        assert_eq!(calculate_checksum(&[0x00, 0x00, 0x00, 0x00]), 0xFF);
        // [0x00, 0x00, 0x00, 0x01] -> sum = 1 -> !1 = 0xFE
        assert_eq!(calculate_checksum(&[0x00, 0x00, 0x00, 0x01]), 0xFE);
    }
}
