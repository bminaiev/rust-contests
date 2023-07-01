use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

#[derive(Default)]
pub struct IdMap<T>
where
    T: Eq + Hash + Clone,
{
    map: HashMap<T, u32>,
    values: Vec<T>,
}

impl<T> IdMap<T>
where
    T: Eq + Hash + Clone,
{
    //noinspection RsSelfConvention
    pub fn get_or_add(&mut self, key: &T) -> usize {
        if let Some(&res) = self.map.get(key) {
            return res as usize;
        }
        let res = self.values.len() as u32;
        self.values.push(key.clone());
        self.map.insert(key.clone(), res);
        res as usize
    }

    pub fn get(&self, key: &T) -> Option<usize> {
        self.map.get(key).map(|&id| id as usize)
    }

    pub fn get_exn(&self, key: &T) -> usize {
        self.get(key).expect("Key not found")
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.values.iter().enumerate()
    }
}

impl<T> Index<usize> for IdMap<T>
where
    T: Eq + Hash + Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
