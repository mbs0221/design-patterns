// 目标接口
trait Target {
    fn request(&self);
}

// 适配者
struct Adaptee {
    name: String,
}

impl Adaptee {
    fn specific_request(&self) {
        println!("Adaptee: {}", self.name);
    }
}

// 适配器
struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Adapter {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) {
        self.adaptee.specific_request();
    }
}

// 客户端
fn main() {
    let adaptee = Adaptee {
        name: "Adaptee".to_string(),
    };
    let adapter = Adapter::new(adaptee);

    adapter.request();
}
