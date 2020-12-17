
use std::collections::HashSet;
use crate::problems::day16::ticket::Ticket;
use crate::problems::day16::ticket::TicketRule;
use crate::util::read_lines;
use std::iter::FromIterator;

pub fn run() {
    let mut lines = read_lines("./src/problems/day16/input.txt").unwrap();
    let mut rules = Vec::new();
    let mut my_ticket = None;
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
        my_ticket = Some(Ticket::from(&line));
    }

    lines.next();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        tickets.push(Ticket::from(&line));
    }

    let all_fields: HashSet<String> = rules.iter().cloned().map(|r| r.field).collect();
    let mut possible: Vec<HashSet<String>> = Vec::new();
    for _i in 0..all_fields.len() {
        possible.push(all_fields.clone());
    }

    for ticket in tickets.iter().filter(|t| t.is_valid(&rules)) {
        let mut i = 0;
        for fields in ticket.get_possible_fields(&rules) {
            possible[i] = HashSet::from_iter(possible[i].intersection(&fields).cloned());
            i += 1;
        }
    }

    let mut last_total_items = 0;
    let mut total_items: usize = possible.iter().map(|i| i.len()).sum();
    while last_total_items != total_items {
        for field in all_fields.iter() {
            let mut occurs_in: Vec<&mut HashSet<String>> = possible.iter_mut().filter(|p| p.contains(field)).collect();
            if occurs_in.len() == 1 {
                occurs_in[0].drain();
                occurs_in[0].insert(field.clone());
            }
        }
        last_total_items = total_items;
        total_items = possible.iter().map(|i| i.len()).sum();
    }
    println!("Possible: {:?}", possible);
    println!("Ticket: {:?}", &my_ticket);
}