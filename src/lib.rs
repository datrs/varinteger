extern crate failure;
use failure::{bail, Error};

/// Encode a `u64` integer to the byte slice. Returns how many bytes were
/// encoded.
#[inline]
pub fn encode(value: u64, buf: &mut [u8]) -> Result<usize, Error> {
  encode_with_offset(value, buf, 0)
}

/// Encode a `u64` integer at a specific offset in the byte slice. Returns how
/// many bytes were encoded.
#[inline]
pub fn encode_with_offset(
  value: u64,
  buf: &mut [u8],
  offset: usize,
) -> Result<usize, Error> {
  let len = length(value);
  if buf.len() < len {
    bail!["buffer is too small to write varint"]
  }
  let mut v = value;
  let mut off = offset;
  while v > 127 {
    buf[off] = (v as u8) | 128;
    off += 1;
    v >>= 7;
  }
  buf[off] = v as u8;
  Ok(len)
}

/// Decode a byte slice into a `u64` integer. Returns how many bytes were
/// decoded.
#[inline]
pub fn decode(buf: &[u8]) -> Result<(usize, u64), Error> {
  decode_with_offset(buf, 0usize)
}

/// Decode a byte slice into a `u64` integer at a specific offset. Returns how
/// many bytes were decoded.
#[inline]
pub fn decode_with_offset(
  buf: &[u8],
  _offset: usize,
) -> Result<(usize, u64), Error> {
  let mut value = 0u64;
  let mut m = 1u64;
  let mut offset = _offset;
  for _i in 0..8 {
    if offset >= buf.len() {
      bail!["buffer supplied to varint decoding too small"]
    }
    let byte = buf[offset];
    offset += 1;
    value += m * u64::from(byte & 127);
    m *= 128;
    if byte & 128 == 0 {
      break;
    }
  }
  Ok((offset, value))
}

/// Returns how many bytes are needed to encode a value.
#[inline]
pub fn length(value: u64) -> usize {
  let msb = (64 - value.leading_zeros()) as usize;
  (msb.max(1) + 6) / 7
}

/// Returns how many bytes are needed to encode a value.
#[inline]
pub fn signed_length(value: i64) -> usize {
  length(unsign(value))
}

/// Encode a `i64` (signed) integer at a specific offset in the byte slice.
/// Returns how many bytes were encoded.
#[inline]
pub fn signed_encode(value: i64, buf: &mut [u8]) -> Result<usize, Error> {
  encode_with_offset(unsign(value), buf, 0)
}

/// Encode a `i64` (signed) integer at a specific offset in the byte slice.
/// Returns how many bytes were encoded.
#[inline]
pub fn signed_encode_with_offset(
  value: i64,
  buf: &mut [u8],
  offset: usize,
) -> Result<usize, Error> {
  encode_with_offset(unsign(value), buf, offset)
}

/// Decode a byte slice into a `i64` (signed) integer.  Returns how many bytes
/// were decoded.
#[inline]
pub fn signed_decode(buf: &[u8]) -> Result<(usize, i64), Error> {
  signed_decode_with_offset(buf, 0)
}

/// Decode a byte slice into a `i64` (signed) integer at a specific offset.
/// Returns how many bytes were decoded.
#[inline]
pub fn signed_decode_with_offset(
  buf: &[u8],
  offset: usize,
) -> Result<(usize, i64), Error> {
  let mut val = 0;
  let (off, value) = decode_with_offset(buf, offset)?;
  Ok((off, sign(value)))
}

/// Convert an `i64` into a `u64`.
#[inline]
fn unsign(value: i64) -> u64 {
  if value >= 0 {
    (value * 2) as u64
  } else {
    (value * -2 - 1) as u64
  }
}

/// Convert a `u64` into a `i64`.
#[inline]
fn sign(value: u64) -> i64 {
  if value & 1 != 0 {
    -(((value + 1) / 2) as i64)
  } else {
    (value / 2) as i64
  }
}
