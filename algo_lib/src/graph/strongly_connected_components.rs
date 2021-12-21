use crate::collections::bit_set::BitSet;
use crate::dbg;
use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;
use std::time::Instant;

enum Frame {
    Check(u32),
    RemoveFrom(u32),
}

fn dfs1<G>(v: usize, stack: &mut Vec<Frame>, used: &mut BitSet, g: &G, order: &mut Vec<u32>)
where
    G: GraphTrait<SimpleEdge>,
{
    stack.push(Frame::Check(v as u32));
    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Check(v) => {
                let v = v as usize;
                if !used.get(v) {
                    stack.push(Frame::RemoveFrom(v as u32));
                    used.set(v, true);
                    for edge in g.adj(v) {
                        stack.push(Frame::Check(edge.to() as u32));
                    }
                }
            }
            Frame::RemoveFrom(v) => {
                order.push(v);
            }
        }
    }
}

fn dfs2<G>(
    v: usize,
    stack: &mut Vec<Frame>,
    used: &mut BitSet,
    g_rev: &G,
    cur_comp_id: u32,
    comp_id: &mut [u32],
) where
    G: GraphTrait<SimpleEdge>,
{
    stack.push(Frame::Check(v as u32));
    while let Some(frame) = stack.pop() {
        match frame {
            Frame::Check(v) => {
                let v = v as usize;
                if !used.get(v) {
                    used.set(v, true);
                    comp_id[v] = cur_comp_id;
                    for edge in g_rev.adj(v) {
                        stack.push(Frame::Check(edge.to() as u32));
                    }
                }
            }
            Frame::RemoveFrom(_v) => {
                unreachable!();
            }
        }
    }
}

fn rev_graph<G>(graph: &G) -> impl GraphTrait<SimpleEdge>
where
    G: GraphTrait<SimpleEdge>,
{
    let iter = || {
        (0..graph.num_vertices()).flat_map(move |v| {
            graph
                .adj(v)
                .iter()
                .map(move |edge| (edge.to(), SimpleEdge::new(v)))
        })
    };
    CompressedGraph::with_edge_iter(graph.num_vertices(), iter())
}

pub fn find_strongly_connected_component<G>(graph: &G) -> Vec<u32>
where
    G: GraphTrait<SimpleEdge>,
{
    let n = graph.num_vertices();
    let mut comp_id = vec![0; n];
    let mut used = BitSet::new(n);
    let n = graph.num_vertices();
    let mut stack = vec![];

    let mut order: Vec<u32> = Vec::with_capacity(n);
    {
        for v in 0..n {
            if !used.get(v) {
                dfs1(v, &mut stack, &mut used, graph, &mut order);
            }
        }
    }

    used.clear();
    let mut cur_comp_id = 0;
    {
        let rev_graph = rev_graph(graph);

        for &v in order.iter().rev() {
            if used.get(v as usize) {
                continue;
            }
            dfs2(
                v as usize,
                &mut stack,
                &mut used,
                &rev_graph,
                cur_comp_id,
                &mut comp_id,
            );
            cur_comp_id += 1;
        }
    }
    comp_id
}
