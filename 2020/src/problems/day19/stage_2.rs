use std::collections::HashMap;

use pest::Parser;

use crate::util::read_lines;

#[derive(Clone, Debug)]
enum TextRuleDef {
    Seq(String),
    RuleSeq(Vec<u32>),
    Alternative(Box<TextRuleDef>, Box<TextRuleDef>),
    Combination(Vec<TextRuleDef>),
    ZeroOrMore(Box<TextRuleDef>)
}

#[derive(Clone, Debug)]
struct TextRule {
    id: u32,
    body: TextRuleDef
}

pub fn run() {
    let mut lines = read_lines("./src/problems/day19/input.txt").unwrap();
    let mut rules = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        let rule = TextRule::from(&line);
        rules.insert(rule.id, rule);
    }

    rules.insert(8, TextRule { id: 8, body: TextRuleDef::Combination(vec![TextRuleDef::RuleSeq(vec![42]), TextRuleDef::ZeroOrMore(Box::new(TextRuleDef::RuleSeq(vec![8])))]) });
    rules.insert(11, TextRule { id: 11, body: TextRuleDef::Combination(vec![TextRuleDef::RuleSeq(vec![42]), TextRuleDef::ZeroOrMore(Box::new(TextRuleDef::RuleSeq(vec![11]))), TextRuleDef::RuleSeq(vec![31])]) });

    let mut count = 0;
    while let Some(Ok(line)) = lines.next() {
        if rules[&0].matches(&line, &rules) {
            count += 1;
        }
    }
    println!("Result: {}", count);
}


impl TextRule {
    fn from(line: &str) -> TextRule {
        let mut pairs = RuleParser::parse(Rule::RuleLine, line).unwrap();
        let pair = pairs.next().unwrap();
        if let Rule::Rule = pair.as_rule() {
            return create_from_parse_tree(pair);
        }
        panic!("Could not parse. Got: {:?}", pair);
    }

    fn matches(&self, line: &str, rules: &HashMap<u32, TextRule>) -> bool {
        let (not_matched, result) = self.body.matches(line, rules);
        if result == false {
            return false;
        }
        return not_matched.len() == 0;
    }
}

impl TextRuleDef {
    fn matches<'a> (&self, line: &'a str, existing_rules: &HashMap<u32, TextRule>) -> (&'a str, bool) {
        // TODO: Every rule should be able to return a set of matches.
        match self {
            TextRuleDef::Seq(text) => if line.starts_with(text) { (&line[text.len()..], true) } else { (line, false) }
            TextRuleDef::RuleSeq(rules) => {
                let mut remaining = line;
                for rule in rules {
                    let (line_result, match_result) = existing_rules[rule].body.matches(remaining, existing_rules);
                    if !match_result {
                        return (line, false);
                    }
                    remaining = line_result;
                }
                (remaining, true)
            }
            TextRuleDef::Alternative(lhs, rhs) => {
                let (lhs_line, result) = lhs.matches(line, existing_rules);
                if result == true {
                    return (lhs_line, result);
                }
                rhs.matches(line, existing_rules)
            }
            TextRuleDef::Combination(rules) => {
                let mut remaining = line;
                for rule in rules {
                    let (line_result, match_result) = rule.matches(remaining, existing_rules);
                    if !match_result {
                        return (line, false);
                    }
                    remaining = line_result;
                }
                (remaining, true)
            }
            TextRuleDef::ZeroOrMore(inner) => {
                let mut remaining = line;
                let mut result = true;

                while result == true  {
                    let (line_result, match_result) = inner.matches(remaining, existing_rules);
                    remaining = line_result;
                    result = match_result;
                }
                (remaining, true)
            }
        }
    }
}

fn create_from_parse_tree(pair: pest::iterators::Pair<Rule>) -> TextRule {
    match pair.as_rule() {
        Rule::Rule => {
            let mut iter = pair.into_inner();
            let id = iter.next().unwrap().as_str().parse().unwrap();
            let body = create_body_from_parse_tree(iter.next().unwrap());
            TextRule { id, body }
        },
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn create_body_from_parse_tree(pair: pest::iterators::Pair<Rule>) -> TextRuleDef {
    match pair.as_rule() {
        Rule::Literal => {
            TextRuleDef::Seq(pair.as_str()[1..pair.as_str().len()-1].to_string())
        },
        Rule::Alternative => {
            let mut pair = pair.into_inner();
            let lhs = create_body_from_parse_tree(pair.next().unwrap());
            let rhs = create_body_from_parse_tree(pair.next().unwrap());
            TextRuleDef::Alternative(Box::new(lhs),Box::new(rhs))
        },
        Rule::RuleList => {
            let pair = pair.into_inner();
            let ids = pair.map(|p| p.as_str().parse().unwrap()).collect();
            TextRuleDef::RuleSeq(ids)
        },
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

#[derive(Parser)]
#[grammar = "problems/day19/grammar.pest"]
struct RuleParser;
