pub struct Dsu {
    p: Vec<u32>,
    size: Vec<u32>,
    num_comps: u32,
}

impl Dsu {
    pub fn clear(&mut self) {
        for (idx, val) in self.p.iter_mut().enumerate() {
            *val = idx as u32;
        }
        for val in self.size.iter_mut() {
            *val = 1;
        }
        self.num_comps = self.p.len() as u32;
    }

    pub fn new(n: usize) -> Self {
        let mut res = Self {
            p: vec![0; n],
            size: vec![0; n],
            num_comps: n as u32,
        };
        res.clear();
        res
    }

    pub fn get(&mut self, v: usize) -> usize {
        if self.p[v] as usize != v {
            self.p[v] = self.get(self.p[v] as usize) as u32;
        }
        self.p[v] as usize
    }

    pub fn unite(&mut self, v1: usize, v2: usize) -> bool {
        let v1 = self.get(v1);
        let v2 = self.get(v2);
        if v1 == v2 {
            false
        } else {
            self.p[v1] = v2 as u32;
            self.size[v2] += self.size[v1];
            self.num_comps -= 1;
            true
        }
    }

    pub fn calc_size(&mut self, mut v: usize) -> usize {
        v = self.get(v);
        self.size[v] as usize
    }

    pub fn is_root(&self, v: usize) -> bool {
        self.p[v] as usize == v
    }

    pub fn calc_components(&mut self) -> Vec<Vec<usize>> {
        let n = self.p.len();
        let mut res = vec![vec![]; n];
        for v in 0..n {
            res[self.get(v)].push(v);
        }
        res.into_iter().filter(|vec| !vec.is_empty()).collect()
    }

    pub fn num_components(&self) -> usize {
        self.num_comps as usize
    }

    pub fn len(&self) -> usize {
        self.p.len()
    }
}
