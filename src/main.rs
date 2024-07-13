use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
enum Value {
    Int(i64),
    Bool(bool),
    Vector(Vec<Value>),
    Function(String, Vec<String>, Box<Expr>),
    Future(Arc<Mutex<Option<Value>>>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Vector(a), Value::Vector(b)) => a == b,
            (Value::Function(a1, a2, a3), Value::Function(b1, b2, b3)) => {
                a1 == b1 && a2 == b2 && a3 == b3
            }
            (Value::Future(_), Value::Future(_)) => {
                // We can't compare futures, so we'll consider them always unequal
                false
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Literal(Value),
    Variable(String),
    Application(Box<Expr>, Vec<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Spawn(Box<Expr>),
    Sync(Box<Expr>),
}

struct Interpreter {
    env: HashMap<String, Value>,
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            env: self.env.clone()
        }
    }
}

impl Interpreter {
    fn new() -> Self {
        let mut env = HashMap::new();
        
        // Addition
        env.insert("+".to_string(), Value::Function(
            "+".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expr::Variable("x".to_string()))
        ));

        // Subtraction
        env.insert("-".to_string(), Value::Function(
            "-".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expr::Variable("x".to_string()))
        ));

        // Equality
        env.insert("=".to_string(), Value::Function(
            "=".to_string(),
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expr::Variable("x".to_string()))
        ));

        // If
        env.insert("if".to_string(), Value::Function(
            "if".to_string(),
            vec!["condition".to_string(), "then".to_string(), "else".to_string()],
            Box::new(Expr::Variable("condition".to_string()))
        ));

        // Cons
        env.insert("cons".to_string(), Value::Function(
            "cons".to_string(),
            vec!["head".to_string(), "tail".to_string()],
            Box::new(Expr::Variable("head".to_string()))
        ));

        // Head
        env.insert("head".to_string(), Value::Function(
            "head".to_string(),
            vec!["list".to_string()],
            Box::new(Expr::Variable("list".to_string()))
        ));

        // Tail
        env.insert("tail".to_string(), Value::Function(
            "tail".to_string(),
            vec!["list".to_string()],
            Box::new(Expr::Variable("list".to_string()))
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
                        match name.as_str() {
                            "+" => self.builtin_add(args),
                            "-" => self.builtin_sub(args),
                            "=" => self.builtin_eq(args),
                            "if" => self.builtin_if(args),
                            "cons" => self.builtin_cons(args),
                            "head" => self.builtin_head(args),
                            "tail" => self.builtin_tail(args),
                            _ => {
                                if name == "+" {
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
                            }
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
            Expr::Spawn(expr) => {
                let future = Arc::new(Mutex::new(None));
                let future_clone = future.clone();
                let expr_clone = expr.clone(); // Clone the expression
                let interpreter = self.clone();
                thread::scope(|s| {
                    s.spawn(move || {
                        let mut interpreter = interpreter;
                        let result = interpreter.eval(&expr_clone); // Use the cloned expression
                        *future_clone.lock().unwrap() = Some(result.unwrap());
                    });
                });
                Ok(Value::Future(future))
            },
            Expr::Sync(expr) => {
                match self.eval(expr)? {
                    Value::Future(future) => {
                        let result = future.lock().unwrap().take().unwrap();
                        Ok(result)
                    },
                    _ => Err("Cannot sync non-future value".to_string()),
                }
            },
        }
    }

    fn builtin_add(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("add takes 2 arguments".to_string());
        }
        let x = self.eval(&args[0])?;
        let y = self.eval(&args[1])?;
        match (x, y) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            _ => Err("add takes two integers".to_string()),
        }
    }

    fn builtin_sub(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("subtract takes 2 arguments".to_string());
        }
        let x = self.eval(&args[0])?;
        let y = self.eval(&args[1])?;
        match (x, y) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            _ => Err("subtract takes two integers".to_string()),
        }
    }

    fn builtin_eq(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("eq takes 2 arguments".to_string());
        }
        let x = self.eval(&args[0])?;
        let y = self.eval(&args[1])?;
        Ok(Value::Bool(x == y))
    }

    fn builtin_if(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("if takes 3 arguments".to_string());
        }
        let condition = self.eval(&args[0])?;
        match condition {
            Value::Bool(true) => self.eval(&args[1]),
            Value::Bool(false) => self.eval(&args[2]),
            _ => Err("if condition must be a boolean".to_string()),
        }
    }

    fn builtin_cons(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("cons takes 2 arguments".to_string());
        }
        let head = self.eval(&args[0])?;
        let tail = self.eval(&args[1])?;
        match tail {
            Value::Vector(mut v) => {
                let mut new_vec = vec![head];
                new_vec.append(&mut v);
                Ok(Value::Vector(new_vec))
            },
            _ => Err("cons second argument must be a vector".to_string()),
        }
    }

    fn builtin_head(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("head takes 1 argument".to_string());
        }
        let list = self.eval(&args[0])?;
        match list {
            Value::Vector(v) => v.first().cloned().ok_or("empty vector".to_string()),
            _ => Err("head argument must be a vector".to_string()),
        }
    }

    fn builtin_tail(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("tail takes 1 argument".to_string());
        }
        let list = self.eval(&args[0])?;
        match list {
            Value::Vector(v) => {
                if v.is_empty() {
                    Err("empty vector".to_string())
                } else {
                    Ok(Value::Vector(v[1..].to_vec()))
                }
            },
            _ => Err("tail argument must be a vector".to_string()),
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new();

    // Define recursive_pmap function
    let recursive_pmap_func = Expr::Application(
        Box::new(Expr::Variable("if".to_string())),
        vec![
            Expr::Application(
                Box::new(Expr::Variable("=".to_string())),
                vec![Expr::Variable("n".to_string()), Expr::Literal(Value::Int(0))]
            ),
            Expr::Literal(Value::Vector(vec![])),
            Expr::Application(
                Box::new(Expr::Variable("cons".to_string())),
                vec![
                    Expr::Application(
                        Box::new(Expr::Variable("f".to_string())),
                        vec![Expr::Application(
                            Box::new(Expr::Variable("head".to_string())),
                            vec![Expr::Variable("v".to_string())]
                        )]
                    ),
                    Expr::Application(
                        Box::new(Expr::Variable("recursive_pmap".to_string())),
                        vec![
                            Expr::Variable("A".to_string()),
                            Expr::Variable("B".to_string()),
                            Expr::Application(
                                Box::new(Expr::Variable("-".to_string())),
                                vec![Expr::Variable("n".to_string()), Expr::Literal(Value::Int(1))]
                            ),
                            Expr::Variable("f".to_string()),
                            Expr::Application(
                                Box::new(Expr::Variable("tail".to_string())),
                                vec![Expr::Variable("v".to_string())]
                            )
                        ]
                    )
                ]
            )
        ]
    );
    interpreter.env.insert("recursive_pmap".to_string(), Value::Function(
        "recursive_pmap".to_string(),
        vec!["A".to_string(), "B".to_string(), "n".to_string(), "f".to_string(), "v".to_string()],
        Box::new(recursive_pmap_func)
    ));

    // Define pmap function
    let pmap_func = Expr::Application(
        Box::new(Expr::Variable("recursive_pmap".to_string())),
        vec![
            Expr::Variable("A".to_string()),
            Expr::Variable("B".to_string()),
            Expr::Variable("n".to_string()),
            Expr::Variable("f".to_string()),
            Expr::Variable("v".to_string()),
        ]
    );
    interpreter.env.insert("pmap".to_string(), Value::Function(
        "pmap".to_string(),
        vec!["A".to_string(), "B".to_string(), "n".to_string(), "f".to_string(), "v".to_string()],
        Box::new(pmap_func)
    ));

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

    // Create a vector [1, 2, 3, 4]
    let vector = Expr::Literal(Value::Vector(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
    ]));

    // Apply pmap with double to the vector
    let result = Expr::Application(
        Box::new(Expr::Variable("pmap".to_string())),
        vec![
            Expr::Literal(Value::Function("".to_string(), vec![], Box::new(Expr::Literal(Value::Int(0))))), // A (placeholder)
            Expr::Literal(Value::Function("".to_string(), vec![], Box::new(Expr::Literal(Value::Int(0))))), // B (placeholder)
            Expr::Literal(Value::Int(4)), // n
            Expr::Variable("double".to_string()),
            vector,
        ]
    );

    match interpreter.eval(&result) {
        Ok(Value::Vector(v)) => {
            println!("Result: {:?}", v);
            println!("Length: {}", v.len());
        },
        Ok(_) => println!("Unexpected result type"),
        Err(e) => println!("Error: {}", e),
    }
}