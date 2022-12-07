use ::std::fs::read_to_string;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
}


fn part_a(data: &str) -> usize {
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut dir_total_sizes: HashMap<String, usize> = HashMap::new();
    let mut current_path = vec!["/"];
    for line in data.lines() {
        //eprintln!("> {line}");
        if line.starts_with("$ ") {
            let cmd = &line[2..];
            //eprintln!(">> |{cmd}|");
            if cmd == "cd /" {
                current_path.clear();
                current_path.push("/");
            } else if cmd == "cd .." {
                current_path.pop();
            } else if cmd.starts_with("cd ") {
                let (_, dir) = cmd.split_once(' ').unwrap();
                current_path.push(dir);
            } else if cmd == "ls" {
                // do nothing
            } else {
                panic!("unknown command on line {line}")
            }
        } else {
            if line.starts_with("dir") {
                continue
            }
            assert!(line.chars().next().unwrap().is_digit(10), "not digit: {line}");
            let (size, _) = line.split_once(' ').expect("expected space in dir listing entry");
            let size: usize = size.parse().unwrap();
            for i in 0 .. current_path.len() {
                let mut subpath = "/".to_owned();
                subpath.push_str(&current_path[1..i+1].join("/"));
                match dir_total_sizes.entry(subpath) {
                    Entry::Occupied(mut entry) => *entry.get_mut() += size,
                    Entry::Vacant(entry) => { entry.insert(size); },
                }
            }
        }
    }
    dbg!(&dir_total_sizes);
    dir_total_sizes.into_iter()
        .map(|(_, size)| size)
        .filter(|size| *size <= 100000)
        .sum()
}
