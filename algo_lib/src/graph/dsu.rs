#[derive(Eq, PartialEq, Copy, Clone)]
pub struct DsuGroupId(u32);

impl DsuGroupId {
    fn new(v: usize) -> Self {
        Self(v as u32)
    }
}

pub struct Dsu {
    p: Vec<DsuGroupId>,
    size: Vec<u32>,
}

impl Dsu {
    pub fn clear(&mut self) {
        for (idx, val) in self.p.iter_mut().enumerate() {
            *val = DsuGroupId::new(idx);
        }
        for val in self.size.iter_mut() {
            *val = 1;
        }
    }

    pub fn new(n: usize) -> Self {
        let mut res = Self {
            p: vec![DsuGroupId::new(0); n],
            size: vec![0; n],
        };
        res.clear();
        res
    }

    pub fn get(&mut self, v: usize) -> DsuGroupId {
        if self.p[v].0 as usize != v {
            self.p[v] = self.get(self.p[v].0 as usize);
        }
        self.p[v]
    }

    pub fn unite(&mut self, v1: usize, v2: usize) -> bool {
        let group1 = self.get(v1);
        let group2 = self.get(v2);
        if group1 == group2 {
            false
        } else {
            self.p[group1.0 as usize] = group2;
            self.size[group2.0 as usize] += self.size[group1.0 as usize];
            true
        }
    }

    pub fn get_size(&self, group_id: DsuGroupId) -> usize {
        self.size[group_id.0 as usize] as usize
    }

    pub fn is_root(&self, v: usize) -> bool {
        self.p[v].0 as usize == v
    }

    pub fn calc_components(&mut self) -> Vec<Vec<usize>> {
        let n = self.p.len();
        let mut res = vec![vec![]; n];
        for v in 0..n {
            res[self.get(v).0 as usize].push(v);
        }
        res.into_iter().filter(|vec| !vec.is_empty()).collect()
    }
}
