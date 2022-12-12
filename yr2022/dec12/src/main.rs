use ::std::fs::read_to_string;
use ::std::rc::Rc;

// long time, only partly because of kids

fn main() {
    let data = read_to_string("data.txt").unwrap();
    println!("A: {}", part_a(&data));
    println!("B: {}", part_b(&data));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Step {
    pos: Pos,
    cost: usize,
    prev: Option<Rc<Step>>,
}

fn part_a(data: &str) -> usize {
    // intentionally invert start and end
    let (end, start, grid) = parse(data);
    let mut min_cost = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    search(Step { pos: end, cost: 0, prev: None }, &grid, &mut min_cost);
    show_cost_grid(&min_cost);
    // let mut step_count = 0;
    // while let Some(step) = path.prev {
    //     print!("{},{}; ", step.pos.x, step.pos.y);
    //     path = (*step).clone();
    //     step_count += 1;
    // }
    // println!("");
    //step_count
    todo!()
}

fn part_b(data: &str) -> usize {
    let (_, start, grid) = parse(data);
    let mut min_cost = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut path = search(Step { pos: start, cost: 0, prev: None }, &grid, &mut min_cost);
    min_cost;
    todo!()
}

fn search(cur: Step, grid: &[Vec<u8>], min_cost: &mut [Vec<usize>]) {
    //eprintln!("{}, {}: c{} h{}", cur.pos.x, cur.pos.y, cur.cost, grid[cur.pos.x][cur.pos.y]);
    // if cur.pos == end {
    //     return;
    // }
    let cur_pos = cur.pos;
    let mut q = cur.clone();
    while let Some(p) = q.prev {
        //eprint!("{},{}; ", p.pos.x, p.pos.y);
        q = (*p).clone();
    }
    //eprintln!("!!! {},{}", cur_pos.x, cur_pos.y); //TODO @mark: TEMPORARY! REMOVE THIS!
    if cur.cost >= min_cost[cur_pos.x][cur_pos.y] {
        //eprintln!("{}, {}  EE", cur_pos.x, cur_pos.y);
        return;
    }
    min_cost[cur_pos.x][cur_pos.y] = cur.cost;
    let next_cost = cur.cost + 1;
    let cur_ref = Rc::new(cur);
    let next_min_height = grid[cur_pos.x][cur_pos.y].saturating_sub(1);
    if cur_pos.x < grid.len() - 1 && grid[cur_pos.x + 1][cur_pos.y] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x + 1, y: cur_pos.y }, cost: next_cost + 1, prev: Some(cur_ref.clone()) };
        //eprintln!("{}, {}  x+", cur_pos.x, cur_pos.y);
        search(next, grid, min_cost);
        //} else {
        //if cur_pos.x < grid.len() - 1 { eprintln!("{}, {}  x+ NOT h={}", cur_pos.x, cur_pos.y, grid[cur_pos.x + 1][cur_pos.y]) };
    };
    if cur_pos.x > 0 && grid[cur_pos.x - 1][cur_pos.y] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x - 1, y: cur_pos.y }, cost: next_cost + 1, prev: Some(cur_ref.clone()) };
        //eprintln!("{}, {}  x-", cur_pos.x, cur_pos.y);
        search(next, grid, min_cost);
        //} else {
        //if cur_pos.x > 0 { eprintln!("{}, {}  x- NOT h={}", cur_pos.x, cur_pos.y, grid[cur_pos.x - 1][cur_pos.y]) };
    };
    if cur_pos.y < grid[0].len() - 1 && grid[cur_pos.x][cur_pos.y + 1] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y + 1 }, cost: next_cost + 1, prev: Some(cur_ref.clone()) };
        //eprintln!("{}<{}", cur_pos.y, grid[0].len() - 1);  //TODO @mark: TEMPORARY! REMOVE THIS!
        //eprintln!("{}, {}  y+", cur_pos.x, cur_pos.y);
        search(next, grid, min_cost);
        //} else {
        //if cur_pos.y < grid[0].len() - 1 { eprintln!("{}, {}  y+ NOT h={}", cur_pos.x, cur_pos.y, grid[cur_pos.x][cur_pos.y + 1]) };
    };
    if cur_pos.y > 0 && grid[cur_pos.x][cur_pos.y - 1] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y - 1 }, cost: next_cost + 1, prev: Some(cur_ref) };
        //eprintln!("{}, {}  y-", cur_pos.x, cur_pos.y);
        search(next, grid, min_cost);
        //} else {
        //if cur_pos.y > 0 { eprintln!("{}, {}  y- NOT h={}", cur_pos.x, cur_pos.y, grid[cur_pos.x][cur_pos.y - 1]) };
    };
}

fn show_cost_grid(costs: &[Vec<usize>]) {
    for row in costs {
        for cell in row {
            print!("{cell: >3} ")
        }
        println!("");
    }
}

fn parse(data: &str) -> (Pos, Pos, Vec<Vec<u8>>) {
    let mut start = Pos { x: usize::MAX, y: usize::MAX };
    let mut end = Pos { x: usize::MAX, y: usize::MAX };
    let mut grid: Vec<Vec<u8>> = vec![];
    for (x, line) in data.lines().enumerate() {
        let mut row = vec![];
        for (y, chr) in line.chars().enumerate() {
            let val = if chr == 'S' {
                start = Pos { x, y };
                0
            } else if chr == 'E' {
                end = Pos { x, y };
                25
            } else {
                chr.to_digit(36).expect("not a b36 digit") - 10
            };
            assert!(val >= 0 && val < 26);
            row.push(val as u8);
        }
        if !grid.is_empty() {
            assert_eq!(grid[0].len(), row.len());
        }
        grid.push(row);
    }
    assert!(start.x < usize::MAX);
    assert!(start.y < usize::MAX);
    assert!(end.x < usize::MAX);
    assert!(end.y < usize::MAX);
    (start, end, grid)
}