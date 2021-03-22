use regex::Regex;

const FIELDS: [&str;7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

pub struct Passport<'a> {
    fields: Vec<(&'a str, &'a str)>
}

impl Passport<'_> {
    
    pub fn new(passport: &str) -> Result<Passport, &'static str> {
        let mut fields = Vec::new();

        for field in passport.split_whitespace() {
            let mut params = field.trim().split(':');
            if let Some(key) = params.next() {
                if let Some(value) = params.next() {
                    fields.push((key, value));
                } else {
                    return Err("Invalid param: no value");
                }
            } else {
                return Err("Invalid param: no key");
            }
        }

        Ok(Passport{ fields })
    }

    pub fn is_complete(&self) -> bool {
        FIELDS
            .iter()
            .filter(|x| {
                self.fields
                    .iter()
                    .filter(|y| y.0 == **x)
                    .count() == 0
            })
            .count() == 0
    }

    pub fn is_valid(&self) -> bool {
        let hair_regex = Regex::new("^#[a-fA-F0-9]{6}$").unwrap();
        let eye_regex = Regex::new("^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
        let pid_regex = Regex::new("^[0-9]{9}$").unwrap();

        if !self.is_complete() {
            false
        } else {
            for (key, value) in self.fields.iter() {
                match *key {
                    "byr" => {
                        let year: isize = value.parse().expect("Failed parsing year");
                        if year < 1920 || year > 2002 { return false; }
                    },
                    "iyr" => {
                        let year: isize = value.parse().expect("Failed parsing year");
                        if year < 2010 || year > 2020 { return false; }
                    },
                    "eyr" => {
                        let year: isize = value.parse().expect("Failed parsing year");
                        if year < 2020 || year > 2030 { return false; }
                    },
                    "hgt" => {
                        let mut value = String::from(*value);
                        if value.ends_with("cm") {
                            value.pop();
                            value.pop();
                            let height: isize = value.parse().expect("Failed parsing height");
                            if height < 150 || height > 193 { return false; }
                        } else if value.ends_with("in") {
                            value.pop();
                            value.pop();
                            let height: isize = value.parse().expect("Failed parsing height");
                            if height < 59 || height > 76 { return false; }
                        } else { return false; }
                    },
                    "hcl" => {
                        if !hair_regex.is_match(value) { return false; }
                    },
                    "ecl" => {
                        if !eye_regex.is_match(value) { return false; }
                    },
                    "pid" => {
                        if !pid_regex.is_match(value) { return false; }
                    },
                    _ => {},
                };
                
            }
            true
        }
    }
}
