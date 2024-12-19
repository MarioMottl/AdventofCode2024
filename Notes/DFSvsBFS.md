### Breadth-First Search (BFS)

**BFS Overview:**

- BFS explores the graph level by level.
- It starts at a given node (often called the root) and explores all its neighbors before moving on to the neighbors'
  neighbors.
- BFS uses a queue (FIFO) to keep track of the nodes to be explored next.

**BFS Characteristics:**

- **Traversal Order:** BFS visits nodes in the order of their distance from the starting node, exploring all nodes at
  the present depth level before moving on to nodes at the next depth level.
- **Data Structure:** BFS uses a queue to manage the nodes to be explored.
- **Shortest Path:** BFS can be used to find the shortest path in an unweighted graph because it explores all nodes at
  the current depth level before moving on to the next level.
- **Memory Usage:** BFS can consume a lot of memory if the graph is wide (i.e., nodes have many neighbors) because it
  needs to store all nodes at the current level in the queue.

**BFS Example:**

```rust
fn bfs(start: usize, graph: &Vec<Vec<usize>>) {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        println!("Visited node: {}", node);
        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
}
```

### Depth-First Search (DFS)

**DFS Overview:**

- DFS explores the graph by going as deep as possible along each branch before backtracking.
- It starts at a given node and explores as far as possible along each branch before backtracking to explore other
  branches.
- DFS uses a stack (LIFO) to keep track of the nodes to be explored next. This can be implemented using recursion (
  implicit stack) or an explicit stack.

**DFS Characteristics:**

- **Traversal Order:** DFS visits nodes by exploring as far down a branch as possible before backtracking. This means it
  goes deep into the graph before exploring siblings.
- **Data Structure:** DFS uses a stack to manage the nodes to be explored. This can be an explicit stack or the call
  stack in the case of recursion.
- **Path Finding:** DFS can be used to find paths in a graph, but it does not guarantee the shortest path in an
  unweighted graph.
- **Memory Usage:** DFS can be more memory-efficient than BFS if the graph is wide, as it only needs to store the
  current path in the stack.

**DFS Example:**

```rust
fn dfs(start: usize, graph: &Vec<Vec<usize>>) {
    let mut visited = vec![false; graph.len()];
    let mut stack = Vec::new();
    stack.push(start);

    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            println!("Visited node: {}", node);
            for &neighbor in &graph[node] {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
    }
}
```

### Key Differences

1. **Traversal Order:**
    - **BFS:** Explores nodes level by level.
    - **DFS:** Explores nodes by going as deep as possible along each branch before backtracking.

2. **Data Structure:**
    - **BFS:** Uses a queue (FIFO) to manage nodes.
    - **DFS:** Uses a stack (LIFO) to manage nodes, which can be implemented using recursion or an explicit stack.

3. **Path Finding:**
    - **BFS:** Guarantees the shortest path in an unweighted graph.
    - **DFS:** Does not guarantee the shortest path.

4. **Memory Usage:**
    - **BFS:** Can consume more memory if the graph is wide.
    - **DFS:** Can be more memory-efficient if the graph is wide, but can consume more memory if the graph is deep.

### Application in the Provided Code

In the provided Rust code, both BFS and DFS are used to explore regions in a 2D grid. The `flood_fill` function uses BFS
to explore all cells in a region, while the `bfs_count_sides` function uses BFS to count the unique sides of the region.

**BFS in `flood_fill`:**

- The `flood_fill` function uses a queue to explore all cells in a region level by level.
- It starts from a given cell and explores all its neighbors before moving on to the neighbors' neighbors.

**BFS in `bfs_count_sides`:**

- The `bfs_count_sides` function uses a queue to explore the perimeter cells and count the unique sides.
- It ensures that each side is counted only once by marking perimeter cells as seen.

By using BFS in both functions, the code ensures that all cells in a region and all perimeter cells are explored
efficiently.

## Example Usage

### Flood Fill Using BFS

**Breadth-First Search (BFS) Approach:**

- BFS explores the graph level by level.
- It uses a queue (FIFO) to keep track of the nodes to be explored next.
- BFS is useful when you want to explore all nodes at the present depth level before moving on to nodes at the next
  depth level.

**Example Implementation of Flood Fill Using BFS:**

```rust
use std::collections::VecDeque;

fn flood_fill_bfs(grid: &mut Vec<Vec<i32>>, start_row: usize, start_col: usize, new_value: i32) {
    let initial_value = grid[start_row][start_col];
    if initial_value == new_value {
        return;
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    while let Some((row, col)) = queue.pop_front() {
        if row >= grid.len() || col >= grid[0].len() || grid[row][col] != initial_value {
            continue;
        }

        grid[row][col] = new_value;

        for &(dr, dc) in &directions {
            let new_row = row.wrapping_add(dr as usize);
            let new_col = col.wrapping_add(dc as usize);
            if new_row < grid.len() && new_col < grid[0].len() {
                queue.push_back((new_row, new_col));
            }
        }
    }
}

fn main() {
    let mut grid = vec![
        vec![1, 1, 1, 2],
        vec![1, 1, 0, 2],
        vec![1, 0, 0, 2],
        vec![2, 2, 2, 2],
    ];

    let start_row = 0;
    let start_col = 0;
    let new_value = 3;

    flood_fill_bfs(&mut grid, start_row, start_col, new_value);

    for row in &grid {
        println!("{:?}", row);
    }
}
```

