pub fn to_u16(bytes: &[u8], start: usize, end: usize) -> u16 {
    let input: [u8; 2] = bytes[start..end + 1]
        .try_into()
        .expect("incorrect length of slice");
    u16::from_be_bytes(input)
}
