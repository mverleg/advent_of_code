use ::std::cmp::Ordering;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::fs::read_to_string;

//

fn main() {
    let data = parse(&read_to_string("data.txt").unwrap());
    for (entry1, entry2) in &data {
        println!("{}\n{}\n", entry1, entry2);
    }
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
}

fn part_a(data: &[(Entry, Entry)]) -> usize {
    let mut index_sum = 0;
    for (i, (entry1, entry2)) in data.iter().enumerate() {
        if determine_ordering(entry1.as_list(), entry2.as_list()) == Ordering::Less {
            //println!("i={}", i+1);
            index_sum += i + 1
        }
    }
    index_sum
}

fn part_b(data: &[(Entry, Entry)]) -> usize {
    use Entry::*;
    let mut flat = vec![List(vec![List(vec![Int(2)])]), List(vec![List(vec![Int(6)])])];
    for (entry1, entry2) in data {
        flat.push(entry1.clone());
        flat.push(entry2.clone());
    }
    flat.sort_by(determine_ordering);
    dbg!(&flat);
    todo!()
}

#[derive(Debug, Clone)]
enum Entry {
    List(Vec<Entry>),
    Int(usize),
}

impl Entry {
    fn as_list(&self) -> &[Entry] {
        match self {
            Entry::List(li) => li,
            Entry::Int(_) => panic!("cannot call as_list on nr"),
        }
    }
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
    let mut pairs = vec![];
    loop {
        pairs.push((
            parse_entry(&to_chrs(lines.next())).1,
            parse_entry(&to_chrs(lines.next())).1,
        ));
        match lines.next() {
            Some("") => {},
            Some(txt) => unimplemented!("expected empty line, got {}", txt),
            None => break,
        }
    }
    pairs
}

fn to_chrs(line: Option<&str>) -> Vec<char> {
    line
        .expect("unexpected end of lines in middle of entry").chars()
        .collect::<Vec<_>>()
}

fn parse_entry(line: &[char]) -> (usize, Entry) {
    assert!(line[0] == '[');
    let mut i = 1;
    let mut list = vec![];
    let mut cur_nr = String::new();
    while i < line.len() {
        if line[i].is_digit(10) {
            while line[i].is_digit(10) {
                cur_nr.push(line[i]);
                i += 1;
            }
            list.push(Entry::Int(cur_nr.parse::<usize>().unwrap()));
            cur_nr.clear();
        } else if line[i] == '[' {
            let (char_read_count, sub_list) = parse_entry(&line[i..]);
            i += char_read_count;
            list.push(sub_list);
        } else if line[i] != ']' {
            unimplemented!("'{}' as {}", line[i], i)
        }
        if line[i] == ']' {
            break
        }
        assert!(line[i] == ',');
        i += 1;
    }
    (i + 1, Entry::List(list))
}

fn determine_ordering(entry1: &[Entry], entry2: &[Entry]) -> Ordering {
    use Entry::*;
    let mut j = 0;
    loop {
        //println!("j={j} compare {} and {}",
        //        entry1.get(j).map(|s| s.to_string()).unwrap_or_else(|| "empty".to_string()),
        //        entry2.get(j).map(|s| s.to_string()).unwrap_or_else(|| "empty".to_string()));
        let cmp = match (entry1.get(j), entry2.get(j)) {
            (Some(List(li1)), Some(List(li2))) => determine_ordering(li1, li2),
            (Some(List(li1)), Some(Int(nr2))) => determine_ordering(li1, &[Int(*nr2)]),
            (Some(Int(nr1)), Some(List(li2))) => determine_ordering(&[Int(*nr1)], li2),
            (Some(Int(nr1)), Some(Int(nr2))) => nr1.cmp(nr2),
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        };
        if cmp != Ordering::Equal {
            //println!("  neq: {:?}", cmp);
            return cmp
        }
        j += 1;
    }
    unimplemented!()
}