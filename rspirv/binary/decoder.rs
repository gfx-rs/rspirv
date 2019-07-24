// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::spirv;

use std::result;
use super::DecodeError as Error;

pub type Result<T> = result::Result<T, Error>;

const WORD_NUM_BYTES: usize = 4;

/// The SPIR-V binary decoder.
///
/// Takes in a vector of bytes, and serves requests for raw SPIR-V words
/// or values of a specific SPIR-V enum type. Successful decoding will
/// surely consume the number of words decoded, while unsuccessful decoding
/// may consume any number of bytes.
///
/// TODO: The decoder should not conume words if an error occurs.
///
/// Different from the [`Parser`](struct.Parser.html),
/// this decoder is low-level; it has no knowledge of the SPIR-V grammar.
/// Given a vector of bytes, it solely responds to word decoding requests
/// via method calls: both raw words requests and decoding the raw words
/// into a value of a specific SPIR-V enum type.
///
/// It also provides a limit mechanism. Users can set a limit, and then
/// requesting words. If that limit is reached before the end of the
/// stream, [`State::LimitReached`](enum.ParseState.html) will be
/// returned.
///
/// # Errors
///
/// For its methods, there can be the following errors:
///
/// * `DecodeError::LimitReached(offset)` if the most recent limit has reached.
/// * `DecodeError::StreamExpected(offset)` if more bytes are needed to decode
///    the next word.
/// * `DecodeError::<spirv-enum>Unknown(offset, value)` if failed to decode the
///    next word as the given `<spirv-enum>`.
///
/// All errors contain the byte offset of the word failed decoding.
///
/// # Examples
///
/// ```
/// extern crate rspirv;
/// extern crate spirv_headers as spirv;
///
/// use rspirv::binary::{Decoder, DecodeError};
/// use spirv::SourceLanguage;
///
/// fn main() {
///     let v = vec![0x12, 0x34, 0x56, 0x78,
///                  0x90, 0xab, 0xcd, 0xef,
///                  0x02, 0x00, 0x00, 0x00];
///     let mut d = Decoder::new(&v);
///
///     assert_eq!(Ok(0x78563412), d.word());
///     assert_eq!(Ok(0xefcdab90), d.word());
///     assert_eq!(Ok(SourceLanguage::GLSL), d.source_language());
///
///     assert_eq!(Err(DecodeError::StreamExpected(12)), d.word());
/// }
/// ```
pub struct Decoder {
}
include!("autogen_decode_operand.rs");

#[cfg(test)]
mod tests {
    use crate::spirv;

    use super::Decoder;
    use crate::binary::DecodeError as Error;

    #[test]
    fn test_decoding_word_from_one_bytes() {
        let b = vec![1];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_two_bytes() {
        let b = vec![1, 2];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_three_bytes() {
        let b = vec![1, 2, 3];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::StreamExpected(0)), d.word());
    }

