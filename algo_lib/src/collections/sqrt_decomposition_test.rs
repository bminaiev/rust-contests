#[cfg(test)]
pub mod tests {
    use crate::collections::sqrt_decomposition::{Part, SqrtDecomposition, SqrtNode};
    use crate::misc::rand::Random;
    use std::time::Instant;

    #[derive(Clone)]
    struct SumNode {
        raw_values: Vec<i64>,
        total_sum: i64,
        add_to_each: i64,
    }

    impl SumNode {
        pub fn new(a: &[i64]) -> Self {
            Self {
                raw_values: a.to_vec(),
                total_sum: a.iter().sum::<i64>(),
                add_to_each: 0,
            }
        }
    }

    impl SqrtNode for SumNode {
        fn relax(&mut self) {
            for val in self.raw_values.iter_mut() {
                *val += self.add_to_each;
            }
            self.add_to_each = 0;
        }

        fn rebuild(&mut self) {
            self.total_sum = self.raw_values.iter().sum();
        }
    }

    #[test]
    pub fn speed() {
        let n = 500_000;
        const MAX_VAL: i64 = 1_000_000;
        const BLOCK_SIZE: usize = 333;
        for test_id in 0..5 {
            let mut rnd = Random::new(787788 + test_id);
            let a = rnd.gen_vec(n, 0..MAX_VAL);
            let mut sqrt = SqrtDecomposition::new(n, BLOCK_SIZE, |range| SumNode::new(&a[range]));
            let mut xor = 0;
            let start = Instant::now();
            for _ in 0..n {
                let full_range = rnd.gen_nonempty_range(n);
                if rnd.gen_bool() {
                    // add to elements
                    let add = rnd.gen_range(0..MAX_VAL);
                    sqrt.iter_mut(full_range, |p| match p {
                        Part::Full(node) => {
                            node.add_to_each += add;
                            node.total_sum += add * (node.raw_values.len() as i64);
                        }
                        Part::Range(node, range) => {
                            node.total_sum += (range.len() as i64) * add;
                            for x in node.raw_values[range].iter_mut() {
                                *x += add;
                            }
                        }
                    });
                } else {
                    let mut res = 0;
                    sqrt.iter_mut(full_range, |p| match p {
                        Part::Full(node) => {
                            res += node.total_sum;
                        }
                        Part::Range(node, range) => {
                            for x in node.raw_values[range].iter() {
                                res += *x;
                            }
                        }
                    });
                    xor ^= res;
                }
            }
            println!("xor = {}, time = {:?}", xor, start.elapsed().as_millis());
            if test_id == 0 {
                // 530 - 550ms
                // 400 - 420ms
                assert_eq!(xor, 23762137452875360);
            }
        }
    }
}
