#[cfg(test)]
pub mod tests {
    use crate::{
        collections::array_2d::Array2D,
        flows::{hungarian_algorithm::hungarian_algorithm, min_cost_max_flow::MinCostMaxFlow},
        misc::rand::Random,
    };

    pub fn solve_flow(a: &Array2D<i64>) -> Option<i64> {
        let n = a.len();
        let mut flow = MinCostMaxFlow::new(1 + n + n + 1);
        for i in 0..n {
            flow.add_edge(0, 1 + i, 1, 0);
            flow.add_edge(1 + n + i, 1 + n + n, 1, 0);
            for j in 0..n {
                if a[i][j] != std::i64::MAX {
                    flow.add_edge(1 + i, 1 + n + j, 1, a[i][j]);
                }
            }
        }
        let res = flow.find_min_cost_max_flow(0, 1 + n + n);
        if res.flow != n as i64 {
            None
        } else {
            Some(res.cost)
        }
    }

    pub fn solve_hungarian(a: &Array2D<i64>) -> Option<i64> {
        if let Some(res) = hungarian_algorithm(a) {
            let mut sum_cost = 0;
            for i in 0..a.len() {
                let col = res.column_per_row[i];
                assert_ne!(a[i][col], std::i64::MAX);
                sum_cost += a[i][col];
            }
            assert_eq!(res.min_cost, sum_cost);
            Some(res.min_cost)
        } else {
            None
        }
    }

    #[test]
    pub fn stress() {
        for it in 10..100 {
            dbg!(it);
            let mut rnd = Random::new(787788 + it);
            let n = rnd.gen(1..50usize);
            const MAX: i64 = std::i64::MAX;
            let max_cost = 1 + rnd.gen(1..1000i64);
            let mut a = Array2D::new(MAX, n, n);
            let thresh = rnd.gen_double();
            for i in 0..n {
                for j in 0..n {
                    if rnd.gen_double() < thresh {
                        a[i][j] = rnd.gen(0..max_cost);
                    }
                }
            }
            let flow_cost = solve_flow(&a);
            let hung_cost = solve_hungarian(&a);
            if flow_cost != hung_cost {
                dbg!(it, n);
                dbg!(flow_cost, hung_cost);
                dbg!(a);
            }
            assert_eq!(flow_cost, hung_cost);
        }
    }
}
