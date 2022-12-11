use ::std::fs::read_to_string;
use itertools::Itertools;

use ::evalexpr::eval_int;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
    // wrong B: 2499999999 was for test input (and wrong)
    // wrong B: 32393520315 (too high)
}
#[derive(Debug)]
struct Monkey {
    init_items: Vec<usize>,
    operation: String,
    test_mod: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

fn part_a(data: &str) -> usize {
    run(data, false, 20)
}

fn part_b(data: &str) -> usize {
    run(data, true, 10_000)
}

fn run(data: &str, is_b: bool, iter_count: usize) -> usize {
    let monkeys = parse(data);
    let mut items = monkeys.iter()
        .map(|monkey| monkey.init_items.clone())
        .collect::<Vec<_>>();
    let total_mod = monkeys.iter()
        .map(|monkey| monkey.test_mod)
        .reduce(|a, b| a * b).expect("no total mod");
    println!("total mod = {total_mod}");
    let mut inspection_counts = vec![0usize; monkeys.len()];
    for round in 0 .. iter_count {
        for (i, monkey) in monkeys.iter().enumerate() {
            let current_items = items[i].clone();
            items[i].clear();
            inspection_counts[i] += current_items.len();
            for old_item in current_items {
                let expr = monkey.operation.replace("old", &old_item.to_string());
                let mut new_item: usize = eval_int(&expr)
                    .expect("could not evaluate").try_into().unwrap();
                if is_b {
                    new_item %= total_mod;
                } else {
                    new_item /= 3;
                }
                let to = if new_item % monkey.test_mod == 0 {
                    monkey.monkey_if_true
                } else {
                    monkey.monkey_if_false
                };
                items[to].push(new_item);
            }
        }
        if !is_b {
            println!("\nround {round}");
            for (i, items) in items.iter().enumerate() {
                println!("Monkey {i} passed {} times: {}", inspection_counts[i], items.iter().join(", "))
            }
        }
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    println!("passes: {}", inspection_counts.iter().join(", "));
    inspection_counts[0] * inspection_counts[1]
}

fn parse(data: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut lines = data.lines();
    loop {
        match lines.next() {
            Some(line) => assert_eq!(line, &format!("Monkey {}:", monkeys.len())),
            None => return monkeys
        }
        let item_str = lines.next().expect("no items");
        assert!(item_str.starts_with("  Starting items: "));
        let items = item_str[18..].split(", ")
            .map(|item| item.parse::<usize>().expect("item nan"))
            .collect::<Vec<_>>();
        let op = word_after("  Operation: new = ", lines.next());
        let test_mod = word_after("  Test: divisible by ", lines.next())
            .parse::<usize>().expect("test mod nan");
        let monkey_if_true = word_after("    If true: throw to monkey ", lines.next())
            .parse::<usize>().expect("test true nan");
        let monkey_if_false = word_after("    If false: throw to monkey ", lines.next())
            .parse::<usize>().expect("test false nan");
        monkeys.push(Monkey {
            init_items: items,
            operation: op,
            test_mod,
            monkey_if_true,
            monkey_if_false,
        });
        match lines.next() {
            Some("") => {},
            Some(line) => panic!("unexpected: {line}"),
            None => return monkeys,
        }
    }
}

fn word_after(prefix: &str, line: Option<&str>) -> String {
    let str = line.unwrap_or_else(|| panic!("missing '{prefix}'"));
    assert!(str.starts_with(prefix));
    str[prefix.len()..].to_owned()
}
