// 策略接口
trait DrawStrategy {
    fn draw(&self);
}

// 具体策略：绘制矩形
struct RectangleStrategy;

impl DrawStrategy for RectangleStrategy {
    fn draw(&self) {
        println!("Drawing a rectangle");
    }
}

// 具体策略：绘制圆形
struct CircleStrategy;

impl DrawStrategy for CircleStrategy {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

// 具体策略：绘制三角形
struct TriangleStrategy;

impl DrawStrategy for TriangleStrategy {
    fn draw(&self) {
        println!("Drawing a triangle");
    }
}

// 上下文
struct DrawingApp {
    strategy: Box<dyn DrawStrategy>,
}

impl DrawingApp {
    fn new(strategy: Box<dyn DrawStrategy>) -> DrawingApp {
        DrawingApp { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn DrawStrategy>) {
        self.strategy = strategy;
    }

    fn draw(&self) {
        self.strategy.draw();
    }
}

fn main() {
    // 创建上下文对象，并设置初始策略为绘制矩形
    let mut app = DrawingApp::new(Box::new(RectangleStrategy));

    // 绘制图形
    app.draw();

    // 切换策略为绘制圆形
    app.set_strategy(Box::new(CircleStrategy));

    // 绘制图形
    app.draw();

    // 切换策略为绘制三角形
    app.set_strategy(Box::new(TriangleStrategy));

    // 绘制图形
    app.draw();
}
