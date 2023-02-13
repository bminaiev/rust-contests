use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::dfs_builder::dfs_builder;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;
use crate::misc::num_traits::Number;

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

#[derive(Debug)]
pub struct StronglyConnectedComponents<CompIdType> {
    pub num_comps: usize,
    pub comp_id: Vec<CompIdType>,
}

impl<CompIdType> StronglyConnectedComponents<CompIdType> {
    pub fn generate_components(&self) -> Vec<Vec<usize>>
    where
        CompIdType: Number,
    {
        let mut res = vec![vec![]; self.num_comps];
        for (v, comp_id) in self.comp_id.iter().enumerate() {
            res[comp_id.to_i32() as usize].push(v);
        }
        res
    }
}

///
/// Return comp_id[v]
///
/// If edge [u] -> [v] exist
/// <=>
/// comp_id[u] <= comp_id[v]
///
pub fn find_strongly_connected_component<G, CompIdType: Number>(
    graph: &G,
) -> StronglyConnectedComponents<CompIdType>
where
    G: GraphTrait<SimpleEdge>,
{
    let n = graph.num_vertices();

    let mut order: Vec<u32> = Vec::with_capacity(n);
    {
        let mut dfs1 = dfs_builder(graph, &mut order)
            .on_exit(|_parent, edge, order| order.push(edge.to() as u32));
        for v in 0..n {
            if !dfs1.seen(v) {
                dfs1.run(SimpleEdge::new(v));
            }
        }
    }

    let mut dfs2_state = (vec![CompIdType::ZERO; n], CompIdType::ZERO);
    {
        let rev_graph = rev_graph(graph);

        let mut dfs2 = dfs_builder(&rev_graph, &mut dfs2_state)
            .on_exit(|_parent, edge, (comp_id, cur_comp_id)| comp_id[edge.to()] = *cur_comp_id);

        for &v in order.iter().rev() {
            let v = v as usize;
            if !dfs2.seen(v) {
                dfs2.run(SimpleEdge::new(v));
                dfs2.state.1 += CompIdType::ONE;
            }
        }
    }
    StronglyConnectedComponents {
        num_comps: dfs2_state.1.to_i32() as usize,
        comp_id: dfs2_state.0,
    }
}
