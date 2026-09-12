pub struct TwoSat {
    g: Vec<Vec<usize>>,
    g_rev: Vec<Vec<usize>>,
    restrictions: Vec<(usize, bool, usize, bool)>,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; 2 * n],
            g_rev: vec![vec![]; 2 * n],
            restrictions: vec![],
        }
    }

    // If u is u_val, then v is v_val
    pub fn add_edge(&mut self, u: usize, u_val: bool, v: usize, v_val: bool) {
        self.restrictions.push((u, u_val, v, v_val));
        let u = 2 * u + (u_val as usize);
        let v = 2 * v + (v_val as usize);
        self.g[u].push(v);
        self.g_rev[v].push(u);
        self.g[v ^ 1].push(u ^ 1);
        self.g_rev[u ^ 1].push(v ^ 1);
    }

    pub fn find_solution(&self) -> Option<Vec<bool>> {
        let n = self.g.len();
        let mut order = vec![];
        let mut used = vec![false; n];
        for i in 0..n {
            if !used[i] {
                self.dfs1(i, &mut used, &mut order);
            }
        }
        let mut comp = vec![0; n];
        let mut used = vec![false; n];
        let mut c = 0;
        for &v in order.iter().rev() {
            if !used[v] {
                self.dfs2(v, c, &mut used, &mut comp);
                c += 1;
            }
        }
        for i in 0..n / 2 {
            if comp[2 * i] == comp[2 * i + 1] {
                return None;
            }
        }
        let mut res = vec![false; n / 2];
        for i in 0..n / 2 {
            res[i] = comp[2 * i + 1] > comp[2 * i];
        }
        for &(u, u_val, v, v_val) in self.restrictions.iter() {
            if res[u] == u_val {
                assert!(res[v] == v_val);
            }
        }
        Some(res)
    }

    fn dfs1(&self, v: usize, used: &mut [bool], order: &mut Vec<usize>) {
        used[v] = true;
        let mut stack = vec![(v, 0)];
        while !stack.is_empty() {
            let last = stack.len() - 1;
            let (v, next_edge) = stack[last];
            if next_edge == self.g[v].len() {
                order.push(v);
                stack.pop();
            } else {
                let u = self.g[v][next_edge];
                stack[last].1 += 1;
                if !used[u] {
                    used[u] = true;
                    stack.push((u, 0));
                }
            }
        }
    }

    fn dfs2(&self, v: usize, c: usize, used: &mut [bool], comp: &mut [usize]) {
        used[v] = true;
        let mut stack = vec![v];
        while let Some(v) = stack.pop() {
            comp[v] = c;
            for &u in self.g_rev[v].iter() {
                if !used[u] {
                    used[u] = true;
                    stack.push(u);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TwoSat;

    #[test]
    fn exhaustive_two_variables() {
        let n = 2;
        let mut all_restrictions = vec![];
        for u in 0..n {
            for u_val in [false, true] {
                for v in 0..n {
                    for v_val in [false, true] {
                        all_restrictions.push((u, u_val, v, v_val));
                    }
                }
            }
        }

        for restrictions_mask in 0usize..1 << all_restrictions.len() {
            let restrictions: Vec<_> = all_restrictions
                .iter()
                .enumerate()
                .filter_map(|(i, &restriction)| {
                    ((restrictions_mask >> i) & 1 == 1).then_some(restriction)
                })
                .collect();

            let expected = (0usize..1 << n).find_map(|values_mask| {
                let values: Vec<_> = (0..n).map(|i| (values_mask >> i) & 1 == 1).collect();
                restrictions
                    .iter()
                    .all(|&(u, u_val, v, v_val)| values[u] != u_val || values[v] == v_val)
                    .then_some(values)
            });

            let mut two_sat = TwoSat::new(n);
            for &(u, u_val, v, v_val) in restrictions.iter() {
                two_sat.add_edge(u, u_val, v, v_val);
            }
            let actual = two_sat.find_solution();

            assert_eq!(actual.is_some(), expected.is_some());
        }
    }
}
