// Our core logic
use std::slice::Iter;


pub type Symbol = String;
pub type Expr = Vec<Symbol>;
// Everyone: oh my gosh rust doesn't have a stack, so useless
// Me, an intellectual:
type Stack<T> = Vec<T>;

#[derive(Debug,PartialEq)]
pub enum Order {
    Pre,
    Post
}

#[derive(Debug, PartialEq)]
pub enum ExprTree {
    Num (i32),
    Tree (char, Box<ExprTree>, Box<ExprTree>) // (operator, left, right)
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    InvalidSyntax(Order),
    InvalidSymbol(String),
    DividingByZero
}

pub fn eval(order : &Order, expr : Expr) -> Result<i32, EvalError> {
    match order {
        Order::Pre  => eval_pre(expr),
        Order::Post => eval_pos(expr)
    }
}

fn eval_pos(expr : Expr) -> Result<i32, EvalError>{

    let mut stack : Stack<i32> = Stack::new();

    for s in expr {
        if s == "/" || s == "*" || s == "+" || s == "-" {
            // Try to pop two arguments
            let r = match stack.pop() {
                None    => return Err(EvalError::InvalidSyntax(Order::Post)),
                Some(i) => i
            };

            let l = match stack.pop() {
                None    => return Err(EvalError::InvalidSyntax(Order::Post)),
                Some(i) => i
            };

            let opr = s.chars().next().unwrap();

            // check divide by zero
            if opr == '/' && r == 0 {return Err(EvalError::DividingByZero);}

            stack.push(eval_expr(opr, l, r));
            continue;
        }
        
        let i = match s.parse::<i32>() {
            Ok(i)  => i,
            Err(_) => return Err(EvalError::InvalidSymbol(s))
        };

        stack.push(i);
    }

    // If more than a single number in the stack, this expression was not
    if stack.len() != 1 { return Err(EvalError::InvalidSyntax(Order::Post)) }

    
    Ok(stack.pop().unwrap())
}

fn eval_pre(expr : Expr) -> Result<i32, EvalError> {
    fn eval_pre_aux(expr : &mut Iter<Symbol>) -> Result<i32, EvalError>{
    
        let opr = match expr.next() {
            None    => return Err(EvalError::InvalidSyntax(Order::Pre)),
            Some(o) => o
        };
    
        if opr == "/" || opr == "*" || opr == "+" || opr == "-" {

            let opr = opr.chars().next().unwrap();

            let l = match eval_pre_aux(expr) {
                Ok(i) => i,
                e     => return e
            };
            let r = match eval_pre_aux(expr) {
                Ok(i) => i,
                e     => return e
            };
            
            // check divide by zero
            if opr == '/' && r == 0 {return Err(EvalError::DividingByZero);}

            return Ok(eval_expr(opr, l, r))
        };
    
        // maybe our operator is a number instead?
        match opr.parse::<i32>() {
            Err(_) => Err(EvalError::InvalidSymbol(opr.to_string())),
            Ok(i)  => Ok(i)
        }
    }

    let mut expr = expr.iter();
    let out = eval_pre_aux(&mut expr);

    // if something went wrong, return the error
    if out.is_err() { return out }

    // if there's something else in expresion, that's not a valid expression
    if expr.next().is_some() { return Err(EvalError::InvalidSyntax(Order::Pre)) }
    
    out 
}

fn eval_expr(opr : char, left : i32, right : i32) -> i32 {
    match opr {
        '/' => left / right,
        '*' => left * right,
        '+' => left + right,
        '-' => left - right,
        _   => panic!("oh no, undefined operator")
    }
}

impl EvalError {
    pub fn print(&self){
        match self {
            EvalError::InvalidSymbol(s)   => eprintln!("🚨 This is not a valid symbol: {}", s),
            EvalError::InvalidSyntax(ord) => eprintln!("🚨 This is not a valid syntax for a {} order expression ", 
                match ord {Order::Post => "post", _ => "pre"}
            ),
            EvalError::DividingByZero => eprintln!("🤯 You can't divide by zer0 🤯")
        }
    }
}

