// 享元工厂
struct TreeFactory {
    tree_types: std::collections::HashMap<String, TreeType>,
}

impl TreeFactory {
    fn new() -> TreeFactory {
        TreeFactory {
            tree_types: std::collections::HashMap::new(),
        }
    }

    fn get_tree_type(&mut self, name: &str) -> &TreeType {
        if !self.tree_types.contains_key(name) {
            let tree_type = TreeType::new(name);
            self.tree_types.insert(name.to_string(), tree_type);
        }
        self.tree_types.get(name).unwrap()
    }
}

// 享元对象
struct Tree {
    x: i32,
    y: i32,
    tree_type: String,
}

impl Tree {
    fn new(x: i32, y: i32, tree_type: String) -> Tree {
        Tree {
            x,
            y,
            tree_type,
        }
    }

    fn draw(&self) {
        println!("Drawing tree at ({}, {}) of type: {}", self.x, self.y, self.tree_type);
    }
}

// 具体享元类型
struct TreeType {
    name: String,
    color: String,
    texture: String,
}

impl TreeType {
    fn new(name: &str) -> TreeType {
        TreeType {
            name: name.to_string(),
            color: "green".to_string(),
            texture: "oak".to_string(),
        }
    }
}

// 客户端
fn main() {
    let mut tree_factory = TreeFactory::new();

    let tree1 = Tree::new(1, 2, tree_factory.get_tree_type("oak").name.clone());
    let tree2 = Tree::new(3, 4, tree_factory.get_tree_type("maple").name.clone());
    let tree3 = Tree::new(5, 6, tree_factory.get_tree_type("oak").name.clone());

    tree1.draw();
    tree2.draw();
    tree3.draw();
}
