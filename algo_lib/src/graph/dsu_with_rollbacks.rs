use crate::misc::{func::id, gen_vector::gen_vec};

pub trait DsuNodeTrait: Copy + Clone + Default {
    fn join(lhs: &Self, rhs: &Self) -> Self;
}

#[derive(Clone, Copy)]
struct DsuEvent<T: DsuNodeTrait> {
    idx: usize,
    size: usize,
    p: usize,
    node: T,
}

pub struct DsuWithRollbacks<T: DsuNodeTrait> {
    size: Vec<usize>,
    p: Vec<usize>,
    events: Vec<DsuEvent<T>>,
    nodes: Vec<T>,
}

impl<T: DsuNodeTrait> DsuWithRollbacks<T> {
    pub fn new(n: usize) -> Self {
        let p = gen_vec(n, id);
        let nodes = vec![T::default(); n];
        Self {
            size: vec![1; n],
            p,
            events: vec![],
            nodes,
        }
    }

    pub fn get_current_time(&self) -> usize {
        self.events.len()
    }

    pub fn rollback(&mut self, time: usize) {
        while self.events.len() != time {
            let ev = self.events.pop().unwrap();
            self.size[ev.idx] = ev.size;
            self.p[ev.idx] = ev.p;
            self.nodes[ev.idx] = ev.node;
        }
    }

    pub fn get(&self, mut v: usize) -> usize {
        while self.p[v] != v {
            v = self.p[v];
        }
        return v;
    }

    pub fn get_node(&self, mut v: usize) -> &T {
        v = self.get(v);
        return &self.nodes[v];
    }

    pub fn save(&mut self, i: usize) {
        self.events.push(DsuEvent {
            idx: i,
            size: self.size[i],
            p: self.p[i],
            node: self.nodes[i],
        })
    }

    pub fn unite(&mut self, mut v: usize, mut u: usize) {
        v = self.get(v);
        u = self.get(u);
        if v == u {
            return;
        }
        let (smaller, larger) = if self.size[u] < self.size[v] {
            (u, v)
        } else {
            (v, u)
        };
        self.save(smaller);
        self.save(larger);
        self.p[smaller] = larger;
        self.size[larger] += self.size[smaller];
        self.nodes[larger] = T::join(&self.nodes[smaller], &self.nodes[larger]);
    }

    pub fn set_node(&mut self, v: usize, node: T) {
        self.save(v);
        self.nodes[v] = node;
    }

    pub fn len(&self) -> usize {
        self.p.len()
    }
}
