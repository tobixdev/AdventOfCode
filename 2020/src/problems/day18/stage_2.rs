use crate::util::read_lines;
use pest::Parser;

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
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn parse(expression: &str) -> Expression {
        let pairs = ExpressionParser::parse(Rule::Calculation, expression).unwrap();
        for pair in pairs {
            if let Rule::Expr = pair.as_rule() {
                return create_expr(pair);
            }
        }
        panic!("Could not parse");
    }

    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Number(c) => *c,
            Expression::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Multiply(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }
}

fn create_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::Expr => create_expr(pair.into_inner().next().unwrap()),
        Rule::Int => {
            Expression::Number(pair.as_str().parse().unwrap())
        },
        Rule::Add => {
            let mut pair = pair.into_inner();
            let lhs = create_expr(pair.next().unwrap());
            let rhs = pair.next();
            if rhs.is_none() {
                return lhs;
            }
            let rhs = create_expr(rhs.unwrap());
            Expression::Add(Box::new(lhs), Box::new(rhs))
        },
        Rule::Mul => {
            let mut pair = pair.into_inner();
            let lhs = create_expr(pair.next().unwrap());
            let rhs = pair.next();
            if rhs.is_none() {
                return lhs;
            }
            let rhs = create_expr(rhs.unwrap());
            Expression::Multiply(Box::new(lhs), Box::new(rhs))
        },
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

#[derive(Parser)]
#[grammar = "problems/day18/grammar.pest"] // relative to src
struct ExpressionParser;
