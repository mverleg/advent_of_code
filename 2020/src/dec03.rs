use ::std::fs::read_to_string;

pub fn dec03a() {
    let mut count = 0;
    let mut col = 4;
    let content = read_to_string("dec03.txt").unwrap();
    let mut lines = content.lines();
    let col_cnt = lines.next().unwrap().len();
    let mut line_cnt = 1;
    println!("{}", col_cnt);  //TODO @mark: TEMPORARY! REMOVE THIS!
    for line in lines {
        assert!(col_cnt == line.len());
        line_cnt += 1;
        let prev = (col + col_cnt - 1) % col_cnt;
        let maybe_tree = line.chars().skip(prev).next().unwrap();
        if maybe_tree == '#' {
            count += 1;
        }
        println!("{} / {} = {}", line, prev + 1, maybe_tree);  //TODO @mark: TEMPORARY! REMOVE THIS!
        if maybe_tree == '#' {
            println!("{}# {}", " ".repeat(prev), count);
        } else {
            println!("{}^ {}", " ".repeat(prev), count);
        }
        col += 3;
    }
    assert!(count < line_cnt);
    println!("{}", count)
}
