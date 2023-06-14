// 迭代器 trait
trait Iterator {
    fn has_next(&mut self) -> bool;
    fn next(&mut self) -> Option<i32>;
}

// 具体迭代器
struct ConcreteIterator {
    current: i32,
    max: i32,
}

impl ConcreteIterator {
    fn new(max: i32) -> ConcreteIterator {
        ConcreteIterator { current: 0, max }
    }
}

impl Iterator for ConcreteIterator {
    fn has_next(&mut self) -> bool {
        self.current < self.max
    }

    fn next(&mut self) -> Option<i32> {
        if self.has_next() {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

// 客户端
fn main() {
    let mut iterator = ConcreteIterator::new(5);

    while iterator.has_next() {
        if let Some(value) = iterator.next() {
            println!("Next value: {}", value);
        }
    }
}
