// 定义一个服务接口
trait Service {
    fn do_something(&self);
}

// 实现具体的服务
struct MyService;
impl Service for MyService {
    fn do_something(&self) {
        println!("Doing something in MyService");
    }
}

// 使用注入模式的组件
struct MyComponent {
    service: Box<dyn Service>,
}

impl MyComponent {
    // 使用依赖注入，在构造函数中传入具体的服务实例
    fn new(service: Box<dyn Service>) -> Self {
        MyComponent { service }
    }

    fn do_work(&self) {
        // 调用注入的服务
        self.service.do_something();
    }
}

fn main() {
    // 创建具体的服务实例
    let service = Box::new(MyService);

    // 创建使用注入模式的组件，并将服务实例注入
    let component = MyComponent::new(service);

    // 使用组件进行工作
    component.do_work();
}
