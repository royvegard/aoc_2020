use aoc_runner_derive::aoc;

#[derive(Default)]
struct Passport {
    byr: String, // Birth year
    iyr: String, // Issue year
    eyr: String, // Expiration year
    hgt: String, // Height
    hcl: String, // Hair color
    ecl: String, // Eye color
    pid: String, // Passport ID
}

impl Passport {
    fn is_valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.

        if self.byr.len() != 4
            || self.byr.parse::<u32>().unwrap() < 1920
            || self.byr.parse::<u32>().unwrap() > 2002
        {
            return false;
        }
        if self.iyr.len() != 4
            || self.iyr.parse::<u32>().unwrap() < 2010
            || self.iyr.parse::<u32>().unwrap() > 2020
        {
            return false;
        }
        if self.eyr.len() != 4
            || self.eyr.parse::<u32>().unwrap() < 2020
            || self.eyr.parse::<u32>().unwrap() > 2030
        {
            return false;
        }

        if self.hgt.ends_with("cm") {
            let height: u32 = self.hgt[..self.hgt.len() - 2].parse().unwrap();

            if height < 150 || height > 193 {
                return false;
            }
        } else if self.hgt.ends_with("in") {
            let height: u32 = self.hgt[..self.hgt.len() - 2].parse().unwrap();

            if height < 59 || height > 76 {
                return false;
            }
        } else {
            return false;
        }

        if !self.hcl.starts_with('#') {
            return false;
        }

        for c in self.hcl[1..].chars() {
            if !c.is_digit(16) {
                return false;
            }
        }

        match self.ecl.as_str() {
            "amb" => {}
            "blu" => {}
            "brn" => {}
            "gry" => {}
            "grn" => {}
            "hzl" => {}
            "oth" => {}
            _ => return false,
        }

        if self.pid.chars().count() != 9 {
            return false;
        }

        for c in self.pid.chars() {
            if !c.is_digit(10) {
                return false;
            }
        }

        true
    }
}

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut passports = vec![];
    let mut valid_passports: u32 = 0;

    for passport in input.split("\n\n").collect::<Vec<&str>>() {
        let mut byr: &str = "";
        let mut iyr: &str = "";
        let mut eyr: &str = "";
        let mut hgt: &str = "";
        let mut hcl: &str = "";
        let mut ecl: &str = "";
        let mut pid: &str = "";

        for field in passport.split_whitespace().collect::<Vec<&str>>() {
            let field = field.split(':').collect::<Vec<&str>>();

            match field[0] {
                "byr" => byr = field[1],
                "iyr" => iyr = field[1],
                "eyr" => eyr = field[1],
                "hgt" => hgt = field[1],
                "hcl" => hcl = field[1],
                "ecl" => ecl = field[1],
                "pid" => pid = field[1],
                _ => {}
            }
        }

        if byr.len() > 0
            && iyr.len() > 0
            && eyr.len() > 0
            && hgt.len() > 0
            && hcl.len() > 0
            && ecl.len() > 0
            && pid.len() > 0
        {
            valid_passports += 1;
        }

        passports.push(Passport {
            byr: byr.to_string(),
            iyr: iyr.to_string(),
            eyr: eyr.to_string(),
            hgt: hgt.to_string(),
            hcl: hcl.to_string(),
            ecl: ecl.to_string(),
            pid: pid.to_string(),
        });
    }
    valid_passports
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut passports = vec![];
    let mut valid_passports: u32 = 0;

    for passport in input.split("\n\n").collect::<Vec<&str>>() {
        let mut byr: &str = "";
        let mut iyr: &str = "";
        let mut eyr: &str = "";
        let mut hgt: &str = "";
        let mut hcl: &str = "";
        let mut ecl: &str = "";
        let mut pid: &str = "";

        for field in passport.split_whitespace().collect::<Vec<&str>>() {
            let field = field.split(':').collect::<Vec<&str>>();

            match field[0] {
                "byr" => byr = field[1],
                "iyr" => iyr = field[1],
                "eyr" => eyr = field[1],
                "hgt" => hgt = field[1],
                "hcl" => hcl = field[1],
                "ecl" => ecl = field[1],
                "pid" => pid = field[1],
                _ => {}
            }
        }

        passports.push(Passport {
            byr: byr.to_string(),
            iyr: iyr.to_string(),
            eyr: eyr.to_string(),
            hgt: hgt.to_string(),
            hcl: hcl.to_string(),
            ecl: ecl.to_string(),
            pid: pid.to_string(),
        });
    }

    for passport in passports {
        if passport.is_valid() {
            valid_passports += 1;
        }
    }
    valid_passports
}
