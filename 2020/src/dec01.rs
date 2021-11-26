use ::std::fs::read_to_string;

#[test]
fn dec01a() {
    let res = run1();
    println!("{}", res);
}

#[test]
fn dec01b() {
    let res = run2();
    println!("{}", res);
}

fn run1() -> u32 {
    let goal = 2020;
    let yrs = read_sorted();
    let mut i2 = yrs.len() - 1;
    for yr1 in &yrs {
        //println!("compare {} and {}", yr1, yrs[i2]);
        while yrs[i2] + yr1 > goal {
            i2 -= 1;
        }
        if yrs[i2] + yr1 == goal {
            return yrs[i2] * yr1;
        }
    }
    panic!("unreachable if there is a solution")
}

fn run2() -> u32 {
    let goal = 2020;
    let yrs = read_sorted();
    for yr1 in yrs.iter().rev() {
        for yr2 in &yrs {
            if yr1 + yr2 > goal {
                break
            }
            for yr3 in &yrs {
                if yr1 + yr2 + yr3 > goal {
                    break
                }
                //println!("compare {}, {}, {}", yr1, yr2, yr3);
                if yr1 + yr2 + yr3 == goal {
                    return yr1 * yr2 * yr3;
                }
            }
        }
    }
    panic!("unreachable if there is a solution")
}

fn read_sorted() -> Vec<u32> {
    let mut yrs = read_to_string("../dec01.txt")
        .unwrap()
        .lines()
        .filter(|ln| !ln.is_empty())
        .map(|ln| ln.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    yrs.sort();
    yrs
}