use include_cstr::include_cstr;
use std::ffi::CStr;

const FOO: &CStr = include_cstr!("content.txt");
static BAR: &CStr = include_cstr!("content.txt");

fn main() {
    let expected = b"hello world!\0";
    assert_eq!(FOO, CStr::from_bytes_with_nul(expected).unwrap());
    assert_eq!(BAR, CStr::from_bytes_with_nul(expected).unwrap());
}
