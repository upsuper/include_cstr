//! A macro for getting `&'static CStr` from a file.
//!
//! This macro checks whether the content of the given file is valid for `CStr`
//! at compile time, and returns a static reference of `CStr`.
//!
//! This macro can be used to to initialize constants on Rust 1.46 and above.
//!
//! It currently requires nightly compiler for [`proc_macro_span`][proc_macro_span] feature
//! for resolving relative path to the file,
//! so that it can be used in a similar way as `include_str!` and `include_bytes!` macro.
//!
//! [proc_macro_span]: https://doc.rust-lang.org/unstable-book/library-features/proc-macro-span.html
//!
//! ## Example
//!
//! ```rust
//! use include_cstr::include_cstr;
//! use std::ffi::CStr;
//!
//! let example = include_cstr!("example.txt");
//! assert_eq!(example, CStr::from_bytes_with_nul(b"content in example.txt\0").unwrap());
//! ```

#![feature(proc_macro_span)]

use crate::parse::parse_input;
use proc_macro::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::borrow::Cow;
use std::ffi::CString;
use std::fs;

mod parse;

struct Error(Span, Cow<'static, str>);

#[proc_macro]
pub fn include_cstr(input: TokenStream) -> TokenStream {
    let tokens = match check_file(input) {
        Ok((path, bytes)) => {
            // Use `include_bytes!()` to ensure that the source file using this macro gets
            // re-compiled when the content of the included file is changed.
            // We can't use `&*ptr` to convert the raw pointer to reference, because as of Rust
            // 1.46, dereferencing raw pointer in constants is unstable. This is being tracked in
            // https://github.com/rust-lang/rust/issues/51911
            // So we explicitly disable the clippy lint for this expression.
            quote!({
                const _: &[u8] = include_bytes!(#path);
                unsafe{
                    #[allow(clippy::transmute_ptr_to_ref)]
                    ::std::mem::transmute::<_, &::std::ffi::CStr>(
                        &[#(#bytes),*] as *const [u8] as *const ::std::ffi::CStr
                    )
                }
            })
        }
        Err(Error(span, msg)) => {
            let span = span.into();
            quote_spanned!(span => compile_error!(#msg))
        }
    };
    tokens.into()
}

fn check_file(input: TokenStream) -> Result<(String, Vec<u8>), Error> {
    let (path, literal) = parse_input(input)?;
    let span = literal.span();
    // Safety: the path comes from a valid str literal input from rustc, so it should be a
    // valid UTF-8 string.
    let path = unsafe { String::from_utf8_unchecked(path) };
    let full_path = span.source_file().path().parent().unwrap().join(&path);
    let content = fs::read(&full_path).map_err(|e| Error(span, format!("{}", e).into()))?;
    let content =
        CString::new(content).map_err(|_| Error(span, "nul byte found in the file".into()))?;
    Ok((path, content.into_bytes_with_nul()))
}
