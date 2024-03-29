use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
use ::std::fs::read_to_string;

// 33:35 :(

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("{}", part_a(&data));
    println!("{}", part_b(&data));
    // wrong: 41609574 (root), 21756443
}


fn part_a(data: &str) -> usize {
    let dir_total_sizes = get_dir_sizes(data);
    dir_total_sizes.into_iter()
        .map(|(_, size)| size)
        .filter(|size| *size <= 100000)
        .sum()
}

fn part_b(data: &str) -> usize {
    let dir_total_sizes = get_dir_sizes(data);
    let space_use = *dir_total_sizes.get("/").expect("no root");
    assert!(space_use < 70000000);
    let space_avail = 70000000 - space_use;
    let space_need = if space_avail < 30000000 {
        30000000 - space_avail
    } else {
        30000000
    };
    let mut large_enough_sizes = dir_total_sizes.into_iter()
        .map(|(_, size)| size)
        .filter(|size| *size >= space_need)
        .collect::<Vec<_>>();
    large_enough_sizes.sort();
    large_enough_sizes[0]
}

fn get_dir_sizes(data: &str) -> HashMap<String, usize> {
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
                continue  // do nothing
            }
            assert!(line.chars().next().unwrap().is_digit(10), "not digit: {line}");
            let (size, _) = line.split_once(' ').expect("expected space in dir listing entry");
            let size: usize = size.parse().unwrap();
            for i in 0..current_path.len() {
                let mut subpath = "/".to_owned();
                subpath.push_str(&current_path[1..i + 1].join("/"));
                match dir_total_sizes.entry(subpath) {
                    Entry::Occupied(mut entry) => *entry.get_mut() += size,
                    Entry::Vacant(entry) => { entry.insert(size); },
                }
            }
        }
    }
    dir_total_sizes
}
