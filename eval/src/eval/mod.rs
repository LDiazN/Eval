// Our core logic
use std::slice::Iter;


type Symbol = String;
type Expr = Vec<Symbol>;
// Everyone: oh my gosh rust doesn't have a stack, so useless
// Me, an intellectual:
type Stack<T> = Vec<T>;

pub enum Order {
    Pre,
    Post
}

enum ExprTree {
    Num (i32),
    Tree (Box<ExprTree>)
}

pub enum EvalError {
    InvalidSyntax(Order),
    InvalidSymbol(String)
}



pub fn eval(order : Order, expr : Expr) -> Result<i32, EvalError> {
    match order {
        Order::Pre  => panic!("pre order not yet implemented"),
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

    }
}

impl ExprTree {
    pub fn parse(order : Order, expr : Expr) -> ExprTree {
        match order {
            Order::Pre => panic!("Pre order parsing into tree not yet implemented"),
            Order::Post=> panic!("Post order parsing into tree not yet implemented")
        }
    }

    fn parse_pre(expr :&mut Iter<Symbol>) -> Result<ExprTree, EvalError> {
        
        //match expr.next() {
        //    Some(s)
        //}

        Ok(ExprTree::Num(0))
    }
}