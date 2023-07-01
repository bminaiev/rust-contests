use crate::collections::bit_set::BitSet;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub struct DfsBuilder<'a, Graph, Edge, State, FEnter, FExit>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
    FEnter: FnMut(usize, &Edge, &mut State),
    FExit: FnMut(usize, &Edge, &mut State),
{
    seen: BitSet,
    graph: &'a Graph,
    pub state: &'a mut State,
    on_enter_f: FEnter,
    on_exit_f: FExit,
    stack: Vec<Frame<Edge>>,
}

enum Frame<Edge>
where
    Edge: EdgeTrait,
{
    CheckEdge(u32, Edge),
    ExitFrom(u32, Edge),
}

pub fn dfs_builder<'a, Graph, Edge, State>(
    graph: &'a Graph,
    state: &'a mut State,
) -> DfsBuilder<
    'a,
    Graph,
    Edge,
    State,
    fn(usize, &Edge, &mut State) -> (),
    fn(usize, &Edge, &mut State) -> (),
>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    let n = graph.num_vertices();
    DfsBuilder {
        seen: BitSet::new(n),
        graph,
        state,
        on_enter_f: (|_, _, _| {}),
        on_exit_f: (|_, _, _| {}),
        stack: vec![],
    }
}

impl<'a, Graph, Edge, State, FEnter, FExit> DfsBuilder<'a, Graph, Edge, State, FEnter, FExit>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
    FEnter: FnMut(usize, &Edge, &mut State),
    FExit: FnMut(usize, &Edge, &mut State),
{
    pub fn on_exit<FExit2>(
        self,
        on_exit_f: FExit2,
    ) -> DfsBuilder<'a, Graph, Edge, State, FEnter, FExit2>
    where
        FExit2: FnMut(usize, &Edge, &mut State),
    {
        DfsBuilder {
            seen: self.seen,
            graph: self.graph,
            state: self.state,
            on_enter_f: self.on_enter_f,
            on_exit_f,
            stack: self.stack,
        }
    }

    pub fn on_enter<FEnter2>(
        self,
        on_enter_f: FEnter2,
    ) -> DfsBuilder<'a, Graph, Edge, State, FEnter2, FExit>
    where
        FEnter2: FnMut(usize, &Edge, &mut State),
    {
        DfsBuilder {
            seen: self.seen,
            graph: self.graph,
            state: self.state,
            on_enter_f,
            on_exit_f: self.on_exit_f,
            stack: self.stack,
        }
    }

    pub fn seen(&self, v: usize) -> bool {
        self.seen.get(v)
    }

    pub fn set_seen(&mut self, v: usize, value: bool) {
        self.seen.set(v, value)
    }

    pub fn run(&mut self, fake_edge: Edge) {
        assert!(self.stack.is_empty());
        self.stack
            .push(Frame::CheckEdge(fake_edge.to() as u32, fake_edge));
        while let Some(frame) = self.stack.pop() {
            match frame {
                Frame::CheckEdge(parent, edge) => {
                    if !self.seen.get(edge.to()) {
                        self.stack.push(Frame::ExitFrom(parent, edge));
                        self.seen.set(edge.to(), true);
                        (self.on_enter_f)(parent as usize, &edge, self.state);
                        for edge in self.graph.adj(edge.to()) {
                            self.stack.push(Frame::CheckEdge(edge.to() as u32, *edge));
                        }
                    }
                }
                Frame::ExitFrom(parent, edge) => {
                    (self.on_exit_f)(parent as usize, &edge, self.state);
                }
            }
        }
    }
}
