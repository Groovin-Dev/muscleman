//! # `muscleman`
//!
//! `muscleman` is a library for creating and managing buffers. It gets its name from the fact that it is a buffer manager.
//! And thinking of "buff" as in "buff body", it is a library for managing buffers of data. Plus, muscle man is a funny character.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! muscleman = "0.1.0"
//! ```
//!
//! And this to your crate root:
//!
//! ```rust
//! extern crate muscleman;
//!
//! use muscleman::buffer::Buffer;
//! ```
//!
//! ## Examples
//!
//! ### Creating a buffer
//!
//! ```rust
//! use muscleman::buffer;
//!
//! let mut buffer = buffer::new();
//!
//! buffer.write_byte(0x01);
//! buffer.write_byte(0x02);
//!
//! assert_eq!(buffer.len(), 2);
//! ```
//!
//! ### Writing to a buffer
//!
//! ```rust
//! use muscleman::buffer;
//!
//! let mut buffer = buffer::new();
//!
//! // 8 to 64 bit signed integers
//! buffer.write_i8(1);
//! buffer.write_i16(2);
//! buffer.write_i32(3);
//! buffer.write_i64(4);
//!
//! // 8 to 64 bit unsigned integers
//! buffer.write_u8(5);
//! buffer.write_u16(6);
//! buffer.write_u32(7);
//! buffer.write_u64(8);
//!
//! // 32 and 64 bit floating point numbers
//! buffer.write_f32(9.0_f32);
//! buffer.write_f64(10.0_f64);
//!
//! // Null terminated strings and length prefixed strings
//! buffer.write_string("Hello, world!");
//! buffer.write_string_with_length("Hello, world!"); // The length is written as a u32
//!
//! ```
//!
//! ### Reading from a buffer
//!
//! ```rust
//! use muscleman::buffer;
//!
//! let mut buffer = buffer::new();
//!
//! // Assume the data in the "Writing to a buffer" is currently in the buffer
//!
//! // 8 to 64 bit signed integers
//! assert_eq!(buffer.read_i8(), 1);
//! assert_eq!(buffer.read_i16(), 2);
//! assert_eq!(buffer.read_i32(), 3);
//! assert_eq!(buffer.read_i64(), 4);
//!
//! // 8 to 64 bit unsigned integers
//! assert_eq!(buffer.read_u8(), 5);
//! assert_eq!(buffer.read_u16(), 6);
//! assert_eq!(buffer.read_u32(), 7);
//! assert_eq!(buffer.read_u64(), 8);
//!
//! // 32 and 64 bit floating point numbers
//! assert_eq!(buffer.read_f32(), 9.0_f32);
//! assert_eq!(buffer.read_f64(), 10.0_f64);
//!
//! // Null terminated strings and length prefixed strings
//! assert_eq!(buffer.read_string(), "Hello, world!");
//! assert_eq!(buffer.read_string_with_length(), "Hello, world!");
//! ```
//!
//! ## Why?
//!
//! I was working on a project that required me to send and receive data from a server. I needed a
//! way to easily write and read data to and from a buffer. I looked around and found a few crates
//! that did this, but they were either too complicated or didn't do what I needed. So I decided to
//! make my own.
//!
//! ## License
//!
//! `muscleman` is licensed under the MIT license. See the `LICENSE` file for more information.

