use crate::spirv::Word;

pub fn create_version_from_word(version: Word) -> (u8, u8) {
    let bytes = version.to_le_bytes();
    (bytes[2], bytes[1])
}

pub fn create_word_from_version(major: u8, minor: u8) -> Word {
    Word::from_le_bytes([0, minor, major, 0])
}
