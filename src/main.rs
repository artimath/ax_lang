use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    Int(i64),
    Bool(bool),
    Vector(Vec<Value>),
    Function(String, Vec<String>, Box<Expr>),
}

#[derive(Clone, Debug)]
enum Expr {
    Literal(Value),
    Variable(String),
    Application(Box<Expr>, Vec<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

struct Interpreter {
    env: HashMap<String, Value>,
}

impl Interpreter {
    fn new() -> Self {
        let mut env = HashMap::new();
        env.insert("+".to_string(), Value::Function(
            "+".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expr::Variable("x".to_string())) // This is a placeholder
        ));
        Interpreter { env }
    }

    fn eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Variable(name) => self.env.get(name).cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expr::Application(func, args) => {
                let func_val = self.eval(func)?;
                match func_val {
                    Value::Function(name, params, body) => {
                        if name == "+" {
                            // Handle built-in addition
                            if args.len() != 2 {
                                return Err("add takes 2 arguments".to_string());
                            }
                            let x = self.eval(&args[0])?;
                            let y = self.eval(&args[1])?;
                            match (x, y) {
                                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                                _ => Err("add takes two integers".to_string()),
                            }
                        } else {
                            if params.len() != args.len() {
                                return Err("Wrong number of arguments".to_string());
                            }
                            let mut new_env = self.env.clone();
                            for (param, arg) in params.iter().zip(args) {
                                new_env.insert(param.clone(), self.eval(arg)?);
                            }
                            let mut new_interpreter = Interpreter { env: new_env };
                            new_interpreter.eval(&body)
                        }
                    },
                    _ => Err("Not a function".to_string()),
                }
            },
            Expr::Let(name, value_expr, body) => {
                let value = self.eval(value_expr)?;
                self.env.insert(name.clone(), value);
                self.eval(body)
            },
            Expr::If(cond, then_expr, else_expr) => {
                match self.eval(cond)? {
                    Value::Bool(true) => self.eval(then_expr),
                    Value::Bool(false) => self.eval(else_expr),
                    _ => Err("Condition must be a boolean".to_string()),
                }
            },
        }
    }

    fn builtin_add(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("add takes 2 arguments".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
            _ => Err("add takes two integers".to_string()),
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();

    // Define double function
    let double_func = Expr::Application(
        Box::new(Expr::Variable("+".to_string())),
        vec![Expr::Variable("x".to_string()), Expr::Variable("x".to_string())]
    );
    interpreter.env.insert("double".to_string(), Value::Function(
        "double".to_string(),
        vec!["x".to_string()],
        Box::new(double_func)
    ));

    // Create a vector [1, 2, 3]
    let vector = Expr::Literal(Value::Vector(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
    ]));

    // Apply double to each element (simplified, as we don't have full map implementation)
    let result = Expr::Application(
        Box::new(Expr::Variable("double".to_string())),
        vec![Expr::Literal(Value::Int(1))]
    );

    match interpreter.eval(&result) {
        Ok(Value::Int(n)) => println!("Result: {}", n),
        Ok(_) => println!("Unexpected result type"),
        Err(e) => println!("Error: {}", e),
    }
}