use std::cmp::min;
use std::collections::VecDeque;

#[allow(dead_code)]
struct FlowEdge {
    fr: usize,
    to: usize,
    flow: i64,
    cap: i64,
}

#[allow(dead_code)]
pub struct FlowDinic {
    edges: Vec<FlowEdge>,
    graph: Vec<Vec<usize>>,
    pub n: usize,
}

impl FlowDinic {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let graph = vec![Vec::new(); n];
        let edges = vec![];
        Self { edges, graph, n }
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, fr: usize, to: usize, cap: i64) -> usize {
        let id = self.edges.len();
        let e1 = FlowEdge {
            fr,
            to,
            flow: 0,
            cap,
        };
        let e2 = FlowEdge {
            fr: to,
            to: fr,
            flow: 0,
            cap: 0,
        };
        self.graph[fr].push(id);
        self.graph[to].push(id ^ 1);
        self.edges.push(e1);
        self.edges.push(e2);
        id
    }

    #[allow(dead_code)]
    fn bfs(&self, source: usize, height: &mut [usize], queue: &mut VecDeque<usize>) {
        queue.clear();
        for x in height.iter_mut() {
            *x = usize::MAX;
        }
        height[source] = 0;
        queue.push_back(source);
        while let Some(v) = queue.pop_front() {
            for &e_id in &self.graph[v] {
                let edge = &self.edges[e_id];
                if edge.flow == edge.cap {
                    continue;
                }
                if height[edge.to] != usize::MAX {
                    continue;
                }
                height[edge.to] = height[edge.fr] + 1;
                queue.push_back(edge.to);
            }
        }
    }

    #[allow(dead_code)]
    fn dfs(
        &mut self,
        height: &[usize],
        v: usize,
        target: usize,
        cur_flow: i64,
        iter: &mut [usize],
    ) -> i64 {
        if target == v || cur_flow == 0 {
            return cur_flow;
        }
        while iter[v] < self.graph[v].len() {
            let e_id = self.graph[v][iter[v]];
            iter[v] += 1;
            let e = &self.edges[e_id];
            if height[e.to] != height[e.fr] + 1 || e.flow == e.cap {
                continue;
            }
            let to = e.to;
            let next_flow = min(cur_flow, e.cap - e.flow);
            let add = self.dfs(height, to, target, next_flow, iter);
            if add == 0 {
                continue;
            }
            self.edges[e_id].flow += add;
            self.edges[e_id ^ 1].flow -= add;
            return add;
        }
        0
    }

    #[allow(dead_code)]
    pub fn find_flow(&mut self) -> i64 {
        let source = 0;
        let target = self.n - 1;
        let mut res = 0;
        let mut height = vec![0; self.n];
        let mut queue = VecDeque::new();
        let mut iter = vec![0; self.n];
        loop {
            self.bfs(source, &mut height, &mut queue);
            if height[target] == usize::MAX {
                break;
            }
            for x in iter.iter_mut() {
                *x = 0;
            }
            loop {
                let pushed = self.dfs(&height, source, target, i64::MAX, &mut iter);
                if pushed == 0 {
                    break;
                }
                res += pushed;
            }
        }
        res
    }

    pub fn get_edge_flow(&self, edge_id: usize) -> i64 {
        self.edges[edge_id].flow
    }

    pub fn inc_edge_cap(&mut self, edge_id: usize, delta: i64) {
        self.edges[edge_id].cap += delta;
    }
}
