# varinteger

Rust module for encoding/decoding
[varints](https://developers.google.com/protocol-buffers/docs/encoding) that
doesn't do any IO. Inspired by the Node.js
[varint](https://github.com/chrisdickinson/varint) module.

[![build status](http://img.shields.io/travis/mafintosh/varinteger-rs.svg?style=flat)](http://travis-ci.org/mafintosh/varinteger-rs)

## Usage

```rust
extern crate varinteger;

let mut buf = [0; 512];

// encode 1000 as a varint into buf. returns how many bytes it wrote
let bytes_encoded = varinteger::encode(1000, buf);

let mut value = 0u64;
let bytes_decoded = varinteger::decode(buf, &mut value);

println!("encoded and decoded {}", value);
```

## License
MIT
