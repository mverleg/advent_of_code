use ::std::fs::read_to_string;
use std::rc::Rc;

//

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
    run(data, false)
}

fn part_b(data: &str) -> usize {
    run(data, true)
}

fn run(data: &str, is_b: bool) -> usize {
    let (start, end, grid) = parse(data);
    let mut min_cost = vec![vec![usize::MAX; grid.len()]; grid[0].len()];
    astar(Step { pos: start, cost: 0, prev: None }, end, &grid, &mut min_cost);
    dbg!(&grid);
    todo!();
}

fn astar(cur: Step, end: Pos, grid: &[Vec<u8>], min_cost: &mut [Vec<usize>]) -> Step {
    eprintln!("{}, {}: c{} h{}", cur.pos.x, cur.pos.y, cur.cost, grid[cur.pos.x][cur.pos.y]);
    if cur.pos == end {
        return cur
    }
    let cur_pos = cur.pos;
    let next_cost = cur.cost + 1;
    let cur_ref = Rc::new(cur);
    let next_max_height = grid[cur_pos.x][cur_pos.y] + 1;
    let mut moves = vec![];
    if cur_pos.x < grid.len() - 1 && grid[cur_pos.x + 1][cur_pos.y] <= next_max_height {
        let next = Step { pos: Pos { x: cur_pos.x + 1, y: cur_pos.y }, cost: next_cost + 1, prev: Some(cur_ref.clone()), };
        moves.push(astar(next, end, grid, min_cost));
        eprintln!("  x+");
    } else {
        if cur_pos.x < grid.len() - 1 { eprintln!("  x+ NOT h={}", grid[cur_pos.x + 1][cur_pos.y]) };
    };
    if cur_pos.x > 0 && grid[cur_pos.x - 1][cur_pos.y] <= next_max_height {
        let next = Step { pos: Pos { x: cur_pos.x - 1, y: cur_pos.y }, cost: next_cost + 1, prev: Some(cur_ref.clone()), };
        moves.push(astar(next, end, grid, min_cost));
        eprintln!("  x-");
    } else {
        if cur_pos.x > 0 { eprintln!("  x- NOT h={}", grid[cur_pos.x - 1][cur_pos.y]) };
    };
    if cur_pos.y < grid[0].len() - 1 && grid[cur_pos.x][cur_pos.y + 1] <= next_max_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y + 1 }, cost: next_cost + 1, prev: Some(cur_ref.clone()), };
        moves.push(astar(next, end, grid, min_cost));
        eprintln!("  y+");
    } else {
        if cur_pos.y < grid[0].len() - 1 { eprintln!("  y+ NOT h={}", grid[cur_pos.x][cur_pos.y + 1]) };
    };
    if cur_pos.y > 0 && grid[cur_pos.x][cur_pos.y - 1] <= next_max_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y - 1 }, cost: next_cost + 1, prev: Some(cur_ref), };
        moves.push(astar(next, end, grid, min_cost));
        eprintln!("  y-");
    } else {
        if cur_pos.y > 0 { eprintln!("  y- NOT h={}", grid[cur_pos.x][cur_pos.y - 1]) };
    };
    moves.into_iter().min_by_key(|step| step.cost).expect("did not find exit")
}

fn parse(data: &str) -> (Pos, Pos, Vec<Vec<u8>>) {
    let mut start = Pos { x: usize::MAX, y: usize::MAX };
    let mut end = Pos { x: usize::MAX, y: usize::MAX };
    let mut grid = vec![];
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
            assert!(val >=0 && val < 26);
            row.push(val as u8);
        }
        grid.push(row);
    }
    assert!(start.x < usize::MAX);
    assert!(start.y < usize::MAX);
    assert!(end.x < usize::MAX);
    assert!(end.y < usize::MAX);
    (start, end, grid)
}