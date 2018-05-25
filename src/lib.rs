#![crate_type = "lib"]
#![crate_name = "varinteger"]

pub fn length(value: u64) -> usize {
  if value < 128 {
    1
  } else if value < 16384 {
    2
  } else if value < 2097152 {
    3
  } else if value < 268435456 {
    4
  } else if value < 34359738368 {
    5
  } else if value < 4398046511104 {
    6
  } else if value < 562949953421312 {
    7
  } else if value < 72057594037927936 {
    8
  } else if value < 9223372036854775808 {
    9
  } else {
    10
  }
}

pub fn encode(value: u64, buf: &mut [u8]) -> usize {
  encode_with_offset(value, buf, 0)
}

pub fn encode_with_offset(value: u64, buf: &mut [u8], offset: usize) -> usize {
  let mut off = offset;
  let mut val = value;

  while val > 127 {
    buf[off] = (val as u8) | 128;
    off += 1;
    val >>= 7;
  }
  buf[off] = val as u8;

  off + 1 - offset
}

pub fn decode(buf: &[u8], value: &mut u64) -> usize {
  return decode_with_offset(buf, 0, value);
}

pub fn decode_with_offset(buf: &[u8], offset: usize, value: &mut u64) -> usize {
  let mut val = 0 as u64;
  let mut fac = 1 as u64;
  let mut off = offset;

  loop {
    let byte = buf[off];
    off += 1;
    val += fac * ((byte & 127) as u64);
    fac <<= 7;
    if byte & 128 == 0 {
      break;
    }
  }

  *value = val;

  off - offset
}

pub fn signed_length(value: i64) -> usize {
  length(unsign(value))
}

pub fn signed_encode(value: i64, buf: &mut [u8]) -> usize {
  encode_with_offset(unsign(value), buf, 0)
}

pub fn signed_encode_with_offset(
  value: i64,
  buf: &mut [u8],
  offset: usize,
) -> usize {
  encode_with_offset(unsign(value), buf, offset)
}

pub fn signed_decode(buf: &[u8], value: &mut i64) -> usize {
  signed_decode_with_offset(buf, 0, value)
}

pub fn signed_decode_with_offset(
  buf: &[u8],
  offset: usize,
  value: &mut i64,
) -> usize {
  let mut val = 0;
  let off = decode_with_offset(buf, offset, &mut val);
  *value = sign(val);
  off
}

fn unsign(value: i64) -> u64 {
  if value >= 0 {
    (value * 2) as u64
  } else {
    (value * -2 - 1) as u64
  }
}

fn sign(value: u64) -> i64 {
  if value & 1 != 0 {
    -(((value + 1) / 2) as i64)
  } else {
    (value / 2) as i64
  }
}
