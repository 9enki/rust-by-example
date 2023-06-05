fn main() {
    let _logical: bool = true;
    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;
    let _default_float = 3.0;
    let _default_integer = 10;

    let mut _inferred_type = 12;

    _inferred_type = 4294967296i64;

    let mut _mutable = 12;

    _mutable = 21;

    // error
    // mutable = true;

    let _mutable = true;
}
