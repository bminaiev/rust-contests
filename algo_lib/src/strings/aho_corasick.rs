use crate::strings::trie::{NodeId, Trie};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
struct Node<T> {
    inner: T,
    suf_link: NodeId,
    next_automaton: Vec<NodeId>,
    depth: u32,
}

impl<T> Node<T>
where
    T: Default + Clone,
{
    pub fn new(alph_size: usize) -> Self {
        Self {
            inner: T::default(),
            suf_link: NodeId::NONE,
            next_automaton: vec![NodeId::NONE; alph_size],
            depth: 0,
        }
    }
}

#[derive(Debug)]
pub struct AhoCorasick<T> {
    trie: Trie<Node<T>>,
    nodes_by_depth: Vec<NodeId>,
}

impl<T> AhoCorasick<T>
where
    T: Default + Debug + Clone,
{
    fn calc_next(&mut self, node_id: NodeId, c: usize) -> NodeId {
        if self.trie[node_id].next_automaton[c].is_none() {
            let next = if self.trie.next(node_id, c).is_none() {
                if node_id == NodeId::ROOT {
                    NodeId::ROOT
                } else {
                    let suf_link = self.calc_suf_link(node_id);
                    self.calc_next(suf_link, c)
                }
            } else {
                self.trie.next(node_id, c)
            };
            self.trie[node_id].next_automaton[c] = next;
        }
        self.trie[node_id].next_automaton[c]
    }

    fn calc_suf_link(&mut self, node_id: NodeId) -> NodeId {
        if self.trie[node_id].suf_link.is_none() {
            let parent = self.get_parent(node_id);
            let suf_link = if node_id == NodeId::ROOT || parent == NodeId::ROOT {
                NodeId::ROOT
            } else {
                let parent_suf_link = self.calc_suf_link(parent);
                self.calc_next(parent_suf_link, self.get_parent_symbol(node_id))
            };
            self.trie[node_id].suf_link = suf_link;
        }
        self.trie[node_id].suf_link
    }

    pub fn new(alph_size: usize, strings: &[Vec<u8>]) -> Self {
        let empty_node = Node::new(alph_size);
        let mut trie: Trie<Node<T>> = Trie::new(alph_size, empty_node);
        for str in strings.iter() {
            trie.add_string(str);
        }
        let mut res = Self {
            trie,
            nodes_by_depth: vec![],
        };
        for node_id in res.trie.all_node_ids() {
            if node_id != NodeId::ROOT {
                res.trie[node_id].depth = res.trie[res.get_parent(node_id)].depth + 1;
            }
            res.calc_suf_link(node_id);
            for c in 0..alph_size {
                res.calc_next(node_id, c);
            }
        }
        let mut nodes_by_depth: Vec<_> = (0..res.len()).map(NodeId::from_raw).collect();
        nodes_by_depth.sort_by_key(|&id| res.trie[id].depth);
        res.nodes_by_depth = nodes_by_depth;
        res
    }

    pub fn go(&self, mut node_id: NodeId, str: &[u8]) -> NodeId {
        for c in str.iter().map(|c| self.trie.conv_char(c)) {
            node_id = self.trie[node_id].next_automaton[c];
        }
        node_id
    }

    pub fn all_node_ids(&self) -> Vec<NodeId> {
        self.nodes_by_depth.clone()
    }

    pub fn get_parent(&self, v: NodeId) -> NodeId {
        self.trie.get_parent(v)
    }

    pub fn get_suf_link(&self, v: NodeId) -> NodeId {
        self.trie[v].suf_link
    }

    pub fn get_parent_symbol(&self, v: NodeId) -> usize {
        self.trie.get_parent_symbol(v)
    }

    pub fn len(&self) -> usize {
        self.trie.len()
    }

    pub fn get_full_word(&self, node_id: NodeId) -> Vec<u8> {
        self.trie.get_full_word(node_id)
    }
}

impl<T> Index<NodeId> for AhoCorasick<T> {
    type Output = T;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.trie[index].inner
    }
}

impl<T> IndexMut<NodeId> for AhoCorasick<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.trie[index].inner
    }
}
