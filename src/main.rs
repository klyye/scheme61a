use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
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
fn parse_expr<'a>(buffer: &mut impl Iterator<Item = &'a Token>) -> Result<Expr, SchemeErr> {
    match buffer.next() {
        None => Err(SchemeErr::Reason("Tried to parse empty buffer".to_string())),
        Some(Token::ParenOpen) => parse_list(buffer),
        Some(Token::ParenClose) => Ok(Expr::Nil),// TODO this is wrong
        Some(Token::Integer(i)) => Ok(Expr::Integer(*i)),
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
fn parse_list<'a>(buffer: &mut impl Iterator<Item = &'a Token>) -> Result<Expr, SchemeErr> {
    Ok(Expr::Pair(
        Box::new(parse_expr(buffer)?),
        Box::new(parse_expr(buffer)?),
    ))
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

    mod test_parse {
        use super::Expr::*;
        use super::*;

        // TODO standardize use of parse_str_expr
        fn parse_str_expr(line: &str) -> Result<Expr, SchemeErr> {
            parse_expr(&mut tokenize(line).iter())
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
            let tokens = tokenize("(+ 1 (23 4))");
            let expected = pair(
                sym("+"),
                pair(Integer(1), pair(Integer(23), pair(Integer(4), Nil))),
            );
            assert_eq!(parse_expr(&mut tokens.iter()), Ok(expected));
        }

        #[test]
        fn test_parser_parens() {
            let tokens = tokenize("(+ (1 23) 4)");
            let expected = pair(
                sym("+"),
                pair(
                    pair(Integer(1), pair(Integer(23), Nil)),
                    pair(Integer(4), Nil),
                ),
            );
            assert_eq!(parse_expr(&mut tokens.iter()), Ok(expected));
        }

        #[test]
        fn test_case_2() {
            let src = tokenize("(+ 1 (23 4)) (");

            assert_eq!(src[0], Token::ParenOpen);
            assert_eq!(src[1], Token::Symbol("+".to_string()));
            assert_eq!(src[2], Token::Integer(1));

            let expected = pair(Integer(23), pair(Integer(4), Nil));
            // TODO okay cs61a had the right idea, we should test the 'current' and 'next' calls
            // on an ITERATOR not the token vector, so revert these tests back
            assert_eq!(parse_expr(&mut src.iter()), Ok(expected));

            // assert_eq!(src.current(), &Token::ParenClose);
        }

        #[test]
        fn test_case_3() {
            let src = tokenize("(18 6)");
            let expected = pair(Integer(18), pair(Integer(6), Nil));

            assert_eq!(parse_expr(&mut src.iter()), Ok(expected.clone()));
            assert_eq!(parse_str_expr("(18 6)"), Ok(expected)); // Shorter version of above
        }

        #[test]
        fn test_case_4() {
            let src1 = tokenize(")");
            assert_eq!(parse_expr(&mut src1.iter()), Ok(Nil));

            let src2 = tokenize("(1 2 3)");
            let expected2 = pair(Integer(1), pair(Integer(2), pair(Integer(3), Nil)));
            assert_eq!(parse_expr(&mut src2.iter()), Ok(expected2));

            let src3 = tokenize("2 (3 4))");
            let expected3 = pair(
                Integer(2),
                pair(pair(Integer(3), pair(Integer(4), Nil)), Nil),
            );
            assert_eq!(parse_expr(&mut src3.iter()), Ok(expected3));
        }

        #[test]
        fn test_case_5() {
            let src = tokenize("(1 2 3)");
            assert!(parse_expr(&mut src.iter()).is_err());

            assert!(parse_str_expr("((1 2 3)").is_err());
        }

        #[test]
        fn test_case_6() {
            let src = tokenize("(+ 1 2)");
            let expected = pair(sym("+"), pair(Integer(1), pair(Integer(2), Nil)));
            assert_eq!(parse_expr(&mut src.iter()), Ok(expected));
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
}
