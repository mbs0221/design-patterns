// 子系统 A
struct SubSystemA;

impl SubSystemA {
    fn operation_a(&self) {
        println!("SubSystemA: Operation A");
    }
}

// 子系统 B
struct SubSystemB;

impl SubSystemB {
    fn operation_b(&self) {
        println!("SubSystemB: Operation B");
    }
}

// 子系统 C
struct SubSystemC;

impl SubSystemC {
    fn operation_c(&self) {
        println!("SubSystemC: Operation C");
    }
}

// 外观
struct Facade {
    subsystem_a: SubSystemA,
    subsystem_b: SubSystemB,
    subsystem_c: SubSystemC,
}

impl Facade {
    fn new() -> Facade {
        Facade {
            subsystem_a: SubSystemA,
            subsystem_b: SubSystemB,
            subsystem_c: SubSystemC,
        }
    }

    fn perform_operations(&self) {
        println!("Facade: Starting operations");
        self.subsystem_a.operation_a();
        self.subsystem_b.operation_b();
        self.subsystem_c.operation_c();
        println!("Facade: Operations completed");
    }
}

fn main() {
    let facade = Facade::new();
    facade.perform_operations();
}
