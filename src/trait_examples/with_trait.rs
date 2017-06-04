struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn get_area<T: HasArea>(shape: T) -> f64 {
    shape.area()
}

#[test]
fn call_area_test() {
    let rectangle = Rectangle {
        x: 0f64,
        y: 0f64,
        width: 2f64,
        height: 3f64,
    };

    assert_eq!(6f64, rectangle.area());
    assert_eq!(6f64, get_area(rectangle));
}
