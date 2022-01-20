#[cfg(test)]
pub mod tests {
    use crate::collections::sqrt_decomposition::{Part, SqrtDecomposition, SqrtNode};
    use crate::misc::rand::Random;
    use std::time::Instant;

    #[derive(Clone, Default)]
    struct SumNode {
        total_sum: i64,
        add_to_each: i64,
        len: usize,
    }

    impl SqrtNode for SumNode {
        type Value = i64;

        fn relax(&mut self, raw_values: &mut [i64]) {
            for val in raw_values.iter_mut() {
                *val += self.add_to_each;
            }
            self.add_to_each = 0;
        }

        fn rebuild(&mut self, raw_values: &[i64]) {
            self.len = raw_values.len();
            self.total_sum = raw_values.iter().sum();
        }
    }

    #[test]
    pub fn speed() {
        let n = 100_000;
        const MAX_VAL: i64 = 1_000_000;
        const BLOCK_SIZE: usize = 333;
        for test_id in 0..5 {
            let mut rnd = Random::new(787788 + test_id);
            let a = rnd.gen_vec(n, 0..MAX_VAL);
            let mut sqrt = SqrtDecomposition::new(a, BLOCK_SIZE, SumNode::default());
            let mut xor = 0;
            let start = Instant::now();
            for _ in 0..n {
                let full_range = rnd.gen_nonempty_range(n);
                if rnd.gen_bool() {
                    // add to elements
                    let add = rnd.gen_range(0..MAX_VAL);
                    sqrt.iter_mut(full_range, |p: Part<SumNode>| match p {
                        Part::Full(node) => {
                            node.add_to_each += add;
                            node.total_sum += add * (node.len as i64);
                        }
                        Part::Single(_node, value) => {
                            *value += add;
                        }
                    });
                } else {
                    let mut res = 0;
                    sqrt.iter_mut(full_range, |p| match p {
                        Part::Full(node) => {
                            res += node.total_sum;
                        }
                        Part::Single(_node, value) => {
                            res += *value;
                        }
                    });
                    xor ^= res;
                }
            }
            println!("xor = {}, time = {:?}", xor, start.elapsed().as_millis());
            if test_id == 0 {
                assert_eq!(xor, 181294495115647);
            }
        }
    }
}
