// 备忘录
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Memento {
        Memento { state }
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }
}

// 发起人
struct Originator {
    state: String,
}

impl Originator {
    fn new(state: String) -> Originator {
        Originator { state }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn create_memento(&self) -> Memento {
        Memento::new(self.state.clone())
    }

    fn restore_from_memento(&mut self, memento: &Memento) {
        self.state = memento.get_state();
    }
}

// 管理者
struct Caretaker {
    mementos: Vec<Memento>,
}

impl Caretaker {
    fn new() -> Caretaker {
        Caretaker { mementos: vec![] }
    }

    fn add_memento(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    fn get_memento(&self, index: usize) -> Option<&Memento> {
        self.mementos.get(index)
    }
}

// 客户端
fn main() {
    let mut originator = Originator::new("State 1".to_string());
    let mut caretaker = Caretaker::new();

    println!("Initial state: {}", originator.state);

    // 保存备忘录
    caretaker.add_memento(originator.create_memento());

    // 修改状态
    originator.set_state("State 2".to_string());

    println!("Modified state: {}", originator.state);

    // 恢复状态
    if let Some(memento) = caretaker.get_memento(0) {
        originator.restore_from_memento(memento);
        println!("Restored state: {}", originator.state);
    }
}
