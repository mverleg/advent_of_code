use ::std::fs::read_to_string;

fn main() {
    let res = run();
    println!("{}", res);
}

fn run() -> u32 {
    let goal = 2020;
    let mut yrs = read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    yrs.sort();
    let mut i2 = yrs.len() - 1;
    for yr1 in &yrs {
        println!("compare {} and {}", yr1, yrs[i2]);
        while yrs[i2] + yr1 > goal {
            i2 -= 1;
        }
        if yrs[i2] + yr1 == goal {
            return yrs[i2] * yr1;
        }
    }
    panic!("unreachable if there is a solution")
}
