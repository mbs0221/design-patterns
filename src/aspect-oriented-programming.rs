// 定义切面
struct LoggingAspect;

impl LoggingAspect {
    fn before(target: &str, method: &str) {
        println!("Before calling {}.{}", target, method);
    }

    fn after(target: &str, method: &str) {
        println!("After calling {}.{}", target, method);
    }
}

// 定义目标类
struct UserService;

impl UserService {
    fn save(&self, user: &str) {
        println!("Saving user: {}", user);
    }
}

// 定义代理类
struct UserServiceProxy {
    target: UserService,
}

impl UserServiceProxy {
    fn new() -> UserServiceProxy {
        UserServiceProxy { target: UserService }
    }

    fn save(&self, user: &str) {
        LoggingAspect::before("UserService", "save");
        self.target.save(user);
        LoggingAspect::after("UserService", "save");
    }
}

// 客户端代码
fn main() {
    let proxy = UserServiceProxy::new();
    proxy.save("John Doe");
}
