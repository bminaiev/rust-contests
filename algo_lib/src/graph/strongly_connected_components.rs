use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;
use crate::graph::simple_graph::SimpleGraphT;

struct State {
    v: i32,
    g_pos: i32,
}

fn dfs1<G>(v: usize, used: &mut [bool], g: &G, order: &mut Vec<i32>)
where
    G: GraphTrait<SimpleEdge>,
{
    let n = g.num_vertices();
    let mut stack = Vec::with_capacity(n);
    stack.push(State {
        v: v as i32,
        g_pos: 0,
    });
    used[v] = true;
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        let edges = g.adj(cur_state.v as usize);
        match edges.get(cur_state.g_pos as usize) {
            None => {
                order.push(cur_state.v);
                stack.pop();
                continue;
            }
            Some(edge) => {
                let next = edge.to();
                if used[next] {
                    cur_state.g_pos += 1;
                    continue;
                }
                used[next] = true;
                stack.push(State {
                    v: next as i32,
                    g_pos: 0,
                });
            }
        }
    }
}

fn dfs2<'a, G>(v: usize, used: &mut [bool], g_rev: &'a G, cur_comp_id: usize, comp_id: &mut [usize])
where
    G: GraphTrait<SimpleEdge>,
{
    let mut stack = vec![];
    stack.push(State {
        v: v as i32,
        g_pos: 0,
    });
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        if g_rev.adj(cur_state.v as usize).len() == cur_state.g_pos as usize {
            comp_id[cur_state.v as usize] = cur_comp_id;
            stack.pop();
            continue;
        }
        let next = g_rev
            .adj(cur_state.v as usize)
            .get(cur_state.g_pos as usize)
            .unwrap()
            .to() as usize;
        if used[next] {
            cur_state.g_pos += 1;
            continue;
        }
        used[next] = true;
        stack.push(State {
            v: next as i32,
            g_pos: 0,
        });
    }
}

fn rev_graph<G>(graph: &G) -> impl GraphTrait<SimpleEdge>
where
    G: GraphTrait<SimpleEdge>,
{
    let mut expected_num_edges = vec![0u32; graph.num_vertices()];
    let iter = || {
        (0..graph.num_vertices()).flat_map(move |v| {
            graph
                .adj(v)
                .iter()
                .map(move |edge| (edge.to(), SimpleEdge::new(v)))
        })
    };
    iter().for_each(|(fr, _edge)| expected_num_edges[fr] += 1);
    let mut adj: Vec<_> = expected_num_edges
        .iter()
        .map(|&size| Vec::with_capacity(size as usize))
        .collect();
    iter().for_each(|(fr, edge)| adj[fr].push(edge));
    SimpleGraphT::with_adj(adj)
}

// TODO: usize -> u32
pub fn find_strongly_connected_component<G>(graph: &G) -> Vec<usize>
where
    G: GraphTrait<SimpleEdge>,
{
    let n = graph.num_vertices();
    let mut comp_id = vec![0; n];
    let mut used = vec![false; n];

    let mut order = Vec::with_capacity(n);
    {
        for v in 0..n {
            if !used[v] {
                dfs1(v, &mut used, graph, &mut order);
            }
        }
    }

    let mut used = vec![false; n];
    let mut cur_comp_id = 0;
    {
        let rev_graph = rev_graph(graph);

        for &v in order.iter().rev() {
            if used[v as usize] {
                continue;
            }
            dfs2(v as usize, &mut used, &rev_graph, cur_comp_id, &mut comp_id);
            cur_comp_id += 1;
        }
    }
    comp_id
}
