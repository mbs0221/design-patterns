// 定义观察者 trait
trait Observer {
    fn update(&self, data: &str);
}

// 定义可观察者 trait
trait Observable {
    fn add_observer(&mut self, observer: Box<dyn Observer>);
    fn remove_observer(&mut self, observer: Box<dyn Observer>);
    fn notify_observers(&self, data: &str);
}

// 具体观察者实现
struct ConcreteObserver {
    name: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, data: &str) {
        println!("{} received data: {}", self.name, data);
    }
}

// 具体可观察者实现
struct ConcreteObservable {
    observers: Vec<Box<dyn Observer>>,
}

impl Observable for ConcreteObservable {
    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.retain(|o| !Arc::ptr_eq(&o, &observer));
    }

    fn notify_observers(&self, data: &str) {
        for observer in &self.observers {
            observer.update(data);
        }
    }
}

fn main() {
    let mut observable = ConcreteObservable {
        observers: Vec::new(),
    };

    let observer1 = Box::new(ConcreteObserver {
        name: "Observer 1".to_string(),
    });
    observable.add_observer(observer1);

    let observer2 = Box::new(ConcreteObserver {
        name: "Observer 2".to_string(),
    });
    observable.add_observer(observer2);

    observable.notify_observers("Data 1");
    observable.notify_observers("Data 2");
}
