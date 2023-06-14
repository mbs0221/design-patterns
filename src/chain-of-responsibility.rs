// 请求结构体
struct Request {
    amount: u32,
    purpose: String,
}

// 处理器接口
trait Handler {
    fn set_successor(&mut self, successor: Box<dyn Handler>);
    fn handle_request(&self, request: &Request);
}

// 具体处理器A
struct ConcreteHandlerA {
    successor: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandlerA {
    fn set_successor(&mut self, successor: Box<dyn Handler>) {
        self.successor = Some(successor);
    }

    fn handle_request(&self, request: &Request) {
        if request.amount <= 100 {
            println!("ConcreteHandlerA handles request: {:?}", request);
        } else if let Some(successor) = &self.successor {
            successor.handle_request(request);
        }
    }
}

// 具体处理器B
struct ConcreteHandlerB {
    successor: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandlerB {
    fn set_successor(&mut self, successor: Box<dyn Handler>) {
        self.successor = Some(successor);
    }

    fn handle_request(&self, request: &Request) {
        if request.amount > 100 && request.amount <= 500 {
            println!("ConcreteHandlerB handles request: {:?}", request);
        } else if let Some(successor) = &self.successor {
            successor.handle_request(request);
        }
    }
}

// 具体处理器C
struct ConcreteHandlerC {
    successor: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandlerC {
    fn set_successor(&mut self, successor: Box<dyn Handler>) {
        self.successor = Some(successor);
    }

    fn handle_request(&self, request: &Request) {
        if request.amount > 500 {
            println!("ConcreteHandlerC handles request: {:?}", request);
        } else if let Some(successor) = &self.successor {
            successor.handle_request(request);
        }
    }
}

fn main() {
    // 创建处理器实例
    let mut handler_a = ConcreteHandlerA {
        successor: None,
    };
    let mut handler_b = ConcreteHandlerB {
        successor: None,
    };
    let mut handler_c = ConcreteHandlerC {
        successor: None,
    };

    // 设置处理器之间的后继关系
    handler_a.set_successor(Box::new(handler_b));
    handler_b.set_successor(Box::new(handler_c));

    // 创建请求
    let request1 = Request {
        amount: 50,
        purpose: "Expense reimbursement".to_string(),
    };
    let request2 = Request {
        amount: 200,
        purpose: "Project funding".to_string(),
    };
    let request3 = Request {
        amount: 1000,
        purpose: "Capital expenditure".to_string(),
    };

    // 处理请求
    handler_a.handle_request(&request1);
    handler_a.handle_request(&request2);
    handler_a.handle_request(&request3);
}
