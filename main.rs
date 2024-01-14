use std::collections::VecDeque;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(num_vertices: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); num_vertices],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list[u].push(v);
    }

    fn bfs(&self, start_vertex: usize) -> Vec<usize> {
        let num_vertices = self.adjacency_list.len();
        let mut visited = vec![false; num_vertices];
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited[start_vertex] = true;
        queue.push_back(start_vertex);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            for &neighbour in &self.adjacency_list[vertex] {
                if !visited[neighbour] {
                    visited[neighbour] = true;
                    queue.push_back(neighbour);
                }
            }
        }

        result
    }

    fn dfs(&self, start_vertex: usize) -> Vec<usize> {
        let size = self.adjacency_list.len();
        let mut visited = vec![false; size];
        let mut stack = Vec::new();
        let mut result = Vec::new();

        stack.push(start_vertex);

        while let Some(vertex) = stack.pop() {
            if visited[vertex] {
                continue;
            }

            visited[vertex] = true;
            result.push(vertex);

            for &neighbour in &self.adjacency_list[vertex] {
                stack.push(neighbour);
            }
        }

        result
    }
}

fn main() {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1);
    graph.add_edge(0, 3);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);

    let bfs_result = graph.bfs(0);
    println!("BFS traversal: {:?}", bfs_result);

    let dfs_result = graph.dfs(0);
    println!("DFS traversal: {:?}", dfs_result);
}