use include_cstr::include_cstr;
use std::ffi::CStr;

macro_rules! include_cstr_expr {
    ($s:expr) => {
        include_cstr!($s)
    };
}

macro_rules! include_cstr_literal {
    ($s:literal) => {
        include_cstr!($s)
    };
}

fn main() {
    let foo: &'static CStr = include_cstr_expr!("content.txt");
    assert_eq!(foo, CStr::from_bytes_with_nul(b"hello world!\0").unwrap());
    let bar: &'static CStr = include_cstr_literal!("content.txt");
    assert_eq!(bar, CStr::from_bytes_with_nul(b"hello world!\0").unwrap());
}
