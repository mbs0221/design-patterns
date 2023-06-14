// 共享接口
trait Shape {
    fn draw(&self);
}

// 具体组件
struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

// 装饰器基类
struct ShapeDecorator {
    shape: Box<dyn Shape>,
}

impl Shape for ShapeDecorator {
    fn draw(&self) {
        self.shape.draw();
    }
}

// 具体装饰器类
struct RedShapeDecorator {
    shape: Box<dyn Shape>,
}

impl RedShapeDecorator {
    fn new(shape: Box<dyn Shape>) -> RedShapeDecorator {
        RedShapeDecorator { shape }
    }

    fn set_red_border(&self) {
        println!("Setting red border");
    }
}

impl Shape for RedShapeDecorator {
    fn draw(&self) {
        self.shape.draw();
        self.set_red_border();
    }
}

fn main() {
    // 创建具体组件
    let circle = Circle;

    // 创建装饰器对象，并传入具体组件
    let decorated_circle = RedShapeDecorator::new(Box::new(circle));

    // 调用装饰器对象的方法，实现装饰效果
    decorated_circle.draw();
}
