use std::ops::Range;

use crate::misc::rec_function::{Callable, Callable3, RecursiveFunction, RecursiveFunction3};

pub struct Hld {
    parent: Vec<usize>,
    pub order: Vec<usize>,
    block_start: Vec<usize>,
    pub pos_in_order: Vec<usize>,
}

impl Hld {
    pub fn new(mut g: Vec<Vec<usize>>, tree_root: usize) -> Self {
        let n = g.len();
        let mut size = vec![0; n];
        let mut parent = vec![0; n];
        RecursiveFunction3::new(|f, v: usize, p: usize, h: usize| {
            parent[v] = p;
            size[v] = 1;
            for &to in g[v].iter() {
                if to != p {
                    f.call(to, v, h + 1);
                    size[v] += size[to];
                }
            }
            g[v].sort_by(|&u, &v| size[u].cmp(&size[v]).reverse());
        })
        .call(tree_root, tree_root, 0);

        let mut order = vec![];
        RecursiveFunction::new(|f, v: usize| {
            order.push(v);
            for &to in g[v].iter() {
                if to != parent[v] {
                    f.call(to);
                }
            }
        })
        .call(tree_root);
        let mut pos_in_order = vec![0; n];
        for i in 0..n {
            pos_in_order[order[i]] = i;
        }
        let mut block_start = vec![0; n];
        for i in 1..n {
            if order[i - 1] == parent[order[i]] {
                block_start[i] = block_start[i - 1];
            } else {
                block_start[i] = i;
            }
        }

        Self {
            parent,
            order,
            block_start,
            pos_in_order,
        }
    }

    pub fn find_path_segs(&self, mut u: usize, mut v: usize) -> Vec<Range<usize>> {
        let mut segs = vec![];
        while u != v {
            if self.pos_in_order[u] < self.pos_in_order[v] {
                std::mem::swap(&mut u, &mut v);
            }
            let from = std::cmp::max(
                self.block_start[self.pos_in_order[u]],
                self.pos_in_order[v] + 1,
            );
            segs.push(from..self.pos_in_order[u] + 1);
            u = self.parent[self.order[from]];
        }
        segs.push(self.pos_in_order[v]..self.pos_in_order[v] + 1);
        segs
    }

    pub fn lca(&self, mut v: usize, mut u: usize) -> usize {
        while v != u {
            if self.pos_in_order[v] < self.pos_in_order[u] {
                std::mem::swap(&mut v, &mut u);
            }
            if self.block_start[self.pos_in_order[v]] <= self.pos_in_order[u] {
                return u;
            }
            v = self.parent[self.order[self.block_start[self.pos_in_order[v]]]];
        }
        v
    }
}
