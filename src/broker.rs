struct Message {
    content: String,
}

impl Message {
    fn new(content: &str) -> Self {
        Message {
            content: content.to_string(),
        }
    }
}

struct Broker {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl Broker {
    fn new() -> Self {
        Broker {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    fn publish(&self, message: Message) {
        for subscriber in &self.subscribers {
            subscriber.receive(message.clone());
        }
    }
}

trait Subscriber {
    fn receive(&self, message: Message);
}

struct EmailSubscriber {
    email_address: String,
}

impl EmailSubscriber {
    fn new(email_address: &str) -> Self {
        EmailSubscriber {
            email_address: email_address.to_string(),
        }
    }
}

impl Subscriber for EmailSubscriber {
    fn receive(&self, message: Message) {
        println!("Sending email to {}: {}", self.email_address, message.content);
    }
}

struct SmsSubscriber {
    phone_number: String,
}

impl SmsSubscriber {
    fn new(phone_number: &str) -> Self {
        SmsSubscriber {
            phone_number: phone_number.to_string(),
        }
    }
}

impl Subscriber for SmsSubscriber {
    fn receive(&self, message: Message) {
        println!("Sending SMS to {}: {}", self.phone_number, message.content);
    }
}

fn main() {
    let mut broker = Broker::new();

    let email_subscriber = Box::new(EmailSubscriber::new("example@example.com"));
    let sms_subscriber = Box::new(SmsSubscriber::new("+1234567890"));

    broker.subscribe(email_subscriber);
    broker.subscribe(sms_subscriber);

    let message = Message::new("Hello, world!");
    broker.publish(message);
}
