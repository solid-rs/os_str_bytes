#![cfg(feature = "raw_os_str")]

use os_str_bytes::RawOsStr;

mod common;
use common::RAW_WTF8_STRING;

fn test(result: &str, string: &RawOsStr) {
    assert_eq!(format!("RawOsStr({})", result), format!("{:?}", string));
    assert_eq!(
        format!("RawOsString({})", result),
        format!("{:?}", string.to_owned()),
    );
}

#[test]
fn test_debug_empty() {
    test("\"\"", RawOsStr::from_str(""));
}

#[test]
fn test_debug_wft8() {
    let wchar = if cfg!(unix) {
        "\\xED\\xA0\\xBD"
    } else {
        "\\u{D83D}"
    };
    test(&format!("\"foo{}\u{1F4A9}bar\"", wchar), RAW_WTF8_STRING);
}

#[test]
fn test_debug_quote() {
    test("\"foo\\\"bar\"", RawOsStr::from_str("foo\"bar"));
}
