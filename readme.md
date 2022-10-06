# `muscleman`
`muscleman` is a library for creating and managing buffers. It gets its name from the fact that it is a buffer manager.
And thinking of "buff" as in "buff body", it is a library for managing buffers of data. Plus, muscle man is a funny character.
## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
muscleman = "0.3.0"
```
And this to your crate root:
```rust
extern crate muscleman;
use muscleman::buffer::Buffer;
```
## Examples
### Creating a buffer
```rust
use muscleman::buffer;

let mut buffer = buffer::new();
buffer.write_byte(0x01);
buffer.write_byte(0x02);

assert_eq!(buffer.len(), 2);
```
### Writing to a buffer
```rust
use muscleman::buffer;

let mut buffer = buffer::new();

// 8 to 64 bit signed integers
buffer.write_i8(1);
buffer.write_i16(2);
buffer.write_i32(3);
buffer.write_i64(4);

// 8 to 64 bit unsigned integers
buffer.write_u8(5);
buffer.write_u16(6);
buffer.write_u32(7);
buffer.write_u64(8);

// 32 and 64 bit floating point numbers
buffer.write_f32(9.0_f32);
buffer.write_f64(10.0_f64);

// Null terminated strings and length prefixed strings
buffer.write_string("Hello, world!");
buffer.write_string_with_length("Hello, world!"); // The length is written as a u32
```
### Reading from a buffer
```rust
use muscleman::buffer;

let mut buffer = buffer::new();

// Assume the data in the "Writing to a buffer" is currently in the buffer

// 8 to 64 bit signed integers
assert_eq!(buffer.read_i8(), 1);
assert_eq!(buffer.read_i16(), 2);
assert_eq!(buffer.read_i32(), 3);
assert_eq!(buffer.read_i64(), 4);

// 8 to 64 bit unsigned integers
assert_eq!(buffer.read_u8(), 5);
assert_eq!(buffer.read_u16(), 6);
assert_eq!(buffer.read_u32(), 7);
assert_eq!(buffer.read_u64(), 8);

// 32 and 64 bit floating point numbers
assert_eq!(buffer.read_f32(), 9.0_f32);
assert_eq!(buffer.read_f64(), 10.0_f64);

// Null terminated strings and length prefixed strings
assert_eq!(buffer.read_string(), "Hello, world!");
assert_eq!(buffer.read_string_with_length(), "Hello, world!");
```
## Why?
I was working on a project that required me to send and receive data from a server. I needed a
way to easily write and read data to and from a buffer. I looked around and found a few crates
that did this, but they were either too complicated or didn't do what I needed. So I decided to
make my own.
## License
`muscleman` is licensed under the MIT license. See the `LICENSE` file for more information.