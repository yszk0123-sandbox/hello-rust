#[test]
fn call_with_one_argument_test() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));
}
