use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::dfs_builder::DfsBuilder;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;

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

    let mut order: Vec<u32> = Vec::with_capacity(n);
    {
        let mut dfs1 = DfsBuilder::new(graph, &mut order)
            .on_exit(|_parent, edge, order| order.push(edge.to() as u32));
        for v in 0..n {
            if !dfs1.seen(v) {
                dfs1.run(SimpleEdge::new(v));
            }
        }
    }

    let mut dfs2_state = (vec![0; n], 0);
    {
        let rev_graph = rev_graph(graph);

        let mut dfs2 = DfsBuilder::new(&rev_graph, &mut dfs2_state)
            .on_exit(|_parent, edge, (comp_id, cur_comp_id)| comp_id[edge.to()] = *cur_comp_id);

        for &v in order.iter().rev() {
            let v = v as usize;
            if !dfs2.seen(v) {
                dfs2.run(SimpleEdge::new(v));
            }
            dfs2.state.1 += 1;
        }
    }
    dfs2_state.0
}
