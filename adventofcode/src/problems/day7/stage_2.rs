use super::baggage::BaggageDefinitions;

pub fn run() {
    let definitions = BaggageDefinitions::read();
    println!("Count: {}", definitions.count_bags_within("shiny gold"))
}