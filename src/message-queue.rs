use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

enum Message {
    Task(String),
    Quit,
}

fn producer(sender: Sender<Message>) {
    let tasks = vec![
        "Task 1".to_string(),
        "Task 2".to_string(),
        "Task 3".to_string(),
        "Task 4".to_string(),
    ];

    for task in tasks {
        sender.send(Message::Task(task)).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
    }

    sender.send(Message::Quit).unwrap();
}

fn consumer_1(receiver: Receiver<Message>) {
    loop {
        let message = receiver.recv().unwrap();
        match message {
            Message::Task(task) => println!("Consumer 1 processing task: {}", task),
            Message::Quit => {
                println!("Consumer 1 quitting...");
                break;
            }
        }
    }
}

fn consumer_2(receiver: Receiver<Message>) {
    loop {
        let message = receiver.recv().unwrap();
        match message {
            Message::Task(task) => println!("Consumer 2 processing task: {}", task),
            Message::Quit => {
                println!("Consumer 2 quitting...");
                break;
            }
        }
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    let producer_handle = thread::spawn(move || {
        producer(sender);
    });

    let consumer_1_handle = thread::spawn(move || {
        consumer_1(receiver);
    });

    let consumer_2_handle = thread::spawn(move || {
        consumer_2(receiver);
    });

    producer_handle.join().unwrap();
    consumer_1_handle.join().unwrap();
    consumer_2_handle.join().unwrap();
}
