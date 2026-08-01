pub fn calculate_little_endian(buffer: &[u8]) -> u32 {
    let mut result = 0u32;
    for (i, &byte) in buffer.iter().enumerate() {
        result |= (byte as u32) << (i * 8);
    }
    result
}
