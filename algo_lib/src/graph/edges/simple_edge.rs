use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::simple_graph::SimpleGraphT;

#[derive(Copy, Clone, Default)]
pub struct SimpleEdge {
    to: u32,
}

impl SimpleEdge {
    pub fn new(to: usize) -> Self {
        Self { to: to as u32 }
    }
}

impl EdgeTrait for SimpleEdge {
    #[inline(always)]
    fn to(&self) -> usize {
        self.to as usize
    }

    fn rev(&self, from: usize) -> Self {
        Self { to: from as u32 }
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
