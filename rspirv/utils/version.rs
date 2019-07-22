use crate::spirv::Word;

pub fn create_version_from_word(version: Word) -> (u8, u8) {
    (((version & 0xff0000) >> 16) as u8, ((version & 0xff00) >> 8) as u8)
}
