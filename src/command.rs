// 命令 trait
trait Command {
    fn execute(&self);
}

// 具体命令 A
struct ConcreteCommandA {
    receiver: Receiver,
}

impl Command for ConcreteCommandA {
    fn execute(&self) {
        self.receiver.action_a();
    }
}

// 具体命令 B
struct ConcreteCommandB {
    receiver: Receiver,
}

impl Command for ConcreteCommandB {
    fn execute(&self) {
        self.receiver.action_b();
    }
}

// 接收者
struct Receiver;

impl Receiver {
    fn action_a(&self) {
        println!("Receiver: Performing action A");
    }

    fn action_b(&self) {
        println!("Receiver: Performing action B");
    }
}

// 客户端
fn main() {
    let receiver = Receiver;
    let command_a = ConcreteCommandA { receiver: receiver.clone() };
    let command_b = ConcreteCommandB { receiver: receiver.clone() };

    command_a.execute();
    command_b.execute();
}
