use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::iter::Peekable;
// https://stopa.io/post/222
// https://inst.eecs.berkeley.edu/~cs61a/archive/fa19/proj/scheme_stubbed/
// https://code.cs61a.org/

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Nil,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Symbol(String),
    Pair(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
enum Token {
    ParenOpen,
    ParenClose,
    Integer(i64),
    Float(f64),
    Bool(bool),
    Symbol(String),
}

#[derive(Debug, PartialEq)]
enum SchemeErr {
    Reason(String),
}

impl Display for SchemeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let SchemeErr::Reason(x) = self;
        write!(f, "{}", x)
    }
}

impl Error for SchemeErr {}

/// technically a lexer?
fn tokenize(expr: &str) -> Vec<Token> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| match x {
            "(" => Token::ParenOpen,
            ")" => Token::ParenClose,
            _ => {
                if let Ok(num) = x.parse::<i64>() {
                    Token::Integer(num)
                } else if let Ok(x) = x.parse::<bool>() {
                    Token::Bool(x)
                } else if let Ok(x) = x.parse::<f64>() {
                    Token::Float(x)
                } else {
                    Token::Symbol(x.to_string())
                }
            }
        })
        .collect()
}

// scheme_reader.py#L105 scheme_read function
// mutually recursive with parse_rest
// Evals the FIRST given expr in buffer
// Read the next expression from SRC, a Buffer of tokens.
//
//     >>> scheme_read(Buffer(tokenize_lines(['nil'])))
//     nil
//     >>> scheme_read(Buffer(tokenize_lines(['1'])))
//     1
//     >>> scheme_read(Buffer(tokenize_lines(['true'])))
//     True
//     >>> scheme_read(Buffer(tokenize_lines(['(+ 1 2)'])))
//     Pair('+', Pair(1, Pair(2, nil)))
fn parse_expr<'a>(
    buffer: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Expr, SchemeErr> {
    match buffer.next() {
        // None => Ok(Expr::Nil),
        None => Err(SchemeErr::Reason("Tried to parse empty buffer".to_string())),
        Some(Token::ParenOpen) => parse_list(buffer),
        // Some(Token::ParenClose) => Ok(Expr::Nil),
        Some(Token::ParenClose) => Err(SchemeErr::Reason(
            "Can't start expression with closing parenthesis".to_string(),
        )),
        Some(Token::Integer(i)) => Ok(Expr::Integer(*i)),
        Some(Token::Float(f)) => Ok(Expr::Float(*f)),
        Some(Token::Bool(b)) => Ok(Expr::Bool(*b)),
        Some(Token::Symbol(s)) => Ok(Expr::Symbol(s.clone())),
    }
}

// todo do a match and mutually recurse
// Return the remainder of a list in buffer, starting before an element or ).
//
//     >>> read_tail(Buffer(tokenize_lines([')'])))
//     nil
//     >>> read_tail(Buffer(tokenize_lines(['2 3)'])))
//     Pair(2, Pair(3, nil))
// parse_list(tokenize['1 2 3 4)'])
// Pair(1, Pair(2, Pair(3, Pair(4, nil))))
// parse_list(tokenize['(1))']) -> Pair(Pair(1, nil), nil)
fn parse_list<'a>(
    buffer: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Expr, SchemeErr> {
    match buffer.peek() {
        Some(Token::ParenClose) => {
            buffer.next();
            Ok(Expr::Nil)
        }
        _ => Ok(Expr::Pair(
            Box::new(parse_expr(buffer)?),
            Box::new(parse_list(buffer)?),
        )),
    }
}

// Stubs for Part 2 Evaluation
struct Env {
    bindings: HashMap<String, Expr>,
}
impl Env {
    fn global() -> Self {
        Env {
            bindings: HashMap::new(),
        }
    }
}

