use crate::util::read_lines;
use regex::Regex;

pub fn run() {
    let result: i64 = read_lines("./src/problems/day18/input.txt")
        .unwrap()
        .map(|line| line.unwrap().replace(" ", ""))
        .map(|l| Expression::parse(&l))
        .map(|e| e.evaluate())
        .sum();
    println!("Result: {}", result);
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>)
}

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"^\d+").unwrap();
}

impl Expression {
    pub fn parse(expression: &str) -> Expression {
        // reverse to avoid left recursion
        let reversed = expression.chars().rev().collect::<String>();
        let (result, remaining) = parse_expression(&reversed);
        assert!(remaining.len() == 0);
        *result.unwrap()
    }

    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Constant(c) => *c,
            Expression::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Multiply(lhs, rhs) => lhs.evaluate() * rhs.evaluate()
        }
    }
}

// Hakiest parser in the world. Pls no lookeroo and definetely no copyerooo
fn parse_expression(expression: &str) -> (Option<Box<Expression>>, &str) {
    let (lhs, expression) = parse_term(expression);

    if lhs.is_none() {
        return (None, expression);
    }

    let operator = expression.chars().nth(0);

    if let Some('(') = operator {
        return (lhs, expression);
    }

    if operator.is_none() {
        return (lhs, expression);
    }

    let (rhs, expression) = parse_expression(&expression[1..]);

    if rhs.is_none() {
        return (None, expression);
    }

    let expr = match operator.unwrap() {
        '+' => Expression::Add(lhs.unwrap(), rhs.unwrap()),
        '*' => Expression::Multiply(lhs.unwrap(), rhs.unwrap()),
        _ => panic!("Unknown operator {}.", operator.unwrap())
    };
    (Some(Box::new(expr)), expression)
}

fn parse_term(expression: &str) -> (Option<Box<Expression>>, &str) {
    if expression.chars().nth(0).unwrap() == ')' {
        let (result, remaining) = parse_expression(&expression[1..]);
        assert!(remaining.chars().nth(0).unwrap() == '(');
        return (result, &remaining[1..]);
    }
    parse_number(expression)
}

fn parse_number(expression: &str) -> (Option<Box<Expression>>, &str) {
    let opt_captures = NUMBER.captures(expression);
    if let Some(captures) = opt_captures {
        let number = &captures[0];
        let expression = &expression[number.len()..];
        return (Some(Box::new(Expression::Constant(number.parse().unwrap()))), expression);
    }
    (None, expression)
}