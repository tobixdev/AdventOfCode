use regex::Regex;

#[derive(Debug, Clone)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

lazy_static! {
    static ref CM_REGEX: Regex = Regex::new(r"^(\d+)cm$").unwrap();
    static ref IN_REGEX: Regex = Regex::new(r"^(\d+)in$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"[0-9]{9}").unwrap();
}

impl Passport {
    pub fn from(string: &String) -> Passport {
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        string.split(" ").filter(|p| *p != "").for_each(|pair| {
            let key = pair.split(":").nth(0).unwrap();
            let value = pair.split(":").nth(1).unwrap();

            if key == "byr" {
                passport.byr = Some(value.to_string())
            } else if key == "iyr" {
                passport.iyr = Some(value.to_string())
            } else if key == "eyr" {
                passport.eyr = Some(value.to_string())
            } else if key == "hgt" {
                passport.hgt = Some(value.to_string())
            } else if key == "hcl" {
                passport.hcl = Some(value.to_string())
            } else if key == "ecl" {
                passport.ecl = Some(value.to_string())
            } else if key == "pid" {
                passport.pid = Some(value.to_string())
            } else if key == "cid" {
                passport.cid = Some(value.to_string())
            }
        });

        return passport;
    }

    pub fn is_valid_stage1(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    pub fn is_valid_stage2(&self) -> bool {
        is_within_range(&self.byr, 1920, 2002) &&
        is_within_range(&self.iyr, 2010, 2020) &&
        is_within_range(&self.eyr, 2020, 2030) &&
        self.is_valid_height() &&
        matches_exactly(&self.hcl, &HCL_REGEX) &&
        matches_exactly(&self.ecl, &ECL_REGEX) &&
        matches_exactly(&self.pid, &PID_REGEX)
    }

    fn is_valid_height(&self) -> bool {
        return match &self.hgt {
            Some(height) => {
                if CM_REGEX.is_match(&height) {
                    let height = CM_REGEX.captures(&height).unwrap()[1].parse().unwrap();
                    return 150 <= height && height <= 190;
                }
                if IN_REGEX.is_match(&height) {
                    let height = IN_REGEX.captures(&height).unwrap()[1].parse().unwrap();
                    return 59 <= height && height <= 76;
                }
                return false;
            },
            None => false,
        }
    }
}

fn is_within_range(value: &Option<String> , from: i32, to: i32) -> bool {
    match value {
        Some(value) => {
            let parsed = value.parse::<i32>().unwrap();
            return from <= parsed || parsed <= to;
        },
        None => false
    }
}

fn matches_exactly(value: &Option<String>, regex: &Regex) -> bool {
    match value {
        Some(value) => {
            return regex.is_match(value);
        },
        None => false
    }
}