// da plan: bind the builtins (e.g. + - * / in the default global env)
fn eval(expr: &Expr, env: &mut Env) -> Result<Expr, SchemeErr> {
    match expr {
        Expr::Nil | Expr::Integer(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Symbol(_) => {
            Ok(expr.clone())
        }
        Expr::Pair(op, params) => apply(op.as_ref(), params.as_ref(), env),
        // todo recursively eval inner terms first
    }
}

fn apply(proc: &Expr, args: &Expr, env: &mut Env) -> Result<Expr, SchemeErr> {
    let Expr::Symbol(proc_sym) = proc else {
        return Err(SchemeErr::Reason(format!("proc {proc:?} is not a Symbol!")));
    };
    let body = env.bindings.get(proc_sym);
    //todo replace replace argnames in body with args (bindings? same mechanism as env bindings)
    todo!()
}

fn main() {
    // todo repl loop, print 'str' and 'repr' equivalents (Display and Debug respectively)
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let tokens = tokenize(&buffer);
        let iterator = &mut tokens.iter().peekable();
        while iterator.peek().is_some() {
            match parse_expr(iterator).and_then(|x| eval(&x, &mut Env::global())) {
                Ok(x) => println!("{:?}", x),
                Err(x) => eprintln!("Error: {}", x),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    mod test_tokenize {
        use super::Token::*;
        use super::*;

        #[test]
        fn test_tokenize() {
            let output = tokenize("(+ 10 5)");
            assert_eq!(
                output,
                [
                    ParenOpen,
                    Symbol("+".to_string()),
                    Integer(10),
                    Integer(5),
                    ParenClose
                ]
            );
            let tokens = tokenize("(- 1 (23 4))");
            assert_eq!(
                tokens,
                [
                    ParenOpen,
                    Symbol("-".to_string()),
                    Integer(1),
                    ParenOpen,
                    Integer(23),
                    Integer(4),
                    ParenClose,
                    ParenClose
                ]
            );
        }

        #[test]
        fn test_tokenize_empty_expr() {
            let output = tokenize("()");
            assert_eq!(output, [ParenOpen, ParenClose])
        }

        #[test]
        fn test_tokenize_empty_str() {
            let output = tokenize("");
            assert_eq!(output, [])
        }

        #[test]
        fn test_tokenize_numbered_names() {
            let output = tokenize("1be");
            assert_eq!(output, [Symbol("1be".to_string())]);
            let output2 = tokenize("belgium234");
            assert_eq!(output2, [Symbol("belgium234".to_string())])
        }
    }

    mod test_parse {
        use super::Expr::*;
        use super::*;

        fn parse_str_expr(line: &str) -> Result<Expr, SchemeErr> {
            parse_expr(&mut tokenize(line).iter().peekable())
        }

        fn sym(s: &str) -> Expr {
            Symbol(s.to_string())
        }

        fn pair(car: Expr, cdr: Expr) -> Expr {
            Pair(Box::new(car), Box::new(cdr))
        }

        #[test]
        fn test_case_1() {
            assert_eq!(parse_str_expr("()"), Ok(Nil));
            assert_eq!(parse_str_expr("1"), Ok(Integer(1)));
            assert_eq!(parse_str_expr("true"), Ok(Bool(true)));

            assert_eq!(parse_str_expr("3"), Ok(Integer(3)));
            assert_eq!(parse_str_expr("-123"), Ok(Integer(-123)));
            assert_eq!(parse_str_expr("1.25"), Ok(Float(1.25)));
            assert_eq!(parse_str_expr("true"), Ok(Bool(true)));
            assert_eq!(parse_str_expr("(a)"), Ok(pair(sym("a"), Nil)));

            assert!(parse_str_expr(")").is_err());
        }

        #[test]
        fn test_parser_basic() {
            let expected = pair(
                sym("+"),
                pair(
                    Integer(1),
                    pair(pair(Integer(23), pair(Integer(4), Nil)), Nil),
                ),
            );
            assert_eq!(parse_str_expr("(+ 1 (23 4))"), Ok(expected));
        }

        #[test]
        fn test_parser_parens() {
            let expected = pair(
                sym("+"),
                pair(
                    pair(Integer(1), pair(Integer(23), Nil)),
                    pair(Integer(4), Nil),
                ),
            );
            assert_eq!(parse_str_expr("(+ (1 23) 4)"), Ok(expected));
        }

        #[test]
        fn test_case_2() {
            let tokens = tokenize("(+ 1 (23 4)) (");
            let buffer = &mut (tokens.iter().peekable());
            let expected = pair(
                Symbol("+".to_string()),
                pair(
                    Integer(1),
                    pair(pair(Integer(23), pair(Integer(4), Nil)), Nil),
                ),
            );
            assert_eq!(parse_expr(buffer), Ok(expected));
            assert!(parse_expr(buffer).is_err())
        }

        #[test]
        fn test_case_3() {
            let expected = pair(Integer(18), pair(Integer(6), Nil));
            assert_eq!(parse_str_expr("(18 6)"), Ok(expected));
        }

        #[test]
        fn test_case_4() {
            assert_eq!(
                parse_list(&mut [Token::ParenClose].iter().peekable()),
                Ok(Nil)
            );

            let expected2 = pair(Integer(1), pair(Integer(2), pair(Integer(3), Nil)));
            assert_eq!(parse_str_expr("(1 2 3)"), Ok(expected2));

            let expected3 = pair(
                Integer(2),
                pair(pair(Integer(3), pair(Integer(4), Nil)), Nil),
            );
            assert_eq!(parse_str_expr("(2 (3 4))"), Ok(expected3));
        }

        #[test]
        fn test_case_5() {
            let src = tokenize("(1 2 3)");
            assert!(parse_list(&mut src.iter().peekable()).is_err());

            assert!(parse_str_expr("((1 2 3)").is_err());
        }

        #[test]
        fn test_case_6() {
            let expected = pair(sym("+"), pair(Integer(1), pair(Integer(2), Nil)));
            assert_eq!(parse_str_expr("(+ 1 2)"), Ok(expected));
        }

        #[test]
        fn test_case_7() {
            let expected = pair(
                sym("+"),
                pair(
                    pair(sym("-"), pair(Integer(2), pair(Integer(3), Nil))),
                    pair(Integer(1), Nil),
                ),
            );
            assert_eq!(parse_str_expr("(+ (- 2 3) 1)"), Ok(expected));
        }

        #[test]
        fn test_case_8() {
            assert_eq!(parse_str_expr("()"), Ok(Nil));

            let expected_nested = pair(pair(sym("a"), Nil), Nil);
            assert_eq!(parse_str_expr("((a))"), Ok(expected_nested));

            let expected_math = pair(
                sym("+"),
                pair(
                    Integer(1),
                    pair(
                        pair(sym("-"), pair(Integer(2), pair(Integer(3), Nil))),
                        pair(Integer(8), Nil),
                    ),
                ),
            );
            assert_eq!(parse_str_expr("(+ 1 (- 2 3) 8)"), Ok(expected_math));
        }
    }

    // Generated by Gemini 3.1 Pro
    mod test_parse_quotes {
        use super::Expr::*;
        use super::*;

        fn parse_str_expr(line: &str) -> Result<Expr, SchemeErr> {
            parse_expr(&mut tokenize(line).iter().peekable())
        }

        fn sym(s: &str) -> Expr {
            Symbol(s.to_string())
        }
        fn pair(car: Expr, cdr: Expr) -> Expr {
            Pair(Box::new(car), Box::new(cdr))
        }

        #[test]
        fn test_quote_sugar() {
            // NOTE: To pass these tests, you will need to update `tokenize` and
            // `parse_expr` to support the single quote (') syntactic sugar.
            let expected_quote = pair(sym("quote"), pair(sym("hello"), Nil));
            assert_eq!(parse_str_expr("'hello"), Ok(expected_quote));

            let nested_quote = pair(
                sym("quote"),
                pair(pair(sym("quote"), pair(sym("a"), Nil)), Nil),
            );
            assert_eq!(parse_str_expr("''a"), Ok(nested_quote));

            assert!(parse_str_expr("')").is_err()); // SyntaxError
        }
    }

    mod test_eval {
        use super::Expr::*;
        use super::*;

        // Helper functions
        fn sym(s: &str) -> Expr {
            Symbol(s.to_string())
        }
        fn pair(car: Expr, cdr: Expr) -> Expr {
            Pair(Box::new(car), Box::new(cdr))
        }

        // Simulates read_line followed by scheme_eval
        fn eval_str(line: &str, env: &mut Env) -> Result<Expr, SchemeErr> {
            let tokens = tokenize(line);
            let mut buffer = tokens.iter().peekable();
            let expr = parse_expr(&mut buffer)?;
            eval(&expr, env)
        }

        #[test]
        fn test_basic_math_and_builtins() {
            let mut env = Env::global();

            // both the code and its results are of type Expr because Lisp is "homoiconic"
            assert_eq!(eval_str("(+ 2 3)", &mut env), Ok(Integer(5)));
            assert_eq!(eval_str("(* (+ 3 2) (+ 1 7))", &mut env), Ok(Integer(40)));
            assert_eq!(eval_str("(+)", &mut env), Ok(Integer(0)));
            assert_eq!(eval_str("(*)", &mut env), Ok(Integer(1)));

            assert_eq!(eval_str("(odd? 13)", &mut env), Ok(Bool(true)));

            // Nested expressions
            assert_eq!(
                eval_str("(+ (+ 2 2) (+ 1 3) (* 1 4))", &mut env),
                Ok(Integer(12))
            );
            assert_eq!(
                eval_str("(+ (+ 1) (* 2 3) (+ 5) (+ 6 (+ 7)))", &mut env),
                Ok(Integer(25))
            );
        }

        #[test]
        fn test_evaluation_errors() {
            let mut env = Env::global();

            // Undefined symbols and operators
            assert!(eval_str("hello", &mut env).is_err());
            assert!(eval_str("(yolo)", &mut env).is_err());
            assert!(eval_str("(-)", &mut env).is_err());

            // Invalid operator applications
            assert!(eval_str("(1 2)", &mut env).is_err());

            // Procedure checking before operands
            assert!(eval_str("(1 (print 0))", &mut env).is_err());

            // Divide by zero
            assert!(eval_str("(+ (/ 1 0))", &mut env).is_err());
            assert!(eval_str("((/ 1 0) (print 5))", &mut env).is_err());
        }

        #[test]
        fn test_list_operations() {
            let mut env = Env::global();

            // Assuming `list` translates to chained pairs evaluated by Scheme
            assert_eq!(eval_str("(car (list 1 2 3 4))", &mut env), Ok(Integer(1)));
            assert_eq!(
                eval_str("(car (cdr (cdr (list 1 2 3 4))))", &mut env),
                Ok(Integer(3))
            );

            assert!(eval_str("(car car)", &mut env).is_err());
            assert!(eval_str("(car cdr (list 1))", &mut env).is_err());
            assert!(
                eval_str(
                    "(* (car (cdr (cdr (list 1 2 3 4)))) (cdr (cdr (list 1 2 3 4))))",
                    &mut env
                )
                .is_err()
            );
        }

        #[test]
        fn test_define_environment() {
            let mut env = Env::global();

            // Defining and looking up basic values
            assert_eq!(eval_str("(define size 2)", &mut env), Ok(sym("size"))); // Standard CS61A returns the symbol
            assert_eq!(eval_str("size", &mut env), Ok(Integer(2)));

            // Re-evaluating existing definitions
            assert_eq!(eval_str("(define x (+ 2 3))", &mut env), Ok(sym("x")));
            assert_eq!(eval_str("x", &mut env), Ok(Integer(5)));
            assert_eq!(eval_str("(define x (+ 2 7))", &mut env), Ok(sym("x")));
            assert_eq!(eval_str("x", &mut env), Ok(Integer(9)));

            // Invalid define
            assert!(eval_str("(define 0 1)", &mut env).is_err());
            assert!(eval_str("(define error (/ 1 0))", &mut env).is_err());
        }

        #[test]
        fn test_scheme_apply_api() {
            let mut env = Env::global();
            let twos = pair(Integer(2), pair(Integer(2), Nil));

            // Valid apply
            let plus = eval_str("+", &mut env).expect("Builtin + not found");
            assert_eq!(apply(&plus, &twos, &mut env), Ok(Integer(4)));

            // Invalid apply (oddp expects 1 argument, not 2)
            let oddp = eval_str("odd?", &mut env).expect("Builtin odd? not found");
            assert!(apply(&oddp, &twos, &mut env).is_err());
        }

        #[test]
        fn test_quotes_evaluation() {
            let mut env = Env::global();

            assert_eq!(eval_str("(quote hello)", &mut env), Ok(sym("hello")));
            assert_eq!(
                eval_str("'(1 2)", &mut env),
                Ok(pair(Integer(1), pair(Integer(2), Nil)))
            );
            assert_eq!(eval_str("(car '(1 2 3))", &mut env), Ok(Integer(1)));
            assert_eq!(
                eval_str("(cdr '(1 2))", &mut env),
                Ok(pair(Integer(2), Nil))
            );

            assert_eq!(eval_str("(car (car '((1))))", &mut env), Ok(Integer(1)));
            assert_eq!(eval_str("(quote 3)", &mut env), Ok(Integer(3)));
        }
    }
}
