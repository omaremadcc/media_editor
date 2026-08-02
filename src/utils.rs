pub fn calculate_little_endian(buffer: &[u8]) -> u32 {
    let mut result = 0u32;
    for (i, &byte) in buffer.iter().enumerate() {
        result |= (byte as u32) << (i * 8);
    }
    result
}

pub fn to_little_endian(value: u32) -> [u8; 4] {
    [
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
    ]
}
