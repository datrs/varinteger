extern crate failure;
extern crate varinteger;
use failure::Error;

use varinteger::{
  decode, encode, length, signed_decode, signed_encode, signed_length,
};

#[test]
fn test_encode() {
  let mut buf = [0; 512];
  assert_eq!(encode(100, &mut buf).unwrap(), 1);
  assert_eq!(buf[0], 100);

  assert_eq!(encode(1000, &mut buf).unwrap(), 2);
  assert_eq!(buf[0], 232);
  assert_eq!(buf[1], 7);
}

#[test]
fn test_encode_failure() {
  let mut buf = [0; 1];
  assert!(encode(1000, &mut buf).is_err());
}

#[test]
fn test_decode() {
  assert_eq!(decode(&[100]).unwrap(), (1, 100));
  assert_eq!(decode(&[232, 7]).unwrap(), (2, 1000));
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
  assert_eq!(signed_encode(100, &mut buf).unwrap(), 2);
  assert_eq!(buf[0], 200);
  assert_eq!(buf[1], 1);

  assert_eq!(signed_encode(-100, &mut buf).unwrap(), 2);
  assert_eq!(buf[0], 199);
  assert_eq!(buf[1], 1);
}

#[test]
fn test_signed_decode() {
  assert_eq!(signed_decode(&[200, 1]).unwrap(), (2, 100));

  assert_eq!(signed_decode(&[199, 1]).unwrap(), (2, -100));
}

#[test]
fn test_signed_length() {
  assert_eq!(signed_length(100), 2);
  assert_eq!(signed_length(-100), 2);
}
