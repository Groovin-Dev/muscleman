use crate::byte_order::ByteOrder;

pub struct Buffer {
    // The buffer's data
    data: Vec<u8>,

    // The buffer's length
    length: usize,

    // The buffer's capacity
    capacity: usize,

    // The buffer's position
    position: usize,

    // The buffer's mark
    mark: Option<usize>,

    // The buffer's byte order
    byte_order: ByteOrder,

    // The buffer's string encoding
    string_encoding: &'static str,

    // The buffer's string terminator
    string_terminator: &'static str
}

impl Buffer {

    //#region Constants

    // The default capacity of a buffer
    pub const DEFAULT_CAPACITY: usize = 1024;

    // The default byte order of a buffer
    pub const DEFAULT_BYTE_ORDER: ByteOrder = ByteOrder::BigEndian;

    // The default string encoding of a buffer
    pub const DEFAULT_STRING_ENCODING: &'static str = "utf-8";

    // The default string terminator of a buffer
    pub const DEFAULT_STRING_TERMINATOR: &'static str = "\0";

    //#endregion

    //#region Constructors

    // Default constructor
    pub fn new() -> Buffer {
        Buffer {
            data: Vec::with_capacity(Buffer::DEFAULT_CAPACITY),
            length: 0,
            capacity: Buffer::DEFAULT_CAPACITY,
            position: 0,
            mark: None,
            byte_order: Buffer::DEFAULT_BYTE_ORDER,
            string_encoding: Buffer::DEFAULT_STRING_ENCODING,
            string_terminator: Buffer::DEFAULT_STRING_TERMINATOR
        }
    }

    // All arguments constructor
    pub fn new_with_all_args(capacity: usize, byte_order: ByteOrder, string_encoding: &'static str, string_terminator: &'static str) -> Buffer {
        Buffer {
            data: Vec::with_capacity(capacity),
            length: 0,
            capacity: capacity,
            position: 0,
            mark: None,
            byte_order: byte_order,
            string_encoding: string_encoding,
            string_terminator: string_terminator
        }
    }

    //#endregion

    //# region Properties

    /// Gets the buffer's data
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    //#endregion Properties

    //#region Reading methods

    //#region Basic reading methods

    /// Reads a byte from the buffer.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position < self.length {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Reads n bytes from the buffer.
    pub fn read_bytes(&mut self, n: usize) -> Option<Vec<u8>> {
        if self.position + n <= self.length {
            let mut bytes = Vec::with_capacity(n);
            for i in 0..n {
                bytes.push(self.data[self.position + i]);
            }
            self.position += n;
            Some(bytes)
        } else {
            None
        }
    }

