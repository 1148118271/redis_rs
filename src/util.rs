

pub fn to_string(v: &[u8]) -> String {
    unsafe {
        String::from_utf8_unchecked(Vec::from(v))
    }
}