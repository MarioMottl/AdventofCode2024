use anyhow::Result;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(grid: &[Vec<char>]) -> ((usize, usize), (usize, usize), usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end, rows, cols)
}

fn dijkstra(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    rows: usize,
    cols: usize,
) -> HashMap<(usize, usize, usize), usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    let start_state = (start.0, start.1, 1); // Start facing East

    heap.push(State {
        cost: 0,
        position: start,
        direction: 1,
    });
    visited.insert(start_state, 0);

    while let Some(State { cost, position, direction }) = heap.pop() {
        if visited.get(&(position.0, position.1, direction)).unwrap_or(&usize::MAX) < &cost {
            continue;
        }

        // Move forward
        let (dx, dy) = DIRECTIONS[direction];
        let new_position = ((position.0 as isize + dx) as usize, (position.1 as isize + dy) as usize);
        if new_position.0 < rows && new_position.1 < cols && grid[new_position.0][new_position.1] != '#' {
            let new_cost = cost + 1;
            if new_cost < *visited.get(&(new_position.0, new_position.1, direction)).unwrap_or(&usize::MAX) {
                visited.insert((new_position.0, new_position.1, direction), new_cost);
                heap.push(State {
                    cost: new_cost,
                    position: new_position,
                    direction,
                });
            }
        }

        // Turn left or right
        for &new_direction in &[(direction + 3) % 4, (direction + 1) % 4] {
            let new_cost = cost + 1000;
            if new_cost < *visited.get(&(position.0, position.1, new_direction)).unwrap_or(&usize::MAX) {
                visited.insert((position.0, position.1, new_direction), new_cost);
                heap.push(State {
                    cost: new_cost,
                    position,
                    direction: new_direction,
                });
            }
        }
    }

    visited
}

fn backtrack_shortest_paths(
    grid: &[Vec<char>],
    visited: &HashMap<(usize, usize, usize), usize>,
    end: (usize, usize),
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let min_end_cost = (0..4)
        .filter_map(|d| visited.get(&(end.0, end.1, d)))
        .min()
        .cloned()
        .unwrap_or(usize::MAX);

    let mut on_shortest_path = HashSet::new();
    let mut queue = VecDeque::new();
    for direction in 0..4 {
        let end_state = (end.0, end.1, direction);
        if let Some(&cost) = visited.get(&end_state) {
            if cost == min_end_cost {
                on_shortest_path.insert(end_state);
                queue.push_back(end_state);
            }
        }
    }

    while let Some((cx, cy, cd)) = queue.pop_front() {
        let current_cost = visited[&(cx, cy, cd)];

        // Backward for forward moves
        let (dx, dy) = DIRECTIONS[cd];
        let (px, py) = ((cx as isize - dx) as usize, (cy as isize - dy) as usize);
        if px < rows && py < cols && grid[px][py] != '#' {
            let prev_cost = current_cost - 1;
            if prev_cost >= 0 {
                let prev_state = (px, py, cd);
                if let Some(&cost) = visited.get(&prev_state) {
                    if cost == prev_cost && !on_shortest_path.contains(&prev_state) {
                        on_shortest_path.insert(prev_state);
                        queue.push_back(prev_state);
                    }
                }
            }
        }

        // Backward for turns
        let turn_cost = current_cost - 1000;
        if turn_cost >= 0 {
            for &prev_direction in &[(cd + 3) % 4, (cd + 1) % 4] {
                let prev_state = (cx, cy, prev_direction);
                if let Some(&cost) = visited.get(&prev_state) {
                    if cost == turn_cost && !on_shortest_path.contains(&prev_state) {
                        on_shortest_path.insert(prev_state);
                        queue.push_back(prev_state);
                    }
                }
            }
        }
    }

    on_shortest_path.into_iter().map(|(x, y, _)| (x, y)).collect()
}

fn main() {
    let contents = read_input(INPUT_FILE).expect("Error reading file");
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let (start, end, rows, cols) = parse_grid(&grid);

    let start_timer = Instant::now();
    let visited = dijkstra(&grid, start, end, rows, cols);
    let shortest_path_tiles = backtrack_shortest_paths(&grid, &visited, end, rows, cols);
    let duration = start_timer.elapsed();
    println!("Part2 sitting spots: {}", shortest_path_tiles.len());
    println!("Part2 took: {:?}", duration);
}
