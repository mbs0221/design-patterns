struct Event {
    event_type: String,
    data: String,
}

trait EventHandler {
    fn handle_event(&self, event: &Event);
}

struct EventProcessor {
    event_handlers: Vec<Box<dyn EventHandler>>,
}

impl EventProcessor {
    fn new() -> EventProcessor {
        EventProcessor {
            event_handlers: Vec::new(),
        }
    }

    fn add_event_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.event_handlers.push(handler);
    }

    fn process_event(&self, event: Event) {
        for handler in &self.event_handlers {
            handler.handle_event(&event);
        }
    }
}

struct LogEventHandler;

impl EventHandler for LogEventHandler {
    fn handle_event(&self, event: &Event) {
        println!("Logging event: {} - {}", event.event_type, event.data);
    }
}

struct EmailEventHandler;

impl EventHandler for EmailEventHandler {
    fn handle_event(&self, event: &Event) {
        println!("Sending email: {} - {}", event.event_type, event.data);
    }
}

fn main() {
    let mut event_processor = EventProcessor::new();
    
    let log_handler = Box::new(LogEventHandler);
    event_processor.add_event_handler(log_handler);

    let email_handler = Box::new(EmailEventHandler);
    event_processor.add_event_handler(email_handler);

    let event1 = Event {
        event_type: String::from("UserCreated"),
        data: String::from("User John created"),
    };
    event_processor.process_event(event1);

    let event2 = Event {
        event_type: String::from("OrderPlaced"),
        data: String::from("Order #123 placed"),
    };
    event_processor.process_event(event2);
}
