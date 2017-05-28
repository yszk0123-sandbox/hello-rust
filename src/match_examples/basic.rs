fn to_word<'a>(x: i32) -> &'a str {
    match x {
        1 => "one",
        _ => "something else",
    }
}

pub fn run() {
    assert_eq!("one", to_word(1));
    assert_eq!("something else", to_word(2));
}
