use ::std::fs::read_to_string;
use std::collections::HashSet;

pub fn dec04a() {
    println!("{}", do_batch());
}

fn do_batch() -> u32 {
    let mut count = 0;
    let mut current = String::with_capacity(1024);
    for line in read_to_string("dec04ex.txt").unwrap().lines() {
        if line.is_empty() {
            if do_entry(&current) {
                count += 1;
            }
            current.clear();
            continue
        }
        current.push_str(" ");
        current.push_str(line);
    }
    count
}

fn do_entry(inp: &str) -> bool {
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
    expected.is_subset(&keys)
}