impl ExprTree {
    pub fn parse(order : Order, expr : Expr) -> Result<ExprTree, EvalError> {
        match order {
            Order::Pre => ExprTree::parse_pre(expr),
            Order::Post=> ExprTree::parse_post(expr)
        }
    }

    pub fn print(&self) {
        fn print_aux(exp : &ExprTree) {
            match exp {
                ExprTree::Num(e) => print!("{}", e),
                ExprTree::Tree(c, t1, t2) => {

                    let curr_prec = ExprTree::get_prec(c);

                    let should_parenth_t1 = match **t1 {
                        ExprTree::Tree(c,_,_) => ExprTree::get_prec(&c) < curr_prec ,
                        _ => { false } 
                    };

                    let should_parenth_t2 = match **t2 {
                        ExprTree::Tree(c,_,_) => ExprTree::get_prec(&c) < curr_prec,
                        _ => { false } 
                    };

                    if should_parenth_t1 {
                        print!("( ");
                        print_aux(t1);
                        print!(" )");
                    }
                    else{
                        print_aux(t1);
                    }

                    print!(" {} ", c);

                    if should_parenth_t2 {
                        print!("( ");
                        print_aux(t2);
                        print!(" )");
                    }
                    else {
                        print_aux(t2);
                    }                    
                }
            }
        }
        print_aux(self);
        println!("");
    }

    fn get_prec (opr1 : &char) -> i32 {
        use std::collections::HashMap;

        let mut prec = HashMap::new();
        prec.insert('/', 2);
        prec.insert('*', 2);
        prec.insert('+', 1);
        prec.insert('-', 1);

        *prec.get(&opr1).unwrap()
    }

    fn parse_pre(expr : Expr) -> Result<ExprTree, EvalError> {
        
        fn parse_pre_aux(expr :&mut Iter<Symbol>) -> Result<ExprTree, EvalError>{

            // search for operators
            let opr = match expr.next() {
                None    => return Err(EvalError::InvalidSyntax(Order::Pre)),
                Some(s) => s
            };

            // Try to parse a binary expression
            if opr == "/" || opr == "*" || opr == "+" || opr == "-" {
                // If something went wrong, return an error
                let l = match parse_pre_aux(expr) {
                    Ok(t) => t,
                    e     => return e
                };

                let r = match parse_pre_aux(expr) {
                    Ok(t) => t,
                    e     => return e
                };

                return Ok(
                            ExprTree::Tree(
                                opr.chars().next().unwrap(), // operator
                                Box::new(l),                 // left child
                                Box::new(r)                  // right child
                            )
                        );
            }

            // try to parse a number; if not possible, that's not a valid symbol
            match opr.parse::<i32>() {
                Ok(i) => Ok(ExprTree::Num(i)),
                Err(_) => Err(EvalError::InvalidSymbol(opr.to_string()))
            }
        }

        let mut expr = expr.iter();
        let out = parse_pre_aux(&mut expr);

        // check that there's nothing left to parse
        match expr.next() {
            None => out,
            _    => Err(EvalError::InvalidSyntax(Order::Pre))
        }
    }

    fn parse_post(expr : Expr) -> Result<ExprTree, EvalError> {

        let mut stack : Stack<ExprTree> = Stack::new();
        for s in expr {
            if s == "/" || s == "*" || s == "+" || s == "-" {
                let r = match stack.pop() {
                    None => return Err(EvalError::InvalidSyntax(Order::Post)),
                    Some(t) => t
                };

                let l = match stack.pop() {
                    None => return Err(EvalError::InvalidSyntax(Order::Post)),
                    Some(t) => t
                };

                stack.push(
                    ExprTree::Tree(
                        s.chars().next().unwrap(), 
                        Box::new(l), 
                        Box::new(r)
                    )
                );
                continue;
            }

            match s.parse::<i32>() {
                Ok(i) => stack.push(ExprTree::Num(i)),
                _     => return Err(EvalError::InvalidSymbol(s.to_string()))
            }
        };

        if stack.len() != 1 {
            return Err(EvalError::InvalidSyntax(Order::Post))
        };

        Ok(stack.pop().unwrap())
    }
}