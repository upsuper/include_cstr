use include_cstr::include_cstr;
use std::ffi::CStr;

fn main() {
    let foo: &'static CStr = include_cstr!("content.bin");
    let expected = b"\xde\xad\xbe\xef\0";
    assert_eq!(foo, CStr::from_bytes_with_nul(expected).unwrap());
}
