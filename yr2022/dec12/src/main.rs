use ::std::collections::VecDeque;
use ::std::env::args;
use ::std::fs::read_to_string;
use ::std::rc::Rc;

// long time, only partly because of kids
// runtime depth-first (release) with `while let Some(p) = q.prev {`: 6.8s
// runtime depth-first (release): 0.066s
// runtime breadth-first (release): 0.034s

fn main() {
    let data = read_to_string("data.txt").unwrap();
    let args = args().collect::<Vec<_>>();
    let is_depth_first = if let Some(arg) = args.get(1) {
        "df" == arg
    } else {
        false
    };
    println!("is_depth_first = {is_depth_first}  ({:?})", args.get(1));
    let (start, _end, grid, min_costs) = solve_costs(&data, is_depth_first);
    //show_cost_grid(&min_costs);
    println!("A: {}", part_a(&min_costs, start));
    println!("B: {}", part_b(&min_costs, &grid));
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

fn part_a(costs: &[Vec<usize>], start: Pos) -> usize {
    costs[start.x][start.y]
}

fn part_b(costs: &[Vec<usize>], grid: &[Vec<u8>]) -> usize {
    let mut lowest_cost = usize::MAX;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 0 {
                if costs[x][y] < lowest_cost {
                    lowest_cost = costs[x][y];
                }
            }
        }
    }
    lowest_cost
}

fn solve_costs(data: &str, is_depth_first: bool) -> (Pos, Pos, Vec<Vec<u8>>, Vec<Vec<usize>>) {
    let (start, end, grid) = parse(data);
    let mut min_costs = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    if is_depth_first {
        search_depth_first(Step { pos: end, cost: 0, prev: None }, &grid, &mut min_costs);
    } else {
        search_breadth_first(end, &grid, &mut min_costs);
    }
    (start, end, grid, min_costs)
}

fn search_breadth_first(start: Pos, grid: &[Vec<u8>], min_cost: &mut [Vec<usize>]) {
    let mut queue = VecDeque::with_capacity(grid.len());
    queue.push_back((start, 0, grid[start.x][start.y].saturating_sub(1)));
    while let Some((cur, cur_cost, max_height)) = queue.pop_front() {
        if grid[cur.x][cur.y] < max_height {
            continue
        }
        if cur_cost >= min_cost[cur.x][cur.y] {
            continue
        }
        min_cost[cur.x][cur.y] = cur_cost;
        let next_min_height = grid[cur.x][cur.y].saturating_sub(1);
        let next_cost = cur_cost + 1;
        if cur.x < grid.len() - 1 {
            queue.push_back((Pos { x: cur.x + 1, y: cur.y }, next_cost, next_min_height));
        };
        if cur.x > 0 {
            queue.push_back((Pos { x: cur.x - 1, y: cur.y }, next_cost, next_min_height));
        };
        if cur.y < grid[0].len() - 1 {
            queue.push_back((Pos { x: cur.x, y: cur.y + 1 }, next_cost, next_min_height));
        };
        if cur.y > 0 {
            queue.push_back((Pos { x: cur.x, y: cur.y - 1 }, next_cost, next_min_height));
        };
    }
}

fn search_depth_first(cur: Step, grid: &[Vec<u8>], min_cost: &mut [Vec<usize>]) {
    let cur_pos = cur.pos;
    if cur.cost >= min_cost[cur_pos.x][cur_pos.y] {
        return;
    }
    min_cost[cur_pos.x][cur_pos.y] = cur.cost;
    let next_cost = cur.cost + 1;
    let cur_ref = Rc::new(cur);
    let next_min_height = grid[cur_pos.x][cur_pos.y].saturating_sub(1);
    if cur_pos.x < grid.len() - 1 && grid[cur_pos.x + 1][cur_pos.y] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x + 1, y: cur_pos.y }, cost: next_cost, prev: Some(cur_ref.clone()) };
        search_depth_first(next, grid, min_cost);
    };
    if cur_pos.x > 0 && grid[cur_pos.x - 1][cur_pos.y] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x - 1, y: cur_pos.y }, cost: next_cost, prev: Some(cur_ref.clone()) };
        search_depth_first(next, grid, min_cost);
    };
    if cur_pos.y < grid[0].len() - 1 && grid[cur_pos.x][cur_pos.y + 1] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y + 1 }, cost: next_cost, prev: Some(cur_ref.clone()) };
        search_depth_first(next, grid, min_cost);
    };
    if cur_pos.y > 0 && grid[cur_pos.x][cur_pos.y - 1] >= next_min_height {
        let next = Step { pos: Pos { x: cur_pos.x, y: cur_pos.y - 1 }, cost: next_cost, prev: Some(cur_ref) };
        search_depth_first(next, grid, min_cost);
    };
}

fn show_cost_grid(costs: &[Vec<usize>]) {
    for row in costs {
        for cell in row {
            if *cell == usize::MAX {
                print!("*** ")
            } else {
                print!("{cell: >3} ")
            }
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
            assert!(val < 26);
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