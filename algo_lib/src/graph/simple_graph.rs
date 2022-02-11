use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::num_traits::Number;

use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::edges::weighted_edge::WeightedEdge;
use crate::graph::weighted_graph::WeightedGraph;

pub struct SimpleGraphT<E>
where
    E: EdgeTrait,
{
    adj: Vec<Vec<E>>,
    num_edges: usize,
}

impl<E> SimpleGraphT<E>
where
    E: EdgeTrait,
{
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
            num_edges: 0,
        }
    }

    pub fn with_adj(adj: Vec<Vec<E>>) -> Self {
        let num_edges = adj.iter().map(|v| v.len()).sum();
        Self { adj, num_edges }
    }

    pub fn with_edges(n: usize, edges: &[(usize, E)]) -> Self {
        let mut cnt_adj = vec![0u32; n];
        for (fr, _) in edges.iter() {
            cnt_adj[*fr] += 1;
        }
        let mut adj: Vec<_> = (0..n)
            .map(|id| Vec::with_capacity(cnt_adj[id] as usize))
            .collect();
        for (fr, edge) in edges.iter() {
            adj[*fr].push(edge.clone());
        }
        Self {
            adj,
            num_edges: edges.len(),
        }
    }

    pub fn add_complex_edge(&mut self, from: usize, edge: E) {
        self.adj[from].push(edge);
    }

    pub fn add_complex_bi_edge(&mut self, from: usize, edge: E) {
        self.adj[from].push(edge);
        let rev_edge = edge.rev(from);
        self.adj[edge.to()].push(rev_edge);
    }

    fn ensure_vertex_exist(&mut self, v: usize) {
        if v >= self.adj.len() {
            self.adj.resize(v + 1, vec![]);
        }
    }

    pub fn add_edge_maybe_new_vertices(&mut self, from: usize, edge: E) {
        self.ensure_vertex_exist(from);
        self.ensure_vertex_exist(edge.to());
        self.adj[from].push(edge);
    }
}

impl<E> GraphTrait<E> for SimpleGraphT<E>
where
    E: EdgeTrait,
{
    fn len(&self) -> usize {
        self.num_vertices()
    }

    fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn adj(&self, v: usize) -> &[E] {
        &self.adj[v]
    }
}

impl<E> SimpleGraphT<E>
where
    E: EdgeTrait,
{
    pub fn all_edges(&self) -> impl Iterator<Item = (usize, &E)> + '_ {
        (0..self.num_vertices()).flat_map(move |v| self.adj[v].iter().map(move |e| (v, e)))
    }
}

impl SimpleGraphT<SimpleEdge> {
    pub fn add_edge(&mut self, fr: usize, to: usize) {
        self.add_complex_edge(fr, SimpleEdge::new(to));
    }

    pub fn add_bi_edge(&mut self, fr: usize, to: usize) {
        self.add_complex_edge(fr, SimpleEdge::new(to));
        self.add_complex_edge(to, SimpleEdge::new(fr));
    }
}

impl<T: Number> WeightedGraph<T> {
    pub fn add_weighted_edge(&mut self, fr: usize, to: usize, cost: T) {
        self.add_complex_edge(fr, WeightedEdge::new(to, cost));
    }

    pub fn add_bi_weighted_edge(&mut self, fr: usize, to: usize, cost: T) {
        self.add_complex_edge(fr, WeightedEdge::new(to, cost));
        self.add_complex_edge(to, WeightedEdge::new(fr, cost));
    }
}
