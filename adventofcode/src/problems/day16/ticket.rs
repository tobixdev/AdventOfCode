use std::collections::HashSet;
use regex::Regex;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

#[derive(Debug,Clone)]
pub struct TicketRule {
    pub field: String,
    first_range: (u32,u32),
    second_range: (u32,u32)
}

impl TicketRule {
    pub fn from(line: &str) -> TicketRule {
        let captures = RULE_REGEX.captures(&line).unwrap();
        TicketRule {
            field: captures[1].to_string(),
            first_range: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
            second_range: (captures[4].parse().unwrap(), captures[5].parse().unwrap())
        }
    }

    fn value_is_within(&self, value: u32) -> bool {
        self.first_range.0 <= value && value <= self.first_range.1 || self.second_range.0 <= value && value <= self.second_range.1
    }
}

#[derive(Debug,Clone)]
pub struct Ticket {
    pub values: Vec<u32>
}

impl Ticket {
    pub fn from(line: &str) -> Ticket {
        Ticket {
            values: line.split(",").map(|l| l.parse().unwrap()).collect()
        }
    }

    pub fn is_valid(&self, rules: &Vec<TicketRule>) -> bool {
        self.get_invalid_numbers(rules).len() == 0
    }

    pub fn get_invalid_numbers(&self, rules: &Vec<TicketRule>) -> Vec<u32> {
        let mut result = Vec::new();
        for value in self.values.iter() {
            let mut rule_found = false;
            for rule in rules {
                if rule.value_is_within(*value) {
                   rule_found = true;
                   break; 
                }
            }
            if !rule_found {
                result.push(*value);
            }
        } 
        result
    }

    pub fn get_possible_fields(&self, rules: &Vec<TicketRule>) -> Vec<HashSet<String>> {
        let mut result = Vec::new();

        for value in self.values.iter() {
            let mut result_rules = HashSet::new();
            for rule in rules {
                if rule.value_is_within(*value) {
                    result_rules.insert(rule.field.clone());
                }
            }
            result.push(result_rules);
        }

        result
    }
}