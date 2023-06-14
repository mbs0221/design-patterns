struct Product {
    part_a: String,
    part_b: String,
    part_c: String,
}

impl Product {
    fn new() -> Product {
        Product {
            part_a: String::new(),
            part_b: String::new(),
            part_c: String::new(),
        }
    }

    fn set_part_a(&mut self, part_a: String) {
        self.part_a = part_a;
    }

    fn set_part_b(&mut self, part_b: String) {
        self.part_b = part_b;
    }

    fn set_part_c(&mut self, part_c: String) {
        self.part_c = part_c;
    }

    fn display(&self) {
        println!("Part A: {}", self.part_a);
        println!("Part B: {}", self.part_b);
        println!("Part C: {}", self.part_c);
    }
}

trait Builder {
    fn build_part_a(&mut self);
    fn build_part_b(&mut self);
    fn build_part_c(&mut self);
    fn get_product(&self) -> &Product;
}

struct ConcreteBuilderA {
    product: Product,
}

impl ConcreteBuilderA {
    fn new() -> ConcreteBuilderA {
        ConcreteBuilderA {
            product: Product::new(),
        }
    }
}

impl Builder for ConcreteBuilderA {
    fn build_part_a(&mut self) {
        self.product.set_part_a("Part A for Builder A".to_string());
    }

    fn build_part_b(&mut self) {
        self.product.set_part_b("Part B for Builder A".to_string());
    }

    fn build_part_c(&mut self) {
        self.product.set_part_c("Part C for Builder A".to_string());
    }

    fn get_product(&self) -> &Product {
        &self.product
    }
}

struct ConcreteBuilderB {
    product: Product,
}

impl ConcreteBuilderB {
    fn new() -> ConcreteBuilderB {
        ConcreteBuilderB {
            product: Product::new(),
        }
    }
}

impl Builder for ConcreteBuilderB {
    fn build_part_a(&mut self) {
        self.product.set_part_a("Part A for Builder B".to_string());
    }

    fn build_part_b(&mut self) {
        self.product.set_part_b("Part B for Builder B".to_string());
    }

    fn build_part_c(&mut self) {
        self.product.set_part_c("Part C for Builder B".to_string());
    }

    fn get_product(&self) -> &Product {
        &self.product
    }
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder }
    }

    fn construct_product(&mut self) {
        self.builder.build_part_a();
        self.builder.build_part_b();
        self.builder.build_part_c();
    }

    fn get_product(&self) -> &Product {
        self.builder.get_product()
    }
}

fn main() {
    let builder_a = Box::new(ConcreteBuilderA::new());
    let mut director_a = Director::new(builder_a);
    director_a.construct_product();
    let product_a = director_a.get_product();
    product_a.display();

    let builder_b = Box::new(ConcreteBuilderB::new());
    let mut director_b = Director::new(builder_b);
    director_b.construct_product();
    let product_b = director_b.get_product();
    product_b.display();
}
