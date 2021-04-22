use include_cstr::include_cstr;

fn main() {
    let _ = include_cstr!("non-existent");
}
