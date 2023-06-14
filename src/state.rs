// 状态 trait
trait State {
    fn handle(&self, context: &mut Context);
}

// 具体状态 A
struct ConcreteStateA;

impl State for ConcreteStateA {
    fn handle(&self, context: &mut Context) {
        println!("State A: Handling the request");
        context.set_state(Box::new(ConcreteStateB));
    }
}

// 具体状态 B
struct ConcreteStateB;

impl State for ConcreteStateB {
    fn handle(&self, context: &mut Context) {
        println!("State B: Handling the request");
        context.set_state(Box::new(ConcreteStateA));
    }
}

// 上下文
struct Context {
    state: Box<dyn State>,
}

impl Context {
    fn new() -> Context {
        Context {
            state: Box::new(ConcreteStateA),
        }
    }

    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    fn request(&mut self) {
        self.state.handle(self);
    }
}

// 客户端
fn main() {
    let mut context = Context::new();

    context.request();
    context.request();
    context.request();
}
