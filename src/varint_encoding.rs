mod varint_encoding;

extern fn encoding_length (value: u64) -> usize {
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

fn encode (value: u64, buf: &mut [u8]) -> usize {
  return encode_with_offset(value, buf, 0);
}

fn encode_with_offset (value: u64, buf: &mut [u8], offset: usize) -> usize {
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

fn decode (buf: &[u8], value: &mut u64) -> usize {
  return decode_with_offset(buf, 0, value);
}

fn decode_with_offset (buf: &[u8], offset: usize, value: &mut u64) -> usize {
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

fn signed_encoding_length (value: i64) -> usize {
  return encoding_length(unsign(value));
}

fn signed_encode (value: i64, buf: &mut [u8]) -> usize {
  return encode_with_offset(unsign(value), buf, 0);
}

fn signed_encode_with_offset (value: i64, buf: &mut [u8], offset: usize) -> usize {
  return encode_with_offset(unsign(value), buf, offset);
}

fn signed_decode (buf: &[u8], value: &mut i64) -> usize {
  return signed_decode_with_offset(buf, 0, value);
}

fn signed_decode_with_offset (buf: &[u8], offset: usize, value: &mut i64) -> usize {
  let mut val = 0;
  let off = decode_with_offset(buf, offset, &mut val);
  *value = sign(val);
  return off;
}

fn main () {
  let len = signed_encoding_length(2000000);

  println!("need {} bytes to encode the number", len);

  let mut buf = [0; 512];
  let encode_len = signed_encode(2000000, &mut buf);

  for i in 0..encode_len {
    println!("buf[{}] = {}", i, buf[i]);
  }

  let mut val = 0;
  let decode_len = signed_decode(&buf, &mut val);

  println!("decoded buf to {} (consumed {} bytes)", val, decode_len);
}
