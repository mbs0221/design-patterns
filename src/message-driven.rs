use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

// 定义消息的枚举类型
enum Message {
    Greeting(String),
    Increment(i32),
    Decrement(i32),
    Quit,
}

// 定义一个处理消息的函数
fn process_messages(receiver: Receiver<Message>) {
    loop {
        // 接收消息
        let message = receiver.recv().unwrap();

        // 处理不同类型的消息
        match message {
            Message::Greeting(name) => println!("Hello, {}!", name),
            Message::Increment(value) => println!("Increment: {}", value),
            Message::Decrement(value) => println!("Decrement: {}", value),
            Message::Quit => {
                println!("Quitting...");
                break;
            }
        }
    }
}

fn main() {
    // 创建消息通道
    let (sender, receiver) = mpsc::channel();

    // 启动处理消息的线程
    let handle = thread::spawn(move || {
        process_messages(receiver);
    });

    // 发送不同类型的消息
    sender.send(Message::Greeting("Alice".to_string())).unwrap();
    sender.send(Message::Increment(10)).unwrap();
    sender.send(Message::Decrement(5)).unwrap();
    sender.send(Message::Quit).unwrap();

    // 等待处理消息的线程结束
    handle.join().unwrap();
}
