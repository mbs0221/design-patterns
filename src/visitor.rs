// 定义访问者 trait
trait Visitor {
    fn visit_element_a(&self, element: &ElementA);
    fn visit_element_b(&self, element: &ElementB);
}

// 定义元素 trait
trait Element {
    fn accept(&self, visitor: &dyn Visitor);
}

// 具体元素 A
struct ElementA;

impl Element for ElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_a(self);
    }
}

// 具体元素 B
struct ElementB;

impl Element for ElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_element_b(self);
    }
}

// 具体访问者
struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_element_a(&self, element: &ElementA) {
        println!("Visitor is processing Element A");
    }

    fn visit_element_b(&self, element: &ElementB) {
        println!("Visitor is processing Element B");
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> = vec![
        Box::new(ElementA),
        Box::new(ElementB),
    ];

    let visitor = ConcreteVisitor;

    for element in elements {
        element.accept(&visitor);
    }
}
