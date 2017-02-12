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

#![allow(dead_code)]

use std::mem;

/// Splits the given u32 `value` into a vector of bytes in little-endian format.
pub fn u32_to_bytes(val: u32) -> Vec<u8> {
    (0..mem::size_of::<u32>())
        .map(|i| ((val >> (8 * i)) & 0xff) as u8)
        .collect()
}

/// Splits the given u64 `value` into a vector of bytes in little-endian format.
pub fn u64_to_bytes(val: u64) -> Vec<u8> {
    (0..mem::size_of::<u64>())
        .map(|i| ((val >> (8 * i)) & 0xff) as u8)
        .collect()
}

/// Gets the bit pattern of the given f32 `value` as a vector of bytes
/// in little-endian format.
pub fn f32_to_bytes(val: f32) -> Vec<u8> {
    let val = unsafe { mem::transmute::<f32, u32>(val) };
    u32_to_bytes(val)
}

/// Gets the bit pattern of the given f64 `value` as a vector of bytes
/// in little-endian format.
pub fn f64_to_bytes(val: f64) -> Vec<u8> {
    let val = unsafe { mem::transmute::<f64, u64>(val) };
    let mut low = u32_to_bytes((val & 0xffffffff) as u32);
    let mut high = u32_to_bytes(((val >> 32) & 0xffffffff) as u32);
    low.append(&mut high);
    low
}

/// Collects at most the first 4 bytes from the given `values` into a 32-bit
/// unsigned integer following the little-endian convention.
pub fn bytes_to_u32_le(values: &[u8]) -> u32 {
    let len = if values.len() < 4 {
        values.len()
    } else {
        4
    };
    let mut word = 0u32;
    for i in 0..len {
        word = (word << 8) | (values[len - i - 1] as u32);
    }
    word
}

/// Bitwisely casts the given f32 `value` to u32.
/// in little-endian format.
#[inline(always)]
pub fn f32_to_u32(value: f32) -> u32 {
    unsafe { mem::transmute::<f32, u32>(value) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u32_to_bytes() {
        assert_eq!(vec![0x12, 0x34, 0x56, 0x78], u32_to_bytes(0x78563412));
    }

    #[test]
    fn test_u64_to_bytes() {
        assert_eq!(vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef],
                   u64_to_bytes(0xefcdab9078563412));
    }

    #[test]
    fn test_f32_to_bytes() {
        // Bit pattern for 0.575 is 0x3f133333.
        assert_eq!(vec![0x33, 0x33, 0x13, 0x3f], f32_to_bytes(0.575));
    }

    #[test]
    fn test_bytes_to_u32_le() {
        assert_eq!(0x78563412u32, bytes_to_u32_le(&[0x12, 0x34, 0x56, 0x78]));
        assert_eq!(0x563412u32, bytes_to_u32_le(&[0x12, 0x34, 0x56]));
        assert_eq!(0x3412u32, bytes_to_u32_le(&[0x12, 0x34]));
        assert_eq!(0x12u32, bytes_to_u32_le(&[0x12]));
        assert_eq!(0x0u32, bytes_to_u32_le(&[]));
    }
}