### Flood Fill Using DFS

**Depth-First Search (DFS) Approach:**

- DFS explores the graph by going as deep as possible along each branch before backtracking.
- It uses a stack (LIFO) to keep track of the nodes to be explored next. This can be implemented using recursion (
  implicit stack) or an explicit stack.
- DFS is useful when you want to explore as far down a branch as possible before backtracking.

**Example Implementation of Flood Fill Using DFS:**

```rust
fn flood_fill_dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, new_value: i32) {
    let initial_value = grid[row][col];
    if initial_value == new_value {
        return;
    }
    fill(grid, row, col, initial_value, new_value);
}

fn fill(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, initial_value: i32, new_value: i32) {
    if row >= grid.len() || col >= grid[0].len() || grid[row][col] != initial_value {
        return;
    }

    grid[row][col] = new_value;

    if row > 0 {
        fill(grid, row - 1, col, initial_value, new_value); // Up
    }
    if row < grid.len() - 1 {
        fill(grid, row + 1, col, initial_value, new_value); // Down
    }
    if col > 0 {
        fill(grid, row, col - 1, initial_value, new_value); // Left
        fn bfs(start: usize, graph: &Vec<Vec<usize>>) {
            let mut visited = vec![false; graph.len()];
            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited[start] = true;

            while let Some(node) = queue.pop_front() {
                println!("Visited node: {}", node);
                for &neighbor in &graph[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }
```

### Depth-First Search (DFS)

**DFS Overview:**

- DFS explores the graph by going as deep as possible along each branch before backtracking.
- It starts at a given node and explores as far as possible along each branch before backtracking to explore other
  branches.
- DFS uses a stack (LIFO) to keep track of the nodes to be explored next. This can be implemented using recursion (
  implicit stack) or an explicit stack.

**DFS Characteristics:**

- **Traversal Order:** DFS visits nodes by exploring as far down a branch as possible before backtracking. This means it
  goes deep into the graph before exploring siblings.
- **Data Structure:** DFS uses a stack to manage the nodes to be explored. This can be an explicit stack or the call
  stack in the case of recursion.
- **Path Finding:** DFS can be used to find paths in a graph, but it does not guarantee the shortest path in an
  unweighted graph.
- **Memory Usage:** DFS can be more memory-efficient than BFS if the graph is wide, as it only needs to store the
  current path in the stack.

**DFS Example:**

```rust
fn dfs(start: usize, graph: &Vec<Vec<usize>>) {
    let mut visited = vec![false; graph.len()];
    let mut stack = Vec::new();
    stack.push(start);

    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            println!("Visited node: {}", node);
            for &neighbor in &graph[node] {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
    }
}
```

### Key Differences

1. **Traversal Order:**
    - **BFS:** Explores nodes level by level.
    - **DFS:** Explores nodes by going as deep as possible along each branch before backtracking.

2. **Data Structure:**
    - **BFS:** Uses a queue (FIFO) to manage nodes.
    - **DFS:** Uses a stack (LIFO) to manage nodes, which can be implemented using recursion or an explicit stack.

3. **Path Finding:**
    - **BFS:** Guarantees the shortest path in an unweighted graph.
    - **DFS:** Does not guarantee the shortest path.

4. **Memory Usage:**
    - **BFS:** Can consume more memory if the graph is wide.
    - **DFS:** Can be more memory-efficient if the graph is wide, but can consume more memory if the graph is deep.

### Application in the Provided Code

In the provided Rust code, both BFS and DFS are used to explore regions in a 2D grid. The `flood_fill` function uses BFS
to explore all cells in a region, while the `bfs_count_sides` function uses BFS to count the unique sides of the region.

**BFS in `flood_fill`:**

- The `flood_fill` function uses a queue to explore all cells in a region level by level.
- It starts from a given cell and explores all its neighbors before moving on to the neighbors' neighbors.

**BFS in `bfs_count_sides`:**

- The `bfs_count_sides` function uses a queue to explore the perimeter cells and count the unique sides.
- It ensures that each side is counted only once by marking perimeter cells as seen.

By using BFS in both functions, the code ensures that all cells in a region and all perimeter cells are explored
efficiently.

## Example Usage

### Flood Fill Using BFS

**Breadth-First Search (BFS) Approach:**

