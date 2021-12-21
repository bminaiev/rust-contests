use crate::collections::bit_set::BitSet;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph_trait::GraphTrait;

pub struct DfsBuilder<'a, Graph, Edge, State>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    seen: BitSet,
    graph: &'a Graph,
    pub state: &'a mut State,
    on_enter_f: Box<dyn FnMut(usize, &Edge, &mut State) -> ()>,
    on_exit_f: Box<dyn FnMut(usize, &Edge, &mut State) -> ()>,
    stack: Vec<Frame<Edge>>,
}

enum Frame<Edge>
where
    Edge: EdgeTrait,
{
    CheckEdge(u32, Edge),
    ExitFrom(u32, Edge),
}

impl<'a, Graph, Edge, State> DfsBuilder<'a, Graph, Edge, State>
where
    Graph: GraphTrait<Edge>,
    Edge: EdgeTrait,
{
    pub fn new(graph: &'a Graph, state: &'a mut State) -> Self {
        let n = graph.num_vertices();
        Self {
            seen: BitSet::new(n),
            graph,
            state,
            on_enter_f: Box::new(|_, _, _| {}),
            on_exit_f: Box::new(|_, _, _| {}),
            stack: vec![],
        }
    }

    pub fn seen(&self, v: usize) -> bool {
        self.seen.get(v)
    }

    pub fn on_enter(mut self, f: impl FnMut(usize, &Edge, &mut State) -> () + 'static) -> Self {
        self.on_enter_f = Box::new(f);
        self
    }

    pub fn on_exit(mut self, f: impl FnMut(usize, &Edge, &mut State) -> () + 'static) -> Self {
        self.on_exit_f = Box::new(f);
        self
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
                            self.stack
                                .push(Frame::CheckEdge(edge.to() as u32, edge.clone()));
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
