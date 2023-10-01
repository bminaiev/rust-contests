use std::ops::Range;

use crate::misc::{
    min_max::UpdateMinMax,
    rec_function::{Callable3, RecursiveFunction3},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct VertexInfo {
    pub pos: usize,
    // range is [pos..max_subtree_pos)
    pub max_subtree_pos: usize,
    pub parent: usize,
    pub height: usize,
}

impl VertexInfo {
    pub fn range(&self) -> Range<usize> {
        self.pos..self.max_subtree_pos
    }
}

#[derive(Debug)]
pub struct DfsOrder {
    pub order: Vec<usize>,
    pub info: Vec<VertexInfo>,
}

impl DfsOrder {
    pub fn new(g: &[Vec<usize>], root: usize) -> Self {
        let n = g.len();
        let mut order = vec![];
        let mut info = vec![VertexInfo::default(); n];
        RecursiveFunction3::new(|f, v: usize, p: usize, h: usize| {
            order.push(v);
            info[v].pos = order.len() - 1;
            info[v].max_subtree_pos = order.len();
            info[v].parent = p;
            info[v].height = h;
            for &to in g[v].iter() {
                if to != p {
                    f.call(to, v, h + 1);
                    let max_subtree_pos = info[to].max_subtree_pos;
                    info[v].max_subtree_pos.update_max(max_subtree_pos);
                }
            }
        })
        .call(root, root, 0);
        assert_eq!(order.len(), n);
        Self { order, info }
    }

    pub fn is_in_subtree_of(&self, v: usize, anc: usize) -> bool {
        self.info[anc].pos <= self.info[v].pos && self.info[v].pos < self.info[anc].max_subtree_pos
    }
}
