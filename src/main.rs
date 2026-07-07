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

fn tokenize(expr: &str) -> &[Token] {
    todo!()
}

// scheme_reader.py#L105
// in the original, scheme_read returns and removes the next complete expression in src
// i will instead leave it immutable
fn scheme_read(buffer: &[Token]) -> Result<Expr, SchemeErr> {
    todo!()
}

fn read_tail(buffer: &[Token]) -> Result<Expr, SchemeErr> {
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

    // Tests cases transpiled from 61A by Gemini TODO fix them
    // Helper functions to keep tests concise.
    // Replace `Expr` and `Token` with your actual AST and Lexer enums.
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

    fn read_line(line: &str) -> Result<Expr, SchemeErr> {
        scheme_read(&tokenize(line))
    }

    #[test]
    fn test_case_1() {
        assert_eq!(scheme_read(tokenize("nil")), Ok(nil()));
        assert_eq!(scheme_read(tokenize("1")), Ok(int(1)));
        assert_eq!(scheme_read(tokenize("true")), Ok(boolean(true)));

        assert_eq!(read_line("3"), Ok(int(3)));
        assert_eq!(read_line("-123"), Ok(int(-123)));
        assert_eq!(read_line("1.25"), Ok(float(1.25)));
        assert_eq!(read_line("true"), Ok(boolean(true)));
        assert_eq!(read_line("(a)"), Ok(pair(sym("a"), nil())));

        assert!(read_line(")").is_err());
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
        assert_eq!(scheme_read(src), Ok(expected));

        assert_eq!(src.current(), &Token::ParenClose);
    }

    #[test]
    fn test_case_3() {
        let src = tokenize("(18 6)");
        let expected = pair(int(18), pair(int(6), nil()));

        assert_eq!(scheme_read(src), Ok(expected.clone()));
        assert_eq!(read_line("(18 6)"), Ok(expected)); // Shorter version of above
    }

    #[test]
    fn test_case_4() {
        let mut src1 = tokenize(")");
        assert_eq!(read_tail(&mut src1), Ok(nil()));

        let mut src2 = tokenize("1 2 3)");
        let expected2 = pair(int(1), pair(int(2), pair(int(3), nil())));
        assert_eq!(read_tail(&mut src2), Ok(expected2));

        let mut src3 = tokenize("2 (3 4))");
        let expected3 = pair(int(2), pair(pair(int(3), pair(int(4), nil())), nil()));
        assert_eq!(read_tail(&mut src3), Ok(expected3));
    }

    #[test]
    fn test_case_5() {
        let mut src = tokenize("(1 2 3)");
        assert!(read_tail(&mut src).is_err());

        assert!(read_line("((1 2 3)").is_err());
    }

    #[test]
    fn test_case_6() {
        let src = tokenize("(+ 1 2)");
        let expected = pair(sym("+"), pair(int(1), pair(int(2), nil())));

        assert_eq!(scheme_read(src), Ok(expected));
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
        assert_eq!(read_line("(+ (- 2 3) 1)"), Ok(expected));
    }

    #[test]
    fn test_case_8() {
        assert_eq!(read_line("()"), Ok(nil()));

        let expected_nested = pair(pair(sym("a"), nil()), nil());
        assert_eq!(read_line("((a))"), Ok(expected_nested));

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
        assert_eq!(read_line("(+ 1 (- 2 3) 8)"), Ok(expected_math));
    }
}