pub mod buffer;
pub mod byte_order;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::buffer::Buffer;

    //#region Basic reading tests

    #[test]
    fn read_single_byte() {
        let mut buffer = Buffer::new();
        let value = 0x01;
        buffer.write_byte(value);
        let res = buffer.read_byte();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_bytes() {
        let mut buffer = Buffer::new();
        let value = Vec::from([0x01, 0x02, 0x03, 0x04]);
        buffer.write_bytes(&value);
        let res = buffer.read_bytes(4);
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_bool() {
        let mut buffer = Buffer::new();
        let value = 0x1;
        buffer.write_byte(value);
        let res = buffer.read_boolean();
        assert_eq!(res, Some(value == 0x1));
    }

    //#endregion

    //#region Integer reading tests

    //#region Signed integer reading tests

    #[test]
    fn read_i8() {
        let mut buffer = Buffer::new();
        let value = 0x01;
        buffer.write_i8(value);
        let res = buffer.read_i8();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_i16() {
        let mut buffer = Buffer::new();
        let value = 0x0102;
        buffer.write_i16(value);
        let res = buffer.read_i16();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_i32() {
        let mut buffer = Buffer::new();
        let value = 0x01020304;
        buffer.write_i32(value);
        let res = buffer.read_i32();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_i64() {
        let mut buffer = Buffer::new();
        let value = 0x0102030405060708;
        buffer.write_i64(value);
        let res = buffer.read_i64();
        assert_eq!(res, Some(value));
    }

    //#endregion Signed integer reading tests

    //#region Unsigned integer reading tests

    #[test]
    fn read_u8() {
        let mut buffer = Buffer::new();
        let value = 0x01;
        buffer.write_u8(value);
        let res = buffer.read_u8();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_u16() {
        let mut buffer = Buffer::new();
        let value = 0x0102;
        buffer.write_u16(value);
        let res = buffer.read_u16();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_u32() {
        let mut buffer = Buffer::new();
        let value = 0x01020304;
        buffer.write_u32(value);
        let res = buffer.read_u32();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_u64() {
        let mut buffer = Buffer::new();
        let value = 0x0102030405060708;
        buffer.write_u64(value);
        let res = buffer.read_u64();
        assert_eq!(res, Some(value));
    }

    //#endregion Unsigned integer reading tests

    //#endregion Integer reading tests

    //#region Float reading tests

    #[test]
    fn read_f32() {
        let mut buffer = Buffer::new();
        let value = 1.0_f32;
        buffer.write_f32(value);
        let res = buffer.read_f32();
        assert_eq!(res, Some(value));
    }

    #[test]
    fn read_f64() {
        let mut buffer = Buffer::new();
        let value = 1.0_f64;
        buffer.write_f64(value);
        let res = buffer.read_f64();
        assert_eq!(res, Some(value));
    }

    //#endregion Float reading tests

    //#region String reading tests

    #[test]
    fn read_null_terminated_string() {
        let mut buffer = Buffer::new();
        let value = "Hello, world!";
        buffer.write_string(value);
        let res = buffer.read_string();
        assert_eq!(res, Some(value.to_string()));
    }

    #[test]
    fn read_string_with_length() {
        let mut buffer = Buffer::new();
        let value = "Hello, world!";
        buffer.write_string(value);
        let res = buffer.read_string_with_length();
        assert_eq!(res, Some(value.to_string()));
    }

    //#endregion String reading tests

    // Complete test with all reading methods
	#[test]
	fn test_all_reads() {
		// The buffer to test with
		let mut buffer = Buffer::new();

		// The values
		let v_byte = 0x01;
		let v_bool = true;
		let v_i8 = 0x01;
		let v_i16 = 0x0102;
		let v_i32 = 0x01020304;
		let v_i64 = 0x0102030405060708;
		let v_u8 = 0x01;
		let v_u16 = 0x0102;
		let v_u32 = 0x01020304;
		let v_u64 = 0x0102030405060708;
		let v_f32 = 1.0_f32;
		let v_f64 = 1.0_f64;
		let v_string = "Hello, world!";
		let v_string_with_length = "Hello, world!";

		// Write the values
		buffer.write_byte(v_byte);
		buffer.write_byte(v_bool as u8);
		buffer.write_i8(v_i8);
		buffer.write_i16(v_i16);
		buffer.write_i32(v_i32);
		buffer.write_i64(v_i64);
		buffer.write_u8(v_u8);
		buffer.write_u16(v_u16);
		buffer.write_u32(v_u32);
		buffer.write_u64(v_u64);
		buffer.write_f32(v_f32);
		buffer.write_f64(v_f64);
		buffer.write_string(v_string);
		buffer.write_string_with_length(v_string_with_length);

		// For testing reasons, we are going to write the buffer to a file here
		let mut file = File::create("test.bin").unwrap();
		file.write_all(buffer.data()).unwrap();

		// Read the values
		let r_byte = buffer.read_byte();
		let r_bool = buffer.read_boolean();
		let r_i8 = buffer.read_i8();
		let r_i16 = buffer.read_i16();
		let r_i32 = buffer.read_i32();
		let r_i64 = buffer.read_i64();
		let r_u8 = buffer.read_u8();
		let r_u16 = buffer.read_u16();
		let r_u32 = buffer.read_u32();
		let r_u64 = buffer.read_u64();
		let r_f32 = buffer.read_f32();
		let r_f64 = buffer.read_f64();
		let r_string = buffer.read_string();
		let r_string_with_length = buffer.read_string_with_length();

		// Assert the values
		assert_eq!(r_byte, Some(v_byte));
		assert_eq!(r_bool, Some(v_bool));
		assert_eq!(r_i8, Some(v_i8));
		assert_eq!(r_i16, Some(v_i16));
		assert_eq!(r_i32, Some(v_i32));
		assert_eq!(r_i64, Some(v_i64));
		assert_eq!(r_u8, Some(v_u8));
		assert_eq!(r_u16, Some(v_u16));
		assert_eq!(r_u32, Some(v_u32));
		assert_eq!(r_u64, Some(v_u64));
		assert_eq!(r_f32, Some(v_f32));
		assert_eq!(r_f64, Some(v_f64));
		assert_eq!(r_string, Some(v_string.to_string()));
		assert_eq!(r_string_with_length, Some(v_string_with_length.to_string()));
	}
}