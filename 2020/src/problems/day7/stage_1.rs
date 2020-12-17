use super::baggage::BaggageDefinitions;

pub fn run() {
    let definitions = BaggageDefinitions::read();
    println!("Count: {}", definitions.count_possible_root_bags("shiny gold"))
}