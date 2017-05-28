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

pub fn run() {
    let rectangle = Rectangle {
        x: 0f64,
        y: 0f64,
        width: 2f64,
        height: 3f64,
    };

    assert_eq!(6f64, rectangle.area());
}
