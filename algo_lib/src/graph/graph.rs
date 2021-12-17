use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::weighted_edge::WeightedEdge;
use std::ops::Index;
use crate::misc::num_traits::Number;

pub struct GraphT<E>
where
    E: EdgeTrait,
{
    adj: Vec<Vec<E>>,
}

pub struct EdgeIter<'a, E>
where
    E: EdgeTrait,
{
    vertex: usize,
    pos: usize,
    graph: &'a GraphT<E>,
}

impl<E> GraphT<E>
where
    E: EdgeTrait,
{
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
        }
    }

    pub fn with_edges(n: usize, edges: &[(usize, E)]) -> Self {
        let mut cnt_adj = vec![0; n];
        for (fr, _) in edges.iter() {
            cnt_adj[*fr] += 1;
        }
        let mut adj: Vec<_> = (0..n).map(|id| Vec::with_capacity(cnt_adj[id])).collect();
        for (fr, edge) in edges.iter() {
            adj[*fr].push(edge.clone());
        }
        Self { adj }
    }

    pub fn add_edge(&mut self, from: usize, edge: E) {
        self.adj[from].push(edge);
    }

    pub fn all_edges(&self) -> EdgeIter<E> {
        EdgeIter {
            vertex: 0,
            pos: 0,
            graph: self,
        }
    }

    pub fn vertices_num(&self) -> usize {
        self.adj.len()
    }
}

impl<E> Index<usize> for GraphT<E>
where
    E: EdgeTrait,
{
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.adj[index]
    }
}

impl<'a, E> Iterator for EdgeIter<'a, E>
where
    E: EdgeTrait,
{
    type Item = (usize, E);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.vertex == self.graph.adj.len() {
                return None;
            }
            if self.graph[self.vertex].len() == self.pos {
                self.pos = 0;
                self.vertex += 1;
                continue;
            }
            let edge = self.graph.adj[self.vertex][self.pos];
            self.pos += 1;
            return Some((self.vertex, edge));
        }
    }
}
