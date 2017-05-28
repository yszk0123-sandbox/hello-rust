fn hello_closure() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));
}

fn hello_trait_without_trait() {
    struct Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    fn test() {
        let rectangle = Rectangle {
            x: 0f64,
            y: 0f64,
            width: 2f64,
            height: 3f64,
        };

        assert_eq!(6f64, rectangle.area());
    }

    test();
}

fn hello_trait_with_trait() {
    struct Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    trait HasArea {
      fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    fn get_area<T: HasArea>(shape: T) -> f64 {
      shape.area()
    }

    fn test() {
        let rectangle = Rectangle {
            x: 0f64,
            y: 0f64,
            width: 2f64,
            height: 3f64,
        };

        assert_eq!(6f64, rectangle.area());
        assert_eq!(6f64, get_area(rectangle));
    }

    test();
}

fn main() {
    println!("Hello, world!");
    hello_closure();
    hello_trait_without_trait();
    hello_trait_with_trait();
}