    /// Reads a boolean from the buffer.
    pub fn read_boolean(&mut self) -> Option<bool> {
        if self.position < self.length {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte != 0)
        } else {
            None
        }
    }

    //#endregion Basic reading methods

    //#region Integer reading methods

    //# region Signed integer reading methods

    /// Reads a signed 8-bit integer from the buffer.
    pub fn read_i8(&mut self) -> Option<i8> {
        if self.position < self.length {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte as i8)
        } else {
            None
        }
    }

    /// Reads a signed 16-bit integer from the buffer.
    pub fn read_i16(&mut self) -> Option<i16> {
        if self.position + 2 <= self.length {
            let mut bytes = [0; 2];
            for i in 0..2 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 2;
            Some(i16::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Reads a signed 32-bit integer from the buffer.
    pub fn read_i32(&mut self) -> Option<i32> {
        if self.position + 4 <= self.length {
            let mut bytes = [0; 4];
            for i in 0..4 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 4;
            Some(i32::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Reads a signed 64-bit integer from the buffer.
    pub fn read_i64(&mut self) -> Option<i64> {
        if self.position + 8 <= self.length {
            let mut bytes = [0; 8];
            for i in 0..8 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 8;
            Some(i64::from_be_bytes(bytes))
        } else {
            None
        }
    }

    //# endregion Signed integer reading methods

    //# region Unsigned integer reading methods

    /// Reads an unsigned 8-bit integer from the buffer.
    pub fn read_u8(&mut self) -> Option<u8> {
        if self.position < self.length {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Reads an unsigned 16-bit integer from the buffer.
    pub fn read_u16(&mut self) -> Option<u16> {
        if self.position + 2 <= self.length {
            let mut bytes = [0; 2];
            for i in 0..2 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 2;
            Some(u16::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Reads an unsigned 32-bit integer from the buffer.
    pub fn read_u32(&mut self) -> Option<u32> {
        if self.position + 4 <= self.length {
            let mut bytes = [0; 4];
            for i in 0..4 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 4;
            Some(u32::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Reads an unsigned 64-bit integer from the buffer.
    pub fn read_u64(&mut self) -> Option<u64> {
        if self.position + 8 <= self.length {
            let mut bytes = [0; 8];
            for i in 0..8 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 8;
            Some(u64::from_be_bytes(bytes))
        } else {
            None
        }
    }

    //# endregion Unsigned integer reading methods
    //#endregion Integer reading methods

    //#region Floating-point reading methods

    /// Reads a 32-bit floating-point number from the buffer.
    pub fn read_f32(&mut self) -> Option<f32> {
        if self.position + 4 <= self.length {
            let mut bytes = [0; 4];
            for i in 0..4 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 4;
            Some(f32::from_be_bytes(bytes))
        } else {
            None
        }
    }

    /// Reads a 64-bit floating-point number from the buffer.
    pub fn read_f64(&mut self) -> Option<f64> {
        if self.position + 8 <= self.length {
            let mut bytes = [0; 8];
            for i in 0..8 {
                bytes[i] = self.data[self.position + i];
            }
            self.position += 8;
            Some(f64::from_be_bytes(bytes))
        } else {
            None
        }
    }

    //#endregion Floating-point reading methods

    //#region String reading methods

    /// Reads a string from the buffer.
    /// Reads until the first null byte.
    pub fn read_string(&mut self) -> Option<String> {
        let mut string = String::new();
        while self.position < self.length {
            let byte = self.data[self.position];
            self.position += 1;
            if byte == 0 {
                break;
            }
            string.push(byte as char);
        }
        Some(string)
    }

    /// Reads a string from the buffer.
    /// Reads the length of the string from the buffer. Then reads that many bytes.
    pub fn read_string_with_length(&mut self) -> Option<String> {
        let length = self.read_u32()?;
        let mut string = String::new();
        for _ in 0..length {
            let byte = self.read_u8()?;
            string.push(byte as char);
        }
        Some(string)
    }

    //#endregion String reading methods

    //#endregion Reading methods

    //#region Writing methods

    //#region Basic writing methods

    /// Writes a byte to the buffer.
    pub fn write_byte(&mut self, byte: u8) {
        self.data.push(byte);
        self.length += 1;
    }

    /// Writes a byte array to the buffer.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.data.push(*byte);
        }
        self.length += bytes.len();
    }

    //#endregion Basic writing methods

    //#region Integer writing methods

    //# region Signed integer writing methods

    /// Writes a signed 8-bit integer to the buffer.
    pub fn write_i8(&mut self, value: i8) {
        self.data.push(value as u8);
        self.length += 1;
    }

    /// Writes a signed 16-bit integer to the buffer.
    pub fn write_i16(&mut self, value: i16) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 2;
    }

    /// Writes a signed 32-bit integer to the buffer.
    pub fn write_i32(&mut self, value: i32) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 4;
    }

    /// Writes a signed 64-bit integer to the buffer.
    pub fn write_i64(&mut self, value: i64) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 8;
    }

    //# endregion Signed integer writing methods

    //# region Unsigned integer writing methods

    /// Writes an unsigned 8-bit integer to the buffer.
    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
        self.length += 1;
    }

    /// Writes an unsigned 16-bit integer to the buffer.
    pub fn write_u16(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 2;
    }

    /// Writes an unsigned 32-bit integer to the buffer.
    pub fn write_u32(&mut self, value: u32) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 4;
    }

    /// Writes an unsigned 64-bit integer to the buffer.
    pub fn write_u64(&mut self, value: u64) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 8;
    }

    //# endregion Unsigned integer writing methods

    //#endregion Integer writing methods

    //#region Floating-point writing methods

    /// Writes a 32-bit floating-point number to the buffer.
    pub fn write_f32(&mut self, value: f32) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 4;
    }

    /// Writes a 64-bit floating-point number to the buffer.
    pub fn write_f64(&mut self, value: f64) {
        let bytes = value.to_be_bytes();
        for byte in &bytes {
            self.data.push(*byte);
        }
        self.length += 8;
    }

    //#endregion Floating-point writing methods

    //#region String writing methods

    /// Writes a string to the buffer.
    /// Writes as a null-terminated string.
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            self.data.push(byte);
        }
        self.data.push(0);
        self.length += string.len() + 1;
    }

    /// Writes a string to the buffer.
    /// Writes the length of the string as a 32-bit unsigned integer, then writes the string.
    pub fn write_string_with_length(&mut self, string: &str) {
        self.write_u32(string.len() as u32);
        for byte in string.bytes() {
            self.data.push(byte);
        }
        self.length += string.len();
    }

    //#endregion String writing methods

    //#endregion Writing methods
}

