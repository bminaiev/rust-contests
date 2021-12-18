use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub struct SimpleGraphT<E>
where
    E: EdgeTrait,
{
    adj: Vec<Vec<E>>,
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

impl<'a, E: 'a> GraphTrait<'a, E> for SimpleGraphT<E>
where
    E: EdgeTrait,
{
    type OneNodeEdgeIter = core::slice::Iter<'a, E>;

    fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    fn adj(&'a self, v: usize) -> Self::OneNodeEdgeIter {
        self.adj[v].iter()
    }
}

impl<E> SimpleGraphT<E>
where
    E: EdgeTrait,
{
    pub fn all_edges(&self) -> impl Iterator<Item = (usize, &E)> + '_ {
        (0..self.num_vertices()).flat_map(move |v| self.adj(v).map(move |e| (v, e)))
    }
}
