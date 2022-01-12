use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub struct BfsState {
    pub queue: Vec<usize>,
    pub dist: Vec<u32>,
    pub prev: Vec<usize>,
}

pub fn bfs<Edge>(root: usize, graph: &impl GraphTrait<Edge>) -> BfsState
where
    Edge: EdgeTrait,
{
    let mut state = BfsState {
        queue: vec![root],
        dist: vec![u32::MAX; graph.num_vertices()],
        prev: vec![usize::MAX; graph.num_vertices()],
    };
    state.dist[root] = 0;
    state.prev[root] = root;
    let mut it = 0;
    while it != state.queue.len() {
        let v = state.queue[it];
        it += 1;
        for edge in graph.adj(v) {
            if state.dist[edge.to()] == u32::MAX {
                state.dist[edge.to()] = state.dist[v] + 1;
                state.queue.push(edge.to());
                state.prev[edge.to()] = v;
            }
        }
    }
    state
}
