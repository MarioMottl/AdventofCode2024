### Breadth-First Search (BFS)

**BFS Overview:**
- BFS explores the graph level by level.
- It starts at a given node (often called the root) and explores all its neighbors before moving on to the neighbors' neighbors.
- BFS uses a queue (FIFO) to keep track of the nodes to be explored next.

**BFS Characteristics:**
- **Traversal Order:** BFS visits nodes in the order of their distance from the starting node, exploring all nodes at the present depth level before moving on to nodes at the next depth level.
- **Data Structure:** BFS uses a queue to manage the nodes to be explored.
- **Shortest Path:** BFS can be used to find the shortest path in an unweighted graph because it explores all nodes at the current depth level before moving on to the next level.
- **Memory Usage:** BFS can consume a lot of memory if the graph is wide (i.e., nodes have many neighbors) because it needs to store all nodes at the current level in the queue.

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
- It starts at a given node and explores as far as possible along each branch before backtracking to explore other branches.
- DFS uses a stack (LIFO) to keep track of the nodes to be explored next. This can be implemented using recursion (implicit stack) or an explicit stack.

**DFS Characteristics:**
- **Traversal Order:** DFS visits nodes by exploring as far down a branch as possible before backtracking. This means it goes deep into the graph before exploring siblings.
- **Data Structure:** DFS uses a stack to manage the nodes to be explored. This can be an explicit stack or the call stack in the case of recursion.
- **Path Finding:** DFS can be used to find paths in a graph, but it does not guarantee the shortest path in an unweighted graph.
- **Memory Usage:** DFS can be more memory-efficient than BFS if the graph is wide, as it only needs to store the current path in the stack.

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

In the provided Rust code, both BFS and DFS are used to explore regions in a 2D grid. The `flood_fill` function uses BFS to explore all cells in a region, while the `bfs_count_sides` function uses BFS to count the unique sides of the region.

**BFS in `flood_fill`:**
- The `flood_fill` function uses a queue to explore all cells in a region level by level.
- It starts from a given cell and explores all its neighbors before moving on to the neighbors' neighbors.

**BFS in `bfs_count_sides`:**
- The `bfs_count_sides` function uses a queue to explore the perimeter cells and count the unique sides.
- It ensures that each side is counted only once by marking perimeter cells as seen.

By using BFS in both functions, the code ensures that all cells in a region and all perimeter cells are explored efficiently.
