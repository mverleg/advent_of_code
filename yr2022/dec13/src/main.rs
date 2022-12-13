use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::fs::read_to_string;

use ::itertools::Itertools;

//

fn main() {
    let data = parse(&read_to_string("data.txt").unwrap());
    for (entry1, entry2) in &data {
        println!("{}\n{}\n\n", entry1, entry2);
    }
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
}

fn part_a(data: &[(Entry, Entry)]) -> usize {
    todo!()
}

fn part_b(data: &[(Entry, Entry)]) -> usize {
    todo!()
}

#[derive(Debug, Clone)]
enum Entry {
    List(Vec<Entry>),
    Int(usize),
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Entry::List(li) => {
                write!(f, "[")?;
                let mut if_first = true;
                for nr in li {
                    if if_first {
                        if_first = false
                    } else {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", nr)?;
                }
                write!(f, "]")?;
            }
            Entry::Int(nr) => write!(f, "{}", nr)?,
        }
        Ok(())
    }
}

fn parse(data: &str) -> Vec<(Entry, Entry)> {
    let mut lines = data.lines();
    vec![(
        parse_entry(&to_chrs(lines.next())).1,
        parse_entry(&to_chrs(lines.next())).1,
    )]
}

fn to_chrs(line: Option<&str>) -> Vec<char> {
    line
        .expect("unexpected end of lines in middle of entry").chars()
        .collect::<Vec<_>>()
}

fn parse_entry(line: &[char]) -> (usize, Entry) {
    assert!(line[0] == '[');
    println!("parse_entry={}", line.iter().join(""));
    let mut i = 1;
    let mut list = vec![];
    let mut cur_nr = String::new();
    while i < line.len() {
        if line[i].is_digit(10) {
            println!("i={i} nr");
            while line[i].is_digit(10) {
                cur_nr.push(line[i]);
                i += 1;
            }
            if line[i] == ']' {
                break
            }
            assert!(line[i] == ',');
            i += 1;
            list.push(Entry::Int(cur_nr.parse::<usize>().unwrap()));
            cur_nr.clear();
        } else if line[i] == '[' {
            println!("i={i} list");
            let (new_i, sub_list) = parse_entry(&line[i..]);
            i = new_i;
            list.push(sub_list);
        } else {
            unimplemented!("{}", line[i])
        }
    }
    (i, Entry::List(list))
}