- BFS explores the graph level by level.
- It uses a queue (FIFO) to keep track of the nodes to be explored next.
- BFS is useful when you want to explore all nodes at the present depth level before moving on to nodes at the next
  depth level.

**Example Implementation of Flood Fill Using BFS:**

```rust
use std::collections::VecDeque;

fn flood_fill_bfs(grid: &mut Vec<Vec<i32>>, start_row: usize, start_col: usize, new_value: i32) {
    let initial_value = grid[start_row][start_col];
    if initial_value == new_value {
        return;
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    while let Some((row, col)) = queue.pop_front() {
        if row >= grid.len() || col >= grid[0].len() || grid[row][col] != initial_value {
            continue;
        }

        grid[row][col] = new_value;

        for &(dr, dc) in &directions {
            let new_row = row.wrapping_add(dr as usize);
            let new_col = col.wrapping_add(dc as usize);
            if new_row < grid.len() && new_col < grid[0].len() {
                queue.push_back((new_row, new_col));
            }
        }
    }
}

fn main() {
    let mut grid = vec![
        vec![1, 1, 1, 2],
        vec![1, 1, 0, 2],
        vec![1, 0, 0, 2],
        vec![2, 2, 2, 2],
    ];

    let start_row = 0;
    let start_col = 0;
    let new_value = 3;

    flood_fill_bfs(&mut grid, start_row, start_col, new_value);

    for row in &grid {
        println!("{:?}", row);
    }
}
```

### Flood Fill Using DFS

**Depth-First Search (DFS) Approach:**

- DFS explores the graph by going as deep as possible along each branch before backtracking.
- It uses a stack (LIFO) to keep track of the nodes to be explored next. This can be implemented using recursion (
  implicit stack) or an explicit stack.
- DFS is useful when you want to explore as far down a branch as possible before backtracking.

**Example Implementation of Flood Fill Using DFS:**

```rust
fn flood_fill_dfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, new_value: i32) {
    let initial_value = grid[row][col];
    if initial_value == new_value {
        return;
    }
    fill(grid, row, col, initial_value, new_value);
}

fn fill(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, initial_value: i32, new_value: i32) {
    if row >= grid.len() || col >= grid[0].len() || grid[row][col] != initial_value {
        return;
    }

    grid[row][col] = new_value;

    if row > 0 {
        fill(grid, row - 1, col, initial_value, new_value); // Up
    }
    if row < grid.len() - 1 {
        fill(grid, row + 1, col, initial_value, new_value); // Down
    }
    if col > 0 {
        fill(grid, row, col - 1, initial_value, new_value); // Left
    }
    if col < grid[0].len() - 1 {
        fill(grid, row, col + 1, initial_value, new_value); // Right
    }
}

fn main() {
    let mut grid = vec![
        vec![1, 1, 1, 2],
        vec![1, 1, 0, 2],
        vec![1, 0, 0, 2],
        vec![2, 2, 2, 2],
    ];

    let start_row = 0;
    let start_col = 0;
    let new_value = 3;

    flood_fill_dfs(&mut grid, start_row, start_col, new_value);

    for row in &grid {
        println!("{:?}", row);
    }
}
```

### Summary

- **BFS Approach:**
    - Uses a queue to explore nodes level by level.
    - Suitable for finding the shortest path in unweighted graphs.
    - Can consume more memory if the graph is wide.

- **DFS Approach:**
    - Uses a stack (or recursion) to explore nodes by going as deep as possible along each branch before backtracking.
    - Suitable for exploring all possible paths in a graph.
    - Can be more memory-efficient if the graph is wide but can consume more memory if the graph is deep.

Both BFS and DFS can be used to implement the flood fill algorithm, and the choice between them depends on the specific
requirements and constraints of the problem you are solving.
}
if col < grid[0].len() - 1 {
fill(grid, row, col + 1, initial_value, new_value); // Right
}
}

fn main() {
let mut grid = vec![
vec![1, 1, 1, 2],
vec![1, 1, 0, 2],
vec![1, 0, 0, 2],
vec![2, 2, 2, 2],
];

    let start_row = 0;
    let start_col = 0;
    let new_value = 3;

    flood_fill_dfs(&mut grid, start_row, start_col, new_value);

    for row in &grid {
        println!("{:?}", row);
    }

}

```

### Summary

- **BFS Approach:**
  - Uses a queue to explore nodes level by level.
  - Suitable for finding the shortest path in unweighted graphs.
  - Can consume more memory if the graph is wide.

- **DFS Approach:**
  - Uses a stack (or recursion) to explore nodes by going as deep as possible along each branch before backtracking.
  - Suitable for exploring all possible paths in a graph.
  - Can be more memory-efficient if the graph is wide but can consume more memory if the graph is deep.

Both BFS and DFS can be used to implement the flood fill algorithm, and the choice between them depends on the specific requirements and constraints of the problem you are solving.
