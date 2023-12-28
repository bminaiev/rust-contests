use std::collections::VecDeque;

use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::graph_trait::GraphTrait;

#[derive(Debug)]
pub struct BfsState {
    queue: VecDeque<usize>,
    pub dist: Vec<u32>,
    pub prev: Vec<usize>,
}

pub fn bfs<Edge>(root: usize, graph: &impl GraphTrait<Edge>) -> BfsState
where
    Edge: EdgeTrait,
{
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut state = BfsState {
        queue,
        dist: vec![std::u32::MAX; graph.num_vertices()],
        prev: vec![std::usize::MAX; graph.num_vertices()],
    };
    state.dist[root] = 0;
    state.prev[root] = root;
    let mut it = 0;
    while it != state.queue.len() {
        let v = state.queue[it];
        it += 1;
        for edge in graph.adj(v) {
            if state.dist[edge.to()] == std::u32::MAX {
                state.dist[edge.to()] = state.dist[v] + 1;
                state.queue.push_back(edge.to());
                state.prev[edge.to()] = v;
            }
        }
    }
    state
}

pub fn bfs01(root: usize, graph: &impl GraphTrait<WeightedEdge<u32>>) -> BfsState {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut state = BfsState {
        queue,
        dist: vec![std::u32::MAX; graph.num_vertices()],
        prev: vec![std::usize::MAX; graph.num_vertices()],
    };
    state.dist[root] = 0;
    state.prev[root] = root;
    while let Some(v) = state.queue.pop_front() {
        for edge in graph.adj(v) {
            let ndist = state.dist[v] + edge.cost;
            if state.dist[edge.to()] > ndist {
                state.dist[edge.to()] = ndist;
                state.prev[edge.to()] = v;
                if edge.cost == 0 {
                    state.queue.push_front(edge.to());
                } else if edge.cost == 1 {
                    state.queue.push_back(edge.to());
                } else {
                    unreachable!("Only 0-1 weights are supported");
                }
            }
        }
    }
    state
}

impl BfsState {
    ///
    /// ``path[0]`` = root
    ///
    /// ``path[len - 1]`` = to
    ///
    pub fn get_path(&self, mut to: usize) -> Option<Vec<usize>> {
        if self.dist[to] == std::u32::MAX {
            return None;
        }
        let mut res = vec![to];
        while self.prev[to] != to {
            to = self.prev[to];
            res.push(to);
        }
        res.reverse();
        Some(res)
    }
}
