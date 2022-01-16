use nom::{error::{Error, ErrorKind}, Err, IResult};

#[test]
fn complete_bytes_test() {
  use nom::bytes::complete::{is_a, is_not};
  fn digits(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(b'0'..=b'9')(i) // RangeInclusive
  }

  fn not_digits(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_not(b'0'..=b'9')(i) // RangeInclusive
  }
  assert_eq!(digits(&b"123.456;"[..]), Ok((&b".456;"[..], &b"123"[..])));
  assert_eq!(digits(&b"hello!"[..]), Err(Err::Error(Error { input: &b"hello!"[..], code: ErrorKind::IsA })));
  assert_eq!(not_digits(&b"123.456;"[..]), Err(Err::Error(Error { input: &b"123.456;"[..], code: ErrorKind::IsNot })));
  assert_eq!(not_digits(&b"hello!"[..]), Ok((&b""[..], &b"hello!"[..])));
}

#[test]
fn streaming_bytes_test() {
  use nom::bytes::streaming::{is_a, is_not};
  fn digits(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(b'0'..=b'9')(i) // RangeInclusive
  }
  fn not_digits(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_not(b'0'..=b'9')(i) // RangeInclusive
  }
  assert_eq!(digits(&b"123.456;"[..]), Ok((&b".456;"[..], &b"123"[..])));
  assert_eq!(digits(&b"hello!"[..]), Err(Err::Error(Error { input: &b"hello!"[..], code: ErrorKind::IsA })));
  assert_eq!(not_digits(&b"123.456;"[..]), Err(Err::Error(Error { input: &b"123.456;"[..], code: ErrorKind::IsNot })));
  assert_eq!(not_digits(&b"hello!0"[..]), Ok((&b"0"[..], &b"hello!"[..])));
}

#[test]
fn complete_character_test() {
  use nom::character::complete::{one_of, none_of};
  fn digit(i: &str) -> IResult<&str, char> {
    one_of('0'..='9')(i) // RangeInclusive
  }
  fn not_digit(i: &str) -> IResult<&str, char> {
    none_of('0'..='9')(i) // RangeInclusive
  }
  assert_eq!(digit("123.456;"), Ok(("23.456;", '1')));
  assert_eq!(digit("hello!"), Err(Err::Error(Error { input: "hello!", code: ErrorKind::OneOf })));
  assert_eq!(not_digit("123.456;"), Err(Err::Error(Error { input: "123.456;", code: ErrorKind::NoneOf })));
  assert_eq!(not_digit("hello!"), Ok(("ello!", 'h')));
}

#[test]
fn streaming_character_test() {
  use nom::character::streaming::{one_of, none_of};
  fn digit(i: &str) -> IResult<&str, char> {
    one_of('0'..='9')(i) // RangeInclusive
  }
  fn not_digit(i: &str) -> IResult<&str, char> {
    none_of('0'..='9')(i) // RangeInclusive
  }
  assert_eq!(digit("123.456;"), Ok(("23.456;", '1')));
  assert_eq!(digit("hello!"), Err(Err::Error(Error { input: "hello!", code: ErrorKind::OneOf })));
  assert_eq!(not_digit("123.456;"), Err(Err::Error(Error { input: "123.456;", code: ErrorKind::NoneOf })));
  assert_eq!(not_digit("hello!"), Ok(("ello!", 'h')));
}

#[test]
fn other_ranges_test() {
  use nom::bytes::complete::is_a;
  fn lows(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(..128)(i) // RangeTo
  }
  fn mid_highs(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(128..192)(i) // Range
  }
  fn mins(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(..=0)(i) // RangeToInclusive
  }
  fn very_highs(i: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a(192..)(i) // RangeFrom
  }

  let lows_data = &[0, 1, 2, 3, 126, 127, 128][..];
  assert_eq!(lows(lows_data), Ok((&lows_data[6..], &lows_data[..6])));
    
  let mid_highs_data = &[128, 129, 191, 192][..];
  assert_eq!(mid_highs(mid_highs_data), Ok((&mid_highs_data[3..], &mid_highs_data[..3])));
    
  let mins_data = &[0, 1, 2, 3, 126, 127, 128][..];
  assert_eq!(mins(mins_data), Ok((&mins_data[1..], &mins_data[..1])));
    
  let very_highs_data = &[193, 192, 191][..];
  assert_eq!(very_highs(very_highs_data), Ok((&very_highs_data[2..], &very_highs_data[..2])));
}
