pub trait LiChaoFunction<X: Copy>: Clone {
    type Value: Ord + Copy;

    fn eval(&self, x: X) -> Self::Value;
}

/// Li Chao tree for functions where any two functions intersect at most once.
pub struct LiChaoTree<X: Ord + Copy, F: LiChaoFunction<X>> {
    xs: Vec<X>,
    tree: Vec<Option<F>>,
}

impl<X: Ord + Copy, F: LiChaoFunction<X>> LiChaoTree<X, F> {
    pub fn new(mut xs: Vec<X>) -> Self {
        xs.sort();
        xs.dedup();
        assert!(!xs.is_empty());
        let tree = vec![None; xs.len() * 4 + 5];
        Self { xs, tree }
    }

    pub fn add(&mut self, f: F) {
        self.add_(1, 0, self.xs.len(), f);
    }

    fn add_(&mut self, v: usize, l: usize, r: usize, mut f: F) {
        if self.tree[v].is_none() {
            self.tree[v] = Some(f);
            return;
        }

        let mid = (l + r) >> 1;
        let x_mid = self.xs[mid];
        let cur = self.tree[v].as_mut().unwrap();
        if f.eval(x_mid) < cur.eval(x_mid) {
            std::mem::swap(cur, &mut f);
        }
        if r - l == 1 {
            return;
        }

        let cur = self.tree[v].as_ref().unwrap();
        if f.eval(self.xs[l]) < cur.eval(self.xs[l]) {
            self.add_(v << 1, l, mid, f);
        } else {
            self.add_(v << 1 | 1, mid, r, f);
        }
    }

    pub fn query(&self, x: X) -> F::Value {
        let pos = self.xs.binary_search(&x).unwrap();
        self.query_(1, 0, self.xs.len(), pos).unwrap()
    }

    fn query_(&self, v: usize, l: usize, r: usize, pos: usize) -> Option<F::Value> {
        let mut res = self.tree[v].as_ref().map(|f| f.eval(self.xs[pos]));
        if r - l != 1 {
            let mid = (l + r) >> 1;
            let child = if pos < mid { v << 1 } else { v << 1 | 1 };
            let child_res = if pos < mid {
                self.query_(child, l, mid, pos)
            } else {
                self.query_(child, mid, r, pos)
            };
            if let Some(child_res) = child_res {
                if res.map_or(true, |res| child_res < res) {
                    res = Some(child_res);
                }
            }
        }
        res
    }
}
