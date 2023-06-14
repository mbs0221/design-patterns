use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

type EventCallback = Box<dyn Fn() + Send + Sync>;

struct EventBus {
    subscriptions: HashMap<String, Vec<EventCallback>>,
}

impl EventBus {
    fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    fn subscribe(&mut self, event_name: &str, callback: EventCallback) {
        self.subscriptions
            .entry(event_name.to_string())
            .or_insert(Vec::new())
            .push(callback);
    }

    fn publish(&self, event_name: &str) {
        if let Some(subscribers) = self.subscriptions.get(event_name) {
            let cloned_subscribers: Vec<_> = subscribers.clone();

            // 创建一个互斥锁，用于保证多线程访问的安全性
            let mutex = Arc::new(Mutex::new(()));

            // 启动一个线程池来并发执行订阅者的回调函数
            let handles: Vec<_> = cloned_subscribers
                .into_iter()
                .map(|subscriber| {
                    let mutex = Arc::clone(&mutex);

                    thread::spawn(move || {
                        // 获取互斥锁，并执行回调函数
                        let _lock = mutex.lock().unwrap();
                        subscriber();
                    })
                })
                .collect();

            // 等待所有线程执行完成
            for handle in handles {
                handle.join().unwrap();
            }
        }
    }
}

fn main() {
    let mut event_bus = EventBus::new();

    event_bus.subscribe("event1", Box::new(|| {
        println!("Event 1 occurred");
    }));

    event_bus.subscribe("event2", Box::new(|| {
        println!("Event 2 occurred");
    }));

    event_bus.publish("event1");
    event_bus.publish("event2");
}
