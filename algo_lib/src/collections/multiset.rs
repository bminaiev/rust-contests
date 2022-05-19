use std::collections::BTreeMap;

pub struct MultiSet<T> {
    set: BTreeMap<T, u32>,
    len: usize,
}

impl<T: Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            set: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: T) {
        *self.set.entry(key).or_default() += 1;
        self.len += 1;
    }

    pub fn remove(&mut self, key: &T) -> bool
    where
        T: Clone,
    {
        if let Some(&cnt) = self.set.get(key) {
            if cnt == 1 {
                self.set.remove(&key);
            } else {
                self.set.insert(key.clone(), cnt - 1);
            }
            self.len -= 1;
            return true;
        } else {
            return false;
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.set.iter().next().map(|(key, _cnt)| key)
    }

    pub fn last(&self) -> Option<&T> {
        self.set.iter().next_back().map(|(key, _cnt)| key)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn len_total(&self) -> usize {
        self.len
    }

    pub fn len_groups(&self) -> usize {
        self.set.len()
    }

    pub fn iter_counts(&self) -> impl Iterator<Item = (&T, usize)> {
        self.set.iter().map(|(key, val)| (key, *val as usize))
    }

    pub fn get_count(&self, key: &T) -> usize {
        *self.set.get(key).unwrap_or(&0) as usize
    }
}