    #[test]
    fn test_decoding_word_from_four_bytes() {
        let b = vec![0x12, 0x34, 0x56, 0x78];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0x78563412), d.word());
    }

    #[test]
    #[rustfmt::skip]
    fn test_decoding_words() {
        let b = vec![0x12, 0x34, 0x56, 0x78,
                     0x90, 0xab, 0xcd, 0xef,
                     0x01, 0x23, 0x45, 0x67,
                     0x89, 0xfe, 0xdc, 0xba];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(vec![0x78563412, 0xefcdab90]), d.words(2));
        assert_eq!(Ok(vec![0x67452301]), d.words(1));
        assert_eq!(Ok(vec![0xbadcfe89]), d.words(1));
    }

    #[test]
    fn test_decoding_string() {
        {
            let b = vec![0x00, 0x00, 0x00, 0x00];
            let mut d = Decoder::new(&b);
            assert_eq!(Ok(String::new()), d.string());
        }
        {
            let b = b"ok".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Err(Error::StreamExpected(0)), d.string());
        }
        {
            let b = b"ok\0\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("ok".to_string()), d.string());
        }
        {
            let b = b"ok\0\0rust\0\0\0\0rocks\0\0\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("ok".to_string()), d.string());
            assert_eq!(Ok("rust".to_string()), d.string());
            assert_eq!(Ok("rocks".to_string()), d.string());
        }
        {
            let b = b"I..don't know..\0".to_vec();
            let mut d = Decoder::new(&b);
            assert_eq!(Ok("I..don't know..".to_string()), d.string());
        }
    }

    #[test]
    fn test_decoding_source_language() {
        let b = vec![0x02, 0x00, 0x00, 0x00];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(spirv::SourceLanguage::GLSL), d.source_language());
    }

    #[test]
    fn test_decoding_unknown_execution_model() {
        let b = vec![0xef, 0xbe, 0xad, 0xde];
        let mut d = Decoder::new(&b);
        assert_eq!(Err(Error::ExecutionModelUnknown(0, 0xdeadbeef)),
                   d.execution_model());
    }

    #[test]
    #[rustfmt::skip]
    fn test_offset() {
        let b = vec![0x12, 0x34, 0x56, 0x78,
                     0x90, 0xab, 0xcd, 0xef,
                     0x01, 0x23, 0x45, 0x67,
                     0x89, 0xfe, 0xdc, 0xba,
                     0x01, 0x00, 0x00, 0x00,
                     0xff, 0xff, 0xff, 0xff];
        let mut d = Decoder::new(&b);

        assert_eq!(0, d.offset());
        assert!(d.words(1).is_ok());
        assert_eq!(4, d.offset());
        assert!(d.words(2).is_ok());
        assert_eq!(12, d.offset());
        assert!(d.words(1).is_ok());
        assert_eq!(16, d.offset());

        assert!(d.source_language().is_ok());
        assert_eq!(20, d.offset());

        assert!(d.execution_model().is_err());
        assert_eq!(24, d.offset());
    }

    #[test]
    fn test_decoding_after_errors() {
        let b = vec![0x12, 0x34, 0x56, 0x78];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0x78563412), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
        assert_eq!(Err(Error::StreamExpected(4)), d.word());
    }

    #[test]
    fn test_limit() {
        let mut v = vec![];
        for _ in 0..12 {
            v.push(0xff);
        }
        let mut d = Decoder::new(&v);

        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(4);
        assert!(d.has_limit());
        assert!(!d.limit_reached());

        d.clear_limit();
        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(2);
        assert!(d.has_limit());
        assert!(!d.limit_reached());
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(d.has_limit());
        assert!(!d.limit_reached());
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());
        assert_eq!(Err(Error::LimitReached(8)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());
        assert_eq!(Err(Error::LimitReached(8)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());

        d.clear_limit();
        assert_eq!(Ok(0xffffffff), d.word());
        assert!(!d.has_limit());
        assert!(!d.limit_reached());

        d.set_limit(0);
        assert_eq!(Err(Error::LimitReached(12)), d.word());
        assert!(d.has_limit());
        assert!(d.limit_reached());

        d.clear_limit();
        assert_eq!(Err(Error::StreamExpected(12)), d.word());
    }

    #[test]
    fn test_decode_int64() {
        let b = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(0xefcdab9078563412), d.int64());
    }

    #[test]
    fn test_decode_float32() {
        let b = [0x14, 0xAE, 0x29, 0x42];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(42.42), d.float32());

        let b = [0xA4, 0x70, 0x45, 0xC1];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(-12.34), d.float32());
    }

    #[test]
    fn test_decode_float64() {
        let b = [0xF6, 0x28, 0x5C, 0x8F, 0xC2, 0x35, 0x45, 0x40];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(42.42), d.float64());

        let b = [0xAE, 0x47, 0xE1, 0x7A, 0x14, 0xAE, 0x28, 0xC0];
        let mut d = Decoder::new(&b);
        assert_eq!(Ok(-12.34), d.float64());
    }
}
