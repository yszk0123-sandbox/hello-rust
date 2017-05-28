extern crate hello_rust;
use hello_rust::{basic_examples, match_examples, closure_examples, trait_examples};

fn main() {
    basic_examples::greeting::run();
    match_examples::basic::run();
    closure_examples::basic::run();
    trait_examples::without_trait::run();
    trait_examples::with_trait::run();
}
