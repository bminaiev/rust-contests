use crate::graph::all_edges::all_graph_edges;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;
use crate::graph::simple_graph::SimpleGraphT;

struct State {
    v: i32,
    g_pos: i32,
}

fn dfs1<'a, G>(v: usize, used: &mut [bool], g: &'a G, order: &mut Vec<i32>)
where
    G: GraphTrait<'a, SimpleEdge>,
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
        match edges.skip(cur_state.g_pos as usize).next() {
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

fn dfs2<'a, G: 'a>(
    v: usize,
    used: &mut [bool],
    g_rev: &'a G,
    cur_comp_id: usize,
    comp_id: &mut [usize],
) where
    G: GraphTrait<'a, SimpleEdge>,
{
    let mut stack = vec![];
    stack.push(State {
        v: v as i32,
        g_pos: 0,
    });
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        if g_rev.adj(cur_state.v as usize).count() == cur_state.g_pos as usize {
            comp_id[cur_state.v as usize] = cur_comp_id;
            stack.pop();
            continue;
        }
        let next = g_rev
            .adj(cur_state.v as usize)
            .skip(cur_state.g_pos as usize)
            .next()
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

fn rev_graph<'a, G>(graph: &'a G) -> impl GraphTrait<SimpleEdge>
where
    G: GraphTrait<'a, SimpleEdge>,
{
    let rev_edges: Vec<_> = all_graph_edges(graph)
        .map(|(fr, edge)| (edge.to(), SimpleEdge::new(fr)))
        .collect();
    SimpleGraphT::with_edges(graph.num_vertices(), &rev_edges)
}

// TODO: usize -> u32
pub fn find_strongly_connected_component<'a, G>(graph: &'a G) -> Vec<usize>
where
    G: GraphTrait<'a, SimpleEdge>,
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
