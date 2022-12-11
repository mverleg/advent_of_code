use ::std::fs::read_to_string;
use itertools::Itertools;

use ::evalexpr::eval_int;

//

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
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
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let mut monkeys = parse(data);
    let mut items = monkeys.iter()
        .map(|monkey| monkey.init_items.clone())
        .collect::<Vec<_>>();
    for round in 0 .. 20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let current_items = items[i].clone();
            items[i].clear();
            for old_item in current_items {
                let expr = monkey.operation.replace("old", &old_item.to_string());
                let mut new_item: usize = eval_int(&expr)
                    .expect("could not evaluate").try_into().unwrap();
                new_item /= 3;
                let to = if new_item % monkey.test_mod == 0 {
                    monkey.monkey_if_true
                } else {
                    monkey.monkey_if_false
                };
                items[to].push(new_item);
            }
        }
        println!("\nround {round}");
        for (i, items) in items.iter().enumerate() {
            println!("Monkey {i}: {}", items.iter().join(", "))
        }
    }
    todo!()
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
            .parse::<usize>().expect("test true nan");;
        let monkey_if_false = word_after("    If false: throw to monkey ", lines.next())
            .parse::<usize>().expect("test false nan");;
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
