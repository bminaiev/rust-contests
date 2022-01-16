use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct NodeId(u32);

impl NodeId {
    pub fn id(&self) -> usize {
        self.0 as usize
    }

    pub const NONE: Self = Self(u32::MAX);
    pub const ROOT: Self = Self(0);

    pub fn id_opt(&self) -> Option<usize> {
        if self == &Self::NONE {
            None
        } else {
            Some(self.id())
        }
    }

    pub fn is_none(&self) -> bool {
        self == &Self::NONE
    }
    pub fn is_some(&self) -> bool {
        self != &Self::NONE
    }

    pub fn from_raw(x: usize) -> Self {
        Self(x as u32)
    }

    pub fn fmt_slice(elems: &[Self], f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (id, to) in elems.iter().enumerate() {
            if !to.is_none() {
                write!(f, "{} -> {:?}, ", id, to)?
            }
        }
        write!(f, "]")
    }
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_none() {
            write!(f, "None")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Clone)]
struct Node<T> {
    inner: T,
    next: Vec<NodeId>,
    parent: NodeId,
    parent_symbol: usize,
}

impl<T> Node<T> {
    pub fn new(parent: NodeId, parent_symbol: usize, alph_size: usize, inner: T) -> Self {
        Self {
            inner,
            next: vec![NodeId::NONE; alph_size],
            parent,
            parent_symbol,
        }
    }
}

impl<T> Debug for Node<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{inner : {:?}, parent : ({}, {:?}), next = [",
            self.inner, self.parent_symbol, self.parent
        )?;
        NodeId::fmt_slice(&self.next, f)?;
        write!(f, "]}}")
    }
}

#[derive(Debug)]
pub struct Trie<T> {
    nodes: Vec<Node<T>>,
    pub alph_size: usize,
    empty_node: T,
}

impl<T> Trie<T>
where
    T: Clone,
{
    pub fn new(alph_size: usize, empty_node: T) -> Self {
        let mut res = Self {
            nodes: vec![],
            alph_size,
            empty_node,
        };
        res.add_node(NodeId::ROOT, 0);
        res
    }

    fn add_node(&mut self, parent: NodeId, symbol: usize) -> NodeId {
        self.nodes.push(Node::new(
            parent,
            symbol,
            self.alph_size,
            self.empty_node.clone(),
        ));
        NodeId(self.nodes.len() as u32 - 1)
    }

    pub fn conv_char(&self, c: &u8) -> usize {
        (c - b'a') as usize
    }

    fn conv_back(&self, shift: usize) -> u8 {
        b'a' + (shift as u8)
    }

    pub fn get_full_word(&self, mut node_id: NodeId) -> Vec<u8> {
        let mut res = vec![];
        while node_id != NodeId::ROOT {
            res.push(self.conv_back(self.nodes[node_id.id()].parent_symbol));
            node_id = self.nodes[node_id.id()].parent;
        }
        res.reverse();
        res
    }

    pub fn add_string(&mut self, str: &[u8]) -> NodeId {
        let mut node_id = NodeId::ROOT;
        for c in str.iter() {
            let c = self.conv_char(c);
            if self.nodes[node_id.id()].next[c].is_none() {
                self.nodes[node_id.id()].next[c] = self.add_node(node_id, c);
            }
            node_id = self.nodes[node_id.id()].next[c];
        }
        node_id
    }

    pub fn get_parent(&self, v: NodeId) -> NodeId {
        self.nodes[v.id()].parent
    }

    pub fn get_parent_symbol(&self, v: NodeId) -> usize {
        self.nodes[v.id()].parent_symbol
    }

    pub fn all_node_ids(&self) -> impl Iterator<Item = NodeId> {
        (0..self.nodes.len()).map(|id| NodeId(id as u32))
    }

    pub fn next(&self, node_id: NodeId, symbol: usize) -> NodeId {
        assert!(symbol < self.alph_size);
        self.nodes[node_id.id()].next[symbol]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<T> Index<NodeId> for Trie<T> {
    type Output = T;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.id()].inner
    }
}

impl<T> IndexMut<NodeId> for Trie<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index.id()].inner
    }
}
