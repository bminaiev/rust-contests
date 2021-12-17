use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub struct SimpleGraphT<E>
where
    E: EdgeTrait,
{
    adj: Vec<Vec<E>>,
}

pub struct AllNodesEdgeIter<'a, E>
where
    E: EdgeTrait,
{
    vertex: usize,
    pos: usize,
    graph: &'a SimpleGraphT<E>,
}

impl<E> SimpleGraphT<E>
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
}

impl<'a, E: 'a> Iterator for AllNodesEdgeIter<'a, E>
where
    E: EdgeTrait,
{
    type Item = (usize, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.vertex == self.graph.adj.len() {
                return None;
            }
            if self.graph.adj[self.vertex].len() == self.pos {
                self.pos = 0;
                self.vertex += 1;
                continue;
            }
            let edge = &self.graph.adj[self.vertex][self.pos];
            self.pos += 1;
            return Some((self.vertex, edge));
        }
    }
}

impl<'a, E: 'a> GraphTrait<'a, E> for SimpleGraphT<E>
where
    E: EdgeTrait,
{
    type OneNodeEdgeIter = core::slice::Iter<'a, E>;
    type AllNodesEdgeIter = AllNodesEdgeIter<'a, E>;

    fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    fn all_edges(&'a self) -> Self::AllNodesEdgeIter {
        AllNodesEdgeIter {
            vertex: 0,
            pos: 0,
            graph: self,
        }
    }

    fn adj(&'a self, v: usize) -> Self::OneNodeEdgeIter {
        self.adj[v].iter()
    }
}
