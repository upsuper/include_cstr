# include_cstr

<!-- cargo-sync-readme start -->

A macro for getting `&'static CStr` from a file.

This macro checks whether the content of the given file is valid for `CStr`
at compile time, and returns a static reference of `CStr`.

This macro can be used to to initialize constants on Rust 1.46 and above.

It currently requires nightly compiler for [`proc_macro_span`][proc_macro_span] feature
for resolving relative path to the file,
so that it can be used in a similar way as `include_str!` and `include_bytes!` macro.

[proc_macro_span]: https://doc.rust-lang.org/unstable-book/library-features/proc-macro-span.html

## Example

```rust
use include_cstr::include_cstr;
use std::ffi::CStr;

let example = include_cstr!("example.txt");
assert_eq!(example, CStr::from_bytes_with_nul(b"content in example.txt\0").unwrap());
```

<!-- cargo-sync-readme end -->
