pub fn u64_to_bytes(num: u64) -> [u8; 8] {
    num.to_le_bytes()
}
