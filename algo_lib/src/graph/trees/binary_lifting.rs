use crate::collections::array_2d::Array2D;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::bits::index_of_highest_set_bit;
use crate::misc::rec_function::{Callable2, RecursiveFunction2};

pub struct BinaryLifting {
    up: Array2D<u32>,
    height: Vec<u32>,
}

impl BinaryLifting {
    pub fn new<Graph, Edge>(tree: &Graph, root: usize) -> Self
    where
        Graph: GraphTrait<Edge>,
        Edge: EdgeTrait,
    {
        let n = tree.num_vertices();
        let levels = index_of_highest_set_bit(n) + 1;
        let mut up = Array2D::new(root as u32, levels, n);
        let mut height = vec![0; n];
        RecursiveFunction2::new(|f, v, parent| {
            for edge in tree.adj(v) {
                if edge.to() == parent {
                    continue;
                }
                height[edge.to()] = height[v] + 1;
                f.call(edge.to(), v);
                up[0][edge.to()] = v as u32;
            }
        })
        .call(root, root);
        for lvl in 1..levels {
            for v in 0..n {
                up[lvl][v] = up[lvl - 1][up[lvl - 1][v] as usize];
            }
        }
        Self { up, height }
    }

    pub fn height(&self, v: usize) -> usize {
        self.height[v] as usize
    }

    pub fn up(&self, mut v: usize, len: usize) -> usize {
        for lvl in 0..self.up.rows() {
            if ((1 << lvl) & len) != 0 {
                v = self.up[lvl][v] as usize;
            }
        }
        v
    }

    pub fn lca(&self, mut v1: usize, mut v2: usize) -> usize {
        let h1 = self.height[v1];
        let h2 = self.height[v2];
        if h1 < h2 {
            v2 = self.up(v2, (h2 - h1) as usize);
        } else {
            v1 = self.up(v1, (h1 - h2) as usize);
        }
        let levels = self.up.rows();
        for lvl in (0..levels).rev() {
            if self.up[lvl][v1] != self.up[lvl][v2] {
                v1 = self.up[lvl][v1] as usize;
                v2 = self.up[lvl][v2] as usize;
            }
        }
        if v1 == v2 {
            v1
        } else {
            self.up[0][v1] as usize
        }
    }
}
