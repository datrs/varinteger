extern crate varinteger;

use varinteger::{
  decode, encode, length, signed_decode, signed_encode, signed_length,
};

#[test]
fn test_encode() {
  let mut buf = [0; 512];
  assert_eq!(encode(100, &mut buf), 1);
  assert_eq!(buf[0], 100);

  assert_eq!(encode(1000, &mut buf), 2);
  assert_eq!(buf[0], 232);
  assert_eq!(buf[1], 7);
}

#[test]
fn test_decode() {
  let mut value = 0 as u64;
  assert_eq!(decode(&[100], &mut value), 1);
  assert_eq!(value, 100);

  assert_eq!(decode(&[232, 7], &mut value), 2);
  assert_eq!(value, 1000);
}

#[test]
fn test_length() {
  assert_eq!(length(100), 1);
  assert_eq!(length(1000), 2);

  assert_eq!(length(1 << 49), 8);
  assert_eq!(length((1 << 56) - 1), 8);

  assert_eq!(length(1 << 56), 9);
  assert_eq!(length((1 << 63) - 1), 9);

  assert_eq!(length(1 << 63), 10);
}

#[test]
fn test_signed_encode() {
  let mut buf = [0; 512];
  assert_eq!(signed_encode(100, &mut buf), 2);
  assert_eq!(buf[0], 200);
  assert_eq!(buf[1], 1);

  assert_eq!(signed_encode(-100, &mut buf), 2);
  assert_eq!(buf[0], 199);
  assert_eq!(buf[1], 1);
}

#[test]
fn test_signed_decode() {
  let mut value = 0 as i64;
  assert_eq!(signed_decode(&[200, 1], &mut value), 2);
  assert_eq!(value, 100);

  assert_eq!(signed_decode(&[199, 1], &mut value), 2);
  assert_eq!(value, -100);
}

#[test]
fn test_signed_length() {
  assert_eq!(signed_length(100), 2);
  assert_eq!(signed_length(-100), 2);
}
