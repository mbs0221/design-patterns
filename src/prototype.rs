// 原型 trait
trait Cloneable {
    fn clone(&self) -> Box<dyn Cloneable>;
}

// 具体原型 A
#[derive(Clone)]
struct ConcretePrototypeA {
    name: String,
}

impl Cloneable for ConcretePrototypeA {
    fn clone(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

// 具体原型 B
#[derive(Clone)]
struct ConcretePrototypeB {
    number: i32,
}

impl Cloneable for ConcretePrototypeB {
    fn clone(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

// 客户端
fn main() {
    let prototype_a = ConcretePrototypeA {
        name: "Prototype A".to_string(),
    };
    let cloned_a = prototype_a.clone();

    let prototype_b = ConcretePrototypeB { number: 42 };
    let cloned_b = prototype_b.clone();

    println!("Cloned A: {:?}", cloned_a);
    println!("Cloned B: {:?}", cloned_b);
}
