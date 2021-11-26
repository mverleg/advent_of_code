use ::std::fs::read_to_string;
use ::std::collections::HashSet;
use ::lazy_static::lazy_static;
use ::regex::Regex;

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"^(1[5678]\d\s*cm|19[0-3]\s*cm|59\s*in|6\d\s*in|7[0-6]\s*in)$").unwrap();
    static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORTNR_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

pub fn dec04a() {
    println!("4a: {}", do_batch(has_all_fields));
}

pub fn dec04b() {
    println!("4b: {}", do_batch(all_fields_valid));
}

/// Make sure the input ends in newlines
fn do_batch(validate: fn(&str) -> bool) -> u32 {
    let mut count = 0;
    let mut current = String::with_capacity(1024);
    for line in read_to_string("dec04.txt").unwrap().lines() {
        if line.is_empty() {
            if validate(&current) {
                count += 1;
            }
            current.clear();
            continue;
        }
        current.push_str(" ");
        current.push_str(line);
    }
    count
}

fn has_all_fields(inp: &str) -> bool {
    let mut expected = HashSet::new();
    expected.insert("byr".to_owned());
    expected.insert("iyr".to_owned());
    expected.insert("eyr".to_owned());
    expected.insert("hgt".to_owned());
    expected.insert("hcl".to_owned());
    expected.insert("ecl".to_owned());
    expected.insert("pid".to_owned());
    let keys = inp.split(' ')
        .map(|s| s.chars().take(3).collect::<String>())
        .filter(|s| !s.is_empty())
        .collect::<HashSet<_>>();
    if expected.is_subset(&keys) {
        // println!("valid: {:?}", &keys);
        true
    } else {
        // println!("INvalid: {:?}", &keys);
        false
    }
}

fn all_fields_valid(inp: &str) -> bool {
    if !has_all_fields(inp) {
        return false;
    }
    for field in inp.split(' ') {
        if field.is_empty() {
            continue
        }
        let value = field.chars().skip(4).collect::<String>();
        if field.starts_with("byr") {
            if !is_year_in_inclusive_range(&value, 1920, 2002) {
                return false
            }
            //println!("valid byr: {}", &value);
        } else if field.starts_with("iyr") {
            if !is_year_in_inclusive_range(&value, 2010, 2020) {
                return false
            }
        } else if field.starts_with("eyr") {
            if !is_year_in_inclusive_range(&value, 2020, 2030) {
                return false
            }
        } else if field.starts_with("hgt") {
            if !HEIGHT_RE.is_match(&value) {
                //println!("INvalid hgt: {}", &value);
                return false;
            }
            //println!("valid hgt: {}", &value);
        } else if field.starts_with("hcl") {
            if !HAIR_RE.is_match(&value) {
                //println!("INvalid hcl: {}", &value);
                return false;
            }
            //println!("valid hcl: {}", &value);
        } else if field.starts_with("ecl") {
            if !EYE_RE.is_match(&value) {
                return false
            }
        } else if field.starts_with("pid") {
            if !PASSPORTNR_RE.is_match(&value) {
                return false
            }
        } else if field.starts_with("cid") {
            continue;
        } else {
            panic!("{}", field);
        }
    }
    true
}

fn is_year_in_inclusive_range(value: &str, min: u32, max: u32) -> bool {
    value.parse::<u32>().map(|yr| yr >= min && yr <= max).unwrap_or_else(|_| false)
}