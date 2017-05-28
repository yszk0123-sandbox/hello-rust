fn hello_closure() {
  let plus_one = |x: i32| x + 1;

  assert_eq!(2, plus_one(1));
}

fn main() {
  println!("Hello, world!");
  hello_closure();
}
