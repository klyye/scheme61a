use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
// https://stopa.io/post/222
// https://inst.eecs.berkeley.edu/~cs61a/archive/fa19/proj/scheme_stubbed/

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
    Symbol(String),
}

#[derive(Debug, PartialEq)]
enum SchemeErr {
    Reason(String),
}

impl Display for SchemeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for SchemeErr {}

/// technically a lexer?
fn tokenize(expr: &str) -> &[Token] {
    todo!()
}

// scheme_reader.py#L105 scheme_read function
// in the original, scheme_read returns and removes the next complete expression in src
// i will instead leave it immutable
// Instead of the mutually recursive approach used in the course, I will do an iterative approach
fn parse_expr(buffer: &[Token]) -> Result<Expr, SchemeErr> {
    todo!()
}

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        println!("{}", buffer)
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

    // Tests cases transpiled from 61A by Gemini TODO fix them
    // Helper functions to keep tests concise.
    fn nil() -> Expr {
        Expr::Nil
    }
    fn int(n: i64) -> Expr {
        Expr::Integer(n)
    }
    fn float(f: f64) -> Expr {
        Expr::Float(f)
    }
    fn boolean(b: bool) -> Expr {
        Expr::Bool(b)
    }
    fn sym(s: &str) -> Expr {
        Expr::Symbol(s.to_string())
    }
    fn pair(car: Expr, cdr: Expr) -> Expr {
        Expr::Pair(Box::new(car), Box::new(cdr))
    }

    fn parse_str_expr(line: &str) -> Result<Expr, SchemeErr> {
        parse_expr(tokenize(line))
    }

    #[test]
    fn test_case_1() {
        assert_eq!(parse_str_expr("nil"), Ok(nil()));
        assert_eq!(parse_str_expr("1"), Ok(int(1)));
        assert_eq!(parse_str_expr("true"), Ok(boolean(true)));

        assert_eq!(parse_str_expr("3"), Ok(int(3)));
        assert_eq!(parse_str_expr("-123"), Ok(int(-123)));
        assert_eq!(parse_str_expr("1.25"), Ok(float(1.25)));
        assert_eq!(parse_str_expr("true"), Ok(boolean(true)));
        assert_eq!(parse_str_expr("(a)"), Ok(pair(sym("a"), nil())));

        assert!(parse_str_expr(")").is_err());
    }

    #[test]
    fn test_parser_basic() {
        let tokens = tokenize("(+ 1 (23 4))");
        let expected = pair(sym("+"), pair(int(1), pair(int(23), pair(int(4), nil()))));
        assert_eq!(parse_expr(tokens), Ok(expected));
    }

    #[test]
    fn test_parser_parens() {
        let tokens = tokenize("(+ (1 23) 4)");
        let expected = pair(sym("+"), pair(pair(int(1), pair(int(23), nil())), pair(int(4), nil())));
        assert_eq!(parse_expr(tokens), Ok(expected));
    }

    #[test]
    fn test_case_2() {
        let tokens = tokenize("(+ 1 (23 4)) (");
        let mut src = tokens;

        assert_eq!(src[0], Token::ParenOpen);
        assert_eq!(src[1], Token::Symbol("+".to_string()));
        assert_eq!(src[2], Token::Integer(1));

        let expected = pair(int(23), pair(int(4), nil()));
        // TODO because scheme_read no longer consumes from input buffer, this test case is now wrong
        assert_eq!(parse_expr(src), Ok(expected));

        assert_eq!(src.current(), &Token::ParenClose);
    }

    #[test]
    fn test_case_3() {
        let src = tokenize("(18 6)");
        let expected = pair(int(18), pair(int(6), nil()));

        assert_eq!(parse_expr(src), Ok(expected.clone()));
        assert_eq!(parse_str_expr("(18 6)"), Ok(expected)); // Shorter version of above
    }

    #[test]
    fn test_case_4() {
        let src1 = tokenize(")");
        assert_eq!(parse_expr(src1), Ok(nil()));

        let src2 = tokenize("1 2 3)");
        let expected2 = pair(int(1), pair(int(2), pair(int(3), nil())));
        assert_eq!(parse_expr(src2), Ok(expected2));

        let src3 = tokenize("2 (3 4))");
        let expected3 = pair(int(2), pair(pair(int(3), pair(int(4), nil())), nil()));
        assert_eq!(parse_expr(src3), Ok(expected3));
    }

    #[test]
    fn test_case_5() {
        let src = tokenize("(1 2 3)");
        assert!(parse_expr(src).is_err());

        assert!(parse_str_expr("((1 2 3)").is_err());
    }

    #[test]
    fn test_case_6() {
        let src = tokenize("(+ 1 2)");
        let expected = pair(sym("+"), pair(int(1), pair(int(2), nil())));
        assert_eq!(parse_expr(src), Ok(expected));
    }

    #[test]
    fn test_case_7() {
        let expected = pair(
            sym("+"),
            pair(
                pair(sym("-"), pair(int(2), pair(int(3), nil()))),
                pair(int(1), nil()),
            ),
        );
        assert_eq!(parse_str_expr("(+ (- 2 3) 1)"), Ok(expected));
    }

    #[test]
    fn test_case_8() {
        assert_eq!(parse_str_expr("()"), Ok(nil()));

        let expected_nested = pair(pair(sym("a"), nil()), nil());
        assert_eq!(parse_str_expr("((a))"), Ok(expected_nested));

        let expected_math = pair(
            sym("+"),
            pair(
                int(1),
                pair(
                    pair(sym("-"), pair(int(2), pair(int(3), nil()))),
                    pair(int(8), nil()),
                ),
            ),
        );
        assert_eq!(parse_str_expr("(+ 1 (- 2 3) 8)"), Ok(expected_math));
    }
}
