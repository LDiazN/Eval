use crate::eval::*;

#[allow(dead_code)] // since we use this just in test suite
fn to_expr(expr : &str) -> Expr {
    expr
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[test]
fn test_eval_num(){
    // eval a num should return the num itself
    let s = String::from;

    assert_eq!(eval(&Order::Post, vec![s("2")]), Ok(2));
    assert_eq!(eval(&Order::Pre, vec![s("2")]), Ok(2));
}

#[test]
fn test_eval_expr_pre_izi(){

    let o = Order::Pre;

    // simple expressions
    let exp1 = to_expr("* 2 1");
    let exp2 = to_expr("+ 40 2");
    let exp3 = to_expr("/ 138 2");
    let exp4 = to_expr("- 444 24");

    assert_eq!(eval(&o, exp1), Ok(2));
    assert_eq!(eval(&o, exp2), Ok(42));
    assert_eq!(eval(&o, exp3), Ok(69));
    assert_eq!(eval(&o, exp4), Ok(420));
}

#[test]
fn test_eval_expr_pre_medium(){

    let o = Order::Pre;

    // more complex expressions
    let exp1 = to_expr("* / 2 1 + - 9 8 * 3 4"); // 2 / 1 * ( 9 - 8 + 3 * 4)
    let exp2 = to_expr("- - - 3 4 5 6"); // 3 - 4 - 5 - 6
    let exp3 = to_expr("/ 138 -2"); // with negative numbers
    let exp4 = to_expr("- + - 1 -1 1 -1"); // 1 - -1 + 1 - -1 = 4

    assert_eq!(eval(&o, exp1), Ok(26));
    assert_eq!(eval(&o, exp2), Ok(-12));
    assert_eq!(eval(&o, exp3), Ok(-69));
    assert_eq!(eval(&o, exp4), Ok(4));
}

#[test]
fn test_eval_expr_post_izi(){

    let o = Order::Post;

    // simple expressions
    let exp1 = to_expr("2 1 *");
    let exp2 = to_expr("40 2 +");
    let exp3 = to_expr("138 2 /");
    let exp4 = to_expr("444 24 -");

    assert_eq!(eval(&o, exp1), Ok(2));
    assert_eq!(eval(&o, exp2), Ok(42));
    assert_eq!(eval(&o, exp3), Ok(69));
    assert_eq!(eval(&o, exp4), Ok(420));
}

#[test]
fn test_eval_expr_post_medium(){

    let o = Order::Post;

    // more complex expressions
    let exp1 = to_expr(" 2 1 / 9 8 - 3 4 * + *"); // 2 / 1 * ( 9 - 8 + 3 * 4)
    let exp2 = to_expr("3 4 - 5 - 6 - "); // 3 - 4 - 5 - 6
    let exp3 = to_expr("138 -2 /"); // with negative numbers
    let exp4 = to_expr("1 -1 - 1 + -1 -"); // 1 - -1 + 1 - -1 = 4

    assert_eq!(eval(&o, exp1), Ok(26));
    assert_eq!(eval(&o, exp2), Ok(-12));
    assert_eq!(eval(&o, exp3), Ok(-69));
    assert_eq!(eval(&o, exp4), Ok(4));

}

#[test]
fn test_eval_expr_should_scream_division_by_zero_1() {
    
    assert_eq!(eval(&Order::Post, to_expr("2 0 /")), Err(EvalError::DividingByZero));
    assert_eq!(eval(&Order::Post, to_expr("2 1 1 - /")), Err(EvalError::DividingByZero));
}

#[test]
fn test_eval_expr_should_scream_division_by_zero_3() {
    
    assert_eq!(eval(&Order::Pre, to_expr("/ 2 0")), Err(EvalError::DividingByZero));
    assert_eq!(eval(&Order::Pre, to_expr("/ 2 - 1 1")), Err(EvalError::DividingByZero));
}

#[test]
fn test_eval_should_scream_invalid_syntax1() {
    let res = eval(&Order::Pre, to_expr("2 3 /"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Pre)));
}


#[test]
fn test_eval_should_scream_invalid_syntax2() {
    let res = eval(&Order::Post, to_expr("/ 2 3"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Post)));
}

#[test]
fn test_eval_should_scream_invalid_syntax3() {
    let res = eval(&Order::Pre, to_expr("/ 2 3 4"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Pre)));
}

#[test]
fn test_eval_should_scream_invalid_syntax4() {
    let res = eval(&Order::Post, to_expr("1 2 3 /"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Post)));
}

#[test]
fn test_eval_should_scream_invalid_syntax5() {
    let res = eval(&Order::Pre, to_expr("/"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Pre)));
}

#[test]
fn test_eval_should_scream_invalid_syntax6() {
    let res = eval(&Order::Post, to_expr("/"));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Post)));
}

#[test]
fn test_eval_should_scream_invalid_syntax7() {
    let res = eval(&Order::Pre, to_expr(""));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Pre)));
}

#[test]
fn test_eval_should_scream_invalid_syntax8() {
    let res = eval(&Order::Post, to_expr(""));

    assert_eq!(res,Err(EvalError::InvalidSyntax(Order::Post)));
}

#[test]
fn test_eval_should_scream_invalid_symbol1(){
    assert_eq!(
        eval(&Order::Post, to_expr("2 3 %")),
        Err(EvalError::InvalidSymbol(String::from("%")))
    )
}

#[test]
fn test_eval_should_scream_invalid_symbol2(){
    assert_eq!(
        eval(&Order::Pre, to_expr("% 2 3")),
        Err(EvalError::InvalidSymbol(String::from("%")))
    )
}

#[test]
fn test_eval_should_scream_invalid_symbol3(){
    assert_eq!(
        eval(&Order::Post, to_expr("2 c +")),
        Err(EvalError::InvalidSymbol(String::from("c")))
    )
}

#[test]
fn test_eval_should_scream_invalid_symbol4(){
    assert_eq!(
        eval(&Order::Pre, to_expr("+ a 3")),
        Err(EvalError::InvalidSymbol(String::from("a")))
    )
}