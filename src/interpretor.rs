// 抽象表达式 trait
trait Expression {
    fn interpret(&self) -> i32;
}

// 数字表达式
struct NumberExpression {
    value: i32,
}

impl Expression for NumberExpression {
    fn interpret(&self) -> i32 {
        self.value
    }
}

// 加法表达式
struct AddExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for AddExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}

// 减法表达式
struct SubtractExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for SubtractExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() - self.right.interpret()
    }
}

// 解释器
struct Interpreter {
    syntax_tree: Box<dyn Expression>,
}

impl Interpreter {
    fn new(syntax_tree: Box<dyn Expression>) -> Interpreter {
        Interpreter { syntax_tree }
    }

    fn interpret(&self) -> i32 {
        self.syntax_tree.interpret()
    }
}

fn main() {
    // 构建语法树：2 + (3 - 1)
    let syntax_tree = Box::new(
        AddExpression {
            left: Box::new(NumberExpression { value: 2 }),
            right: Box::new(
                SubtractExpression {
                    left: Box::new(NumberExpression { value: 3 }),
                    right: Box::new(NumberExpression { value: 1 }),
                }
            ),
        }
    );

    let interpreter = Interpreter::new(syntax_tree);
    let result = interpreter.interpret();
    println!("Result: {}", result); // 输出：Result: 4
}
