use ::std::fs::read_to_string;
use std::collections::HashSet;

pub fn dec04a() {
    println!("4a: {}", do_batch(has_all_fields));
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
            if !value.parse::<u32>().map(|yr| yr >= 1920 && yr <= 2002).unwrap_or_else(|_| false) {
                return false
            }
            //println!("valid byr: {}", &value);
        } else if field.starts_with("iyr") {
            if !value.parse::<u32>().map(|yr| yr >= 2010 && yr <= 2020).unwrap_or_else(|_| false) {
                return false
            }
            //println!("valid iyr: {}", &value);
        } else if field.starts_with("eyr") {
            return false;  //TODO @mark:
        } else if field.starts_with("hgt") {
            return false;  //TODO @mark:
        } else if field.starts_with("hcl") {
            return false;  //TODO @mark:
        } else if field.starts_with("ecl") {
            return false;  //TODO @mark:
        } else if field.starts_with("pid") {
            return false;  //TODO @mark:
        } else if field.starts_with("cid") {
            continue;
        } else {
            panic!("{}", field);
        }
    }
    true
}
