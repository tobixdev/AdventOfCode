
use crate::problems::day16::ticket::Ticket;
use crate::problems::day16::ticket::TicketRule;
use crate::util::read_lines;

pub fn run() {
    let mut lines = read_lines("./src/problems/day16/input.txt").unwrap();
    let mut rules = Vec::new();
    let mut tickets = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        rules.push(TicketRule::from(&line));
    }

    lines.next();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
    }

    lines.next();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        tickets.push(Ticket::from(&line));
    }
    
    let mut invalid = Vec::new();
    for ticket in tickets {
        invalid.append(&mut ticket.get_invalid_numbers(&rules));
    }
    println!("Result: {}", invalid.iter().sum::<u32>());
}