# varinteger

Rust module for encoding/decoding [varints](https://developers.google.com/protocol-buffers/docs/encoding) that doesn't do any IO. Inspired by the Node.js [varint](https://github.com/chrisdickinson/varint) module. I mostly wrote this to learn more about Rust.

[![build status](http://img.shields.io/travis/mafintosh/varinteger-rs.svg?style=flat)](http://travis-ci.org/mafintosh/varinteger-rs)

## Usage

``` rs
extern crate varinteger;

let mut buf = [0; 512];

// encode 1000 as a varint into buf. returns how many bytes it wrote
let bytes_encoded = varinteger::encode(1000, buf);

let mut value = 0 as u64;
let bytes_decoded = varinteger::decode(buf, &mut value);

println!("encoded and decoded {}", value);
```

## API

#### `encode(value: u64, buf: &mut [u8]) -> usize`

Encode a `u64` integer to the buffer. Returns how many bytes were encoded.

#### `encode_with_offset(value: u64, buf: &mut [u8], offset: usize) -> usize`

Encode a `u64` integer at a specific offset in the buffer. Returns how many bytes were encoded.

#### `length(value: u64) -> usize`

Returns how many bytes are needed to encode value.

#### `signed_encode(value: i64, buf: &mut [u8]) -> usize`

Encode a `i64` (signed) integer to the buffer. Returns how many bytes were encoded.

#### `signed_encode_with_offset(value: i64, buf: &mut [u8], offset: usize) -> usize`

Encode a `i64` (signed) integer at a specific offset in the buffer. Returns how many bytes were encoded.

#### `signed_length(value: i64) -> usize`

Returns how many bytes are needed to encode value.

## License

MIT
