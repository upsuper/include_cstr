use include_cstr::include_cstr;

fn main() {
    let _ = include_cstr!(1);
    let _ = include_cstr!(&1);
    let _ = include_cstr!(true);
    let _ = include_cstr!(("a"));
    let _ = include_cstr!('a');
    let _ = include_cstr!(b'a');
    let _ = include_cstr!(b"a");
}
