// 单例对象
struct Singleton {
    name: String,
}

impl Singleton {
    fn new() -> Singleton {
        Singleton {
            name: "Singleton Instance".to_string(),
        }
    }
}

// 客户端
fn main() {
    let singleton1 = Singleton::new();
    let singleton2 = Singleton::new();

    println!("Singleton 1: {:?}", singleton1);
    println!("Singleton 2: {:?}", singleton2);
}
