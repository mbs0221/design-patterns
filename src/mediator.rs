// 中介者 trait
trait Mediator {
    fn send(&self, message: &str, sender: &Box<dyn Colleague>);
}

// 具体中介者
struct ConcreteMediator {
    colleague1: Option<Box<dyn Colleague>>,
    colleague2: Option<Box<dyn Colleague>>,
}

impl Mediator for ConcreteMediator {
    fn send(&self, message: &str, sender: &Box<dyn Colleague>) {
        if let Some(colleague1) = &self.colleague1 {
            if sender as *const _ != colleague1.as_ref() as *const _ {
                colleague1.receive(message);
            }
        }

        if let Some(colleague2) = &self.colleague2 {
            if sender as *const _ != colleague2.as_ref() as *const _ {
                colleague2.receive(message);
            }
        }
    }
}

// 同事 trait
trait Colleague {
    fn set_mediator(&mut self, mediator: Box<dyn Mediator>);
    fn send(&self, message: &str);
    fn receive(&self, message: &str);
}

// 具体同事 A
struct ConcreteColleagueA {
    mediator: Option<Box<dyn Mediator>>,
}

impl Colleague for ConcreteColleagueA {
    fn set_mediator(&mut self, mediator: Box<dyn Mediator>) {
        self.mediator = Some(mediator);
    }

    fn send(&self, message: &str) {
        if let Some(ref mediator) = self.mediator {
            mediator.send(message, &Box::new(*self));
        }
    }

    fn receive(&self, message: &str) {
        println!("Colleague A received: {}", message);
    }
}

// 具体同事 B
struct ConcreteColleagueB {
    mediator: Option<Box<dyn Mediator>>,
}

impl Colleague for ConcreteColleagueB {
    fn set_mediator(&mut self, mediator: Box<dyn Mediator>) {
        self.mediator = Some(mediator);
    }

    fn send(&self, message: &str) {
        if let Some(ref mediator) = self.mediator {
            mediator.send(message, &Box::new(*self));
        }
    }

    fn receive(&self, message: &str) {
        println!("Colleague B received: {}", message);
    }
}

// 客户端
fn main() {
    let mediator = Box::new(ConcreteMediator {
        colleague1: None,
        colleague2: None,
    });

    let mut colleague1 = Box::new(ConcreteColleagueA { mediator: None });
    let mut colleague2 = Box::new(ConcreteColleagueB { mediator: None });

    colleague1.set_mediator(mediator.clone());
    colleague2.set_mediator(mediator.clone());

    mediator.colleague1 = Some(colleague1);
    mediator.colleague2 = Some(colleague2);

    mediator.send("Hello from Colleague A");
    mediator.send("Hello from Colleague B");
}
