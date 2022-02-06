use crate::misc::rand::Random;
use std::collections::HashMap;
use std::hash::Hash;

pub struct RandomBag<T> {
    max_items: usize,
    items: Vec<T>,
    items_pos: HashMap<T, usize>,
    rnd: Random,
}

impl<T: Eq + Hash + Clone> RandomBag<T> {
    pub fn new(max_items: usize, rand_seed: u64) -> Self {
        Self {
            max_items,
            items: vec![],
            items_pos: HashMap::new(),
            rnd: Random::new(rand_seed),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    fn swap_remove(&mut self, pos: usize) -> Option<T> {
        let res = self.items.swap_remove(pos);
        self.items_pos.remove(&res);
        if pos < self.items.len() {
            self.items_pos.insert(self.items[pos].clone(), pos);
        }
        Some(res)
    }

    // TODO: interior mutability for [rnd]?
    pub fn peek_random(&mut self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let pos = self.rnd.gen_in_range(0..self.items.len());
        Some(&self.items[pos])
    }

    pub fn pop_random(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let pos = self.rnd.gen_in_range(0..self.items.len());
        self.swap_remove(pos)
    }

    fn ensure_max_elements(&mut self) {
        while self.items.len() > self.max_items {
            self.pop_random();
        }
    }

    pub fn insert(&mut self, elem: T) {
        if self
            .items_pos
            .insert(elem.clone(), self.items.len())
            .is_none()
        {
            self.items.push(elem);
            self.ensure_max_elements();
        }
    }

    pub fn remove(&mut self, elem: &T) -> bool {
        if let Some(&pos) = self.items_pos.get(elem) {
            self.swap_remove(pos);
            true
        } else {
            false
        }
    }

    pub fn filter(&mut self, mut should_stay: impl FnMut(&T) -> bool) {
        let to_remove: Vec<_> = self
            .items
            .iter()
            .filter_map(|x| {
                if should_stay(x) {
                    None
                } else {
                    Some(x.clone())
                }
            })
            .collect();
        for elem in to_remove.iter() {
            self.remove(elem);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}
