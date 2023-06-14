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

// 服务定位器
struct ServiceLocator {
    services: std::collections::HashMap<String, Box<dyn Service>>,
}

impl ServiceLocator {
    fn new() -> Self {
        ServiceLocator {
            services: std::collections::HashMap::new(),
        }
    }

    fn register_service<T: Service + 'static>(&mut self, name: &str, service: T) {
        self.services.insert(name.to_string(), Box::new(service));
    }

    fn get_service(&self, name: &str) -> Option<&dyn Service> {
        self.services.get(name).map(|s| s.as_ref())
    }
}

// 使用服务定位器的组件
struct MyComponent {
    service_locator: ServiceLocator,
}

impl MyComponent {
    fn new(service_locator: ServiceLocator) -> Self {
        MyComponent { service_locator }
    }

    fn do_work(&self) {
        // 通过服务定位器获取具体的服务实例
        if let Some(service) = self.service_locator.get_service("my_service") {
            service.do_something();
        } else {
            println!("Service not found");
        }
    }
}

fn main() {
    // 创建服务定位器并注册具体的服务
    let mut service_locator = ServiceLocator::new();
    service_locator.register_service("my_service", MyService);

    // 创建使用服务定位器的组件
    let component = MyComponent::new(service_locator);

    // 使用组件进行工作
    component.do_work();
}
