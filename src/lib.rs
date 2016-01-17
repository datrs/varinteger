#![crate_type = "lib"]
#![crate_name = "varinteger"]

pub fn length (value: u64) -> usize {
  if value < 128 {
    return 1;
  } else if value < 16384 {
    return 2;
  } else if value < 2097152 {
    return 3;
  } else if value < 268435456 {
    return 4;
  } else if value < 34359738368 {
    return 5;
  } else if value < 4398046511104 {
    return 6;
  } else if value < 562949953421312 {
    return 7;
  } else if value < 72057594037927936 {
    return 8;
  } else if value < 9223372036854775808 {
    return 9;
  } else {
    return 10;
  }
}

pub fn encode (value: u64, buf: &mut [u8]) -> usize {
  return encode_with_offset(value, buf, 0);
}

pub fn encode_with_offset (value: u64, buf: &mut [u8], offset: usize) -> usize {
  let mut off = offset;
  let mut val = value;

  while val > 127 {
    buf[off] = (val as u8) | 128;
    off += 1;
    val >>= 7;
  }
  buf[off] = val as u8;

  return off + 1 - offset;
}

pub fn decode (buf: &[u8], value: &mut u64) -> usize {
  return decode_with_offset(buf, 0, value);
}

pub fn decode_with_offset (buf: &[u8], offset: usize, value: &mut u64) -> usize {
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

  return off - offset;
}

pub fn signed_length (value: i64) -> usize {
  return length(unsign(value));
}

pub fn signed_encode (value: i64, buf: &mut [u8]) -> usize {
  return encode_with_offset(unsign(value), buf, 0);
}

pub fn signed_encode_with_offset (value: i64, buf: &mut [u8], offset: usize) -> usize {
  return encode_with_offset(unsign(value), buf, offset);
}

pub fn signed_decode (buf: &[u8], value: &mut i64) -> usize {
  return signed_decode_with_offset(buf, 0, value);
}

pub fn signed_decode_with_offset (buf: &[u8], offset: usize, value: &mut i64) -> usize {
  let mut val = 0;
  let off = decode_with_offset(buf, offset, &mut val);
  *value = sign(val);
  return off;
}

fn unsign (value: i64) -> u64 {
  if value >= 0 {
    return (value * 2) as u64;
  } else {
    return (value * -2 - 1) as u64;
  }
}

fn sign (value: u64) -> i64 {
  if value & 1 != 0 {
    return -(((value + 1) / 2) as i64);
  } else {
    return (value / 2) as i64;
  }
}

// TODO: how do move these to a different file?

#[test]
fn test_encode () {
  let mut buf = [0; 512];
  assert_eq!(encode(100, &mut buf), 1);
  assert_eq!(buf[0], 100);

  assert_eq!(encode(1000, &mut buf), 2);
  assert_eq!(buf[0], 232);
  assert_eq!(buf[1], 7);
}

#[test]
fn test_decode () {
  let mut value = 0 as u64;
  assert_eq!(decode(&[100], &mut value), 1);
  assert_eq!(value, 100);

  assert_eq!(decode(&[232, 7], &mut value), 2);
  assert_eq!(value, 1000);
}

#[test]
fn test_length () {
  assert_eq!(length(100), 1);
  assert_eq!(length(1000), 2);
}

#[test]
fn test_signed_encode () {
  let mut buf = [0; 512];
  assert_eq!(signed_encode(100, &mut buf), 2);
  assert_eq!(buf[0], 200);
  assert_eq!(buf[1], 1);

  assert_eq!(signed_encode(-100, &mut buf), 2);
  assert_eq!(buf[0], 199);
  assert_eq!(buf[1], 1);
}

#[test]
fn test_signed_decode () {
  let mut value = 0 as i64;
  assert_eq!(signed_decode(&[200, 1], &mut value), 2);
  assert_eq!(value, 100);

  assert_eq!(signed_decode(&[199, 1], &mut value), 2);
  assert_eq!(value, -100);
}

#[test]
fn test_signed_length () {
  assert_eq!(signed_length(100), 2);
  assert_eq!(signed_length(-100), 2);
}
