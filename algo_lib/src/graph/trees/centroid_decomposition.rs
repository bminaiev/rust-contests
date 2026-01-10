#[derive(Clone, Copy)]
pub struct Remote {
    pub to: usize,
    pub dist: i64,
}

pub struct CentroidDecomposition {
    alive: Vec<bool>,
    size: Vec<usize>,
    pub ups: Vec<Vec<Remote>>,
    pub children: Vec<Vec<Remote>>,
}

impl CentroidDecomposition {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut res = Self {
            alive: vec![true; n],
            size: vec![0; n],
            ups: vec![vec![]; n],
            children: vec![vec![]; n],
        };
        res.rec(g, 0);
        res
    }

    fn rec(&mut self, g: &[Vec<usize>], mut root: usize) {
        self.calc_sizes(g, root, root);
        let full_size = self.size[root];
        let mut prev = root;
        loop {
            let mut found = false;
            for &to in &g[root] {
                if to != prev && self.alive[to] && self.size[to] * 2 > full_size {
                    prev = root;
                    root = to;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        self.alive[root] = false;
        self.build_paths(g, root, root, 0, root);
        for &to in &g[root] {
            if self.alive[to] {
                self.rec(g, to);
            }
        }
    }

    fn calc_sizes(&mut self, g: &[Vec<usize>], v: usize, p: usize) {
        self.size[v] = 1;
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.calc_sizes(g, to, v);
                self.size[v] += self.size[to];
            }
        }
    }

    fn build_paths(&mut self, g: &[Vec<usize>], v: usize, p: usize, dist: i64, centroid: usize) {
        self.ups[v].push(Remote { to: centroid, dist });
        self.children[centroid].push(Remote { to: v, dist });
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.build_paths(g, to, v, dist + 1, centroid);
            }
        }
    }
}
