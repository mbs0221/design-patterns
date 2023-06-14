// 实现部分的接口
trait DrawingAPI {
    fn draw_circle(&self, x: i32, y: i32, radius: i32);
}

// 具体实现部分A
struct DrawingAPIA;

impl DrawingAPI for DrawingAPIA {
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        println!("Drawing circle at ({}, {}) with radius {} using API A", x, y, radius);
    }
}

// 具体实现部分B
struct DrawingAPIB;

impl DrawingAPI for DrawingAPIB {
    fn draw_circle(&self, x: i32, y: i32, radius: i32) {
        println!("Drawing circle at ({}, {}) with radius {} using API B", x, y, radius);
    }
}

// 抽象部分
trait Shape {
    fn draw(&self);
    fn resize(&mut self, radius: i32);
}

// 具体抽象部分：圆形
struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    drawing_api: Box<dyn DrawingAPI>,
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32, drawing_api: Box<dyn DrawingAPI>) -> Circle {
        Circle {
            x,
            y,
            radius,
            drawing_api,
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.drawing_api.draw_circle(self.x, self.y, self.radius);
    }

    fn resize(&mut self, radius: i32) {
        self.radius = radius;
    }
}

// 客户端
fn main() {
    let circle1 = Circle::new(1, 2, 3, Box::new(DrawingAPIA));
    let circle2 = Circle::new(4, 5, 6, Box::new(DrawingAPIB));

    circle1.draw();
    circle2.draw();
}
