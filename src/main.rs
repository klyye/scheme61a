use std::io;
// https://stopa.io/post/222

#[derive(Clone)]
enum Expression {
    Symbol(String),
    Number(f64),
    List(Vec<Expression>)
}

#[derive(Debug)]
enum SchemeError {
    Reason(String)
}


fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse(tokens: Vec<String>) -> Result<Expression, SchemeError> {
    let (token, rest) = tokens.split_first().ok_or(SchemeError::Reason("couldn't split tokens".to_string()))?;
    match token.as_str() {
        "(" => {
            // loop through the tokens until you hit ')'
            // parse the next primitive and/or list
            // add it to the list
            // once you hit ')' return Expression::List
            todo!()
        },
        ")" => {Err(SchemeError::Reason("found closing paren without opening".to_string()))},
        _ => {
            todo!()
        }
    }
}

fn eval(expr: Expression) {
    todo!()
}

fn apply() -> u64 {
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

    #[test]
    fn test_tokenize() {
        let output = tokenize("(+ 10 5)".to_string());
        assert_eq!(output, ["(", "+", "10", "5", ")"])
    }
}