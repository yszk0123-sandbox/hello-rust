pub fn to_word<'a>(x: i32) -> &'a str {
    match x {
        1 => "one",
        _ => "something else",
    }
}

#[test]
fn call_with_defined_number_test() {
    assert_eq!("one", to_word(1));
}

#[test]
fn call_with_undefined_number_test() {
    assert_eq!("something else", to_word(2));
}
