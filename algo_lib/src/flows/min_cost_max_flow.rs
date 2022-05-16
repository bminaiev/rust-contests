use std::cmp::min;
use std::collections::BTreeSet;

struct MinCostEdge {
    flow: i64,
    cap: i64,
    cost: i64,
    fr: usize,
    to: usize,
}

pub struct MinCostMaxFlow {
    graph: Vec<Vec<usize>>,
    edges: Vec<MinCostEdge>,
    pub n: usize,
}

#[derive(Default, Debug)]
pub struct CostAndFlow {
    pub cost: i64,
    pub flow: i64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Vertex {
    dist: i64,
    v: usize,
    edge_id: usize,
}

impl MinCostMaxFlow {
    const INF: i64 = std::i64::MAX;

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let graph = vec![Vec::new(); n];
        let edges = vec![];
        Self { graph, edges, n }
    }

    #[allow(dead_code)]
    pub fn get_edge_flow(&self, edge_id: usize) -> i64 {
        self.edges[edge_id].flow
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, fr: usize, to: usize, cap: i64, cost: i64) -> usize {
        let id = self.edges.len();
        let e1 = MinCostEdge {
            fr,
            to,
            flow: 0,
            cap,
            cost,
        };
        let e2 = MinCostEdge {
            to: fr,
            fr: to,
            flow: 0,
            cap: 0,
            cost: -cost,
        };
        self.edges.push(e1);
        self.edges.push(e2);
        self.graph[fr].push(id);
        self.graph[to].push(id ^ 1);
        id
    }

    fn dijkstra(&mut self, source: usize, was: &mut [bool], h: &[i64], vertices: &mut [Vertex]) {
        for x in vertices.iter_mut() {
            x.dist = Self::INF;
        }
        for x in was.iter_mut() {
            *x = false;
        }
        vertices[source] = Vertex {
            dist: 0,
            v: source,
            edge_id: 0,
        };

        let mut heap = BTreeSet::new();
        heap.insert(vertices[source]);

        while !heap.is_empty() {
            let vertex = *heap.iter().next().unwrap();
            heap.remove(&vertex);
            if was[vertex.v] {
                continue;
            }
            was[vertex.v] = true;
            for &e_id in &self.graph[vertex.v] {
                let e = &self.edges[e_id];
                if e.flow >= e.cap {
                    continue;
                }
                let edge_cost = e.cost + h[e.fr] - h[e.to];
                assert!(edge_cost >= 0);
                let new_dist = vertices[e.fr].dist + e.cost + h[e.fr] - h[e.to];
                if vertices[e.to].dist > new_dist {
                    assert!(!was[e.to]);
                    vertices[e.to] = Vertex {
                        v: e.to,
                        edge_id: e_id,
                        dist: new_dist,
                    };
                    heap.insert(vertices[e.to]);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn find_min_cost_max_flow(&mut self, source: usize, target: usize) -> CostAndFlow {
        let mut h = vec![0; self.n];
        loop {
            let mut changed = false;
            for e in &self.edges {
                if e.cap > 0 && h[e.to] > h[e.fr] + e.cost {
                    h[e.to] = h[e.fr] + e.cost;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut was = vec![false; self.n];
        let mut ans = CostAndFlow::default();
        let mut vertices: Vec<Vertex> = (0..self.n)
            .map(|v| Vertex {
                dist: 0,
                v,
                edge_id: 0,
            })
            .collect();
        loop {
            self.dijkstra(source, &mut was, &h, &mut vertices);
            if vertices[target].dist == Self::INF {
                break;
            }
            let mut cur_v = target;
            let mut add_flow = Self::INF;
            while cur_v != source {
                let e_id = vertices[cur_v].edge_id;
                add_flow = min(add_flow, self.edges[e_id].cap - self.edges[e_id].flow);
                cur_v = self.edges[e_id].fr;
            }
            assert!(add_flow > 0);
            let path_cost = vertices[target].dist + h[target] - h[source];
            ans.cost += path_cost * add_flow;
            ans.flow += add_flow;

            cur_v = target;
            while cur_v != source {
                let e_id = vertices[cur_v].edge_id;
                self.edges[e_id].flow += add_flow;
                self.edges[e_id ^ 1].flow -= add_flow;
                cur_v = self.edges[e_id].fr;
            }
            for i in 0..self.n {
                if vertices[i].dist != Self::INF {
                    h[i] += vertices[i].dist;
                }
            }
        }
        ans
    }
}
