pub struct PasswordPolicy {
    pub from: usize,
    pub to: usize,
    pub character: char
}

pub struct PasswordLine {
    pub policy: PasswordPolicy,
    pub password: String
}

pub fn parse_line(line: &String) -> PasswordLine {
    return PasswordLine {
        policy: parse_policy(line),
        password: parse_password(line)
    }
}

fn parse_policy(line: &String) -> PasswordPolicy {
    let policy = line.split(": ").collect::<Vec<&str>>()[0];
    let counts = policy.split(" ").nth(0).unwrap();
    let character = policy.split(" ").nth(1).unwrap();
    return PasswordPolicy {
        from: counts.split("-").nth(0).unwrap().parse::<usize>().unwrap(),
        to: counts.split("-").nth(1).unwrap().parse::<usize>().unwrap(),
        character: character.chars().nth(0).unwrap()
    };
}

fn parse_password(line: &String) -> String {
    return line.split(": ").collect::<Vec<&str>>()[1].to_string();
}