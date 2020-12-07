use crate::util::read_lines;
use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"(\w+ \w+) bags?").unwrap();
    static ref BAG_REGEX_WITH_COUNT: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

pub struct BaggageDefinitions {
    definitions: Vec<BaggageDefinition>,
}

struct BaggageDefinition {
    color: String,
    childs: Vec<BaggageDefinitionChild>,
}

struct BaggageDefinitionChild {
    color: String,
    count: usize,
}

impl BaggageDefinitions {
    pub fn read() -> BaggageDefinitions {
        let lines = read_lines("./src/problems/day7/input.txt").unwrap();
        let mut definitions = Vec::<BaggageDefinition>::new();
        for line in lines {
            definitions.push(BaggageDefinition::from(&line.unwrap()));
        }

        return BaggageDefinitions {
            definitions: definitions,
        };
    }

    pub fn count_possible_root_bags(&self, color: &str) -> usize {
        // this isn't really efficient
        let mut possible_roots = HashSet::new();

        let mut work_set: VecDeque<&str> = VecDeque::new();
        work_set.push_back(color);
        while !work_set.is_empty() {
            let current = work_set.pop_front().unwrap();
            for definition in self.definitions.iter() {
                if definition.contains(&current.to_string()) {
                    possible_roots.insert(&definition.color);
                    work_set.push_back(&definition.color);
                }
            }
        }

        possible_roots.len()
    }

    pub fn count_bags_within(&self, color: &str) -> usize {
        // this isn't really efficient either
        let mut result = 0;

        let mut work_set: VecDeque<&str> = VecDeque::new();
        work_set.push_back(color);
        while !work_set.is_empty() {
            let current = work_set.pop_front().unwrap();
            for definition in self.definitions.iter() {
                if definition.color == current {
                    for child in definition.childs.iter() {
                        for _i in 0..child.count {
                            work_set.push_back(&child.color);
                        }
                        result += child.count;
                    }
                }
            }
        }

        result
    }
}

impl BaggageDefinition {
    fn from(line: &str) -> BaggageDefinition {
        let type_part = line.split("contain ").nth(0).unwrap();
        let contains_part = line.split("contain ").nth(1).unwrap();

        let bag_color = BAG_REGEX.captures(type_part).unwrap()[1].to_string();

        if contains_part == "no other bags." {
            return BaggageDefinition {
                color: bag_color,
                childs: Vec::new(),
            };
        }

        let mut child_colors = Vec::new();
        for child_color in contains_part.split(", ") {
            let child_bag_count = BAG_REGEX_WITH_COUNT.captures(child_color).unwrap()[1]
                .parse()
                .unwrap();
            let child_bag_color =
                BAG_REGEX_WITH_COUNT.captures(child_color).unwrap()[2].to_string();
            child_colors.push(BaggageDefinitionChild {
                count: child_bag_count,
                color: child_bag_color,
            });
        }

        return BaggageDefinition {
            color: bag_color,
            childs: child_colors,
        };
    }

    fn contains(&self, color: &str) -> bool {
        self.childs.iter().any(|c| c.color == color)
    }

    fn get_count(&self, color: &str) -> usize {
        self.childs
            .iter()
            .filter(|c| c.color == color)
            .next()
            .unwrap()
            .count
    }
}
