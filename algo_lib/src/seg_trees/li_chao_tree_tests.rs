#[cfg(test)]
mod tests {
    use crate::{
        misc::rand::Random,
        seg_trees::li_chao_tree::{LiChaoFunction, LiChaoTree},
    };

    #[derive(Clone)]
    struct Line {
        a: i64,
        b: i64,
    }

    impl LiChaoFunction<i64> for Line {
        type Value = i64;

        fn eval(&self, x: i64) -> Self::Value {
            self.a * x + self.b
        }
    }

    #[test]
    fn lines_stress() {
        let mut rnd = Random::new(787788);
        for _ in 0..100 {
            let mut xs = vec![];
            while xs.len() < 50 {
                xs.push(rnd.gen_range(-100..101));
                xs.sort();
                xs.dedup();
            }
            let mut tree = LiChaoTree::new(xs.clone());
            let mut lines = vec![];
            for _ in 0..100 {
                let line = Line {
                    a: rnd.gen_range(-100..101),
                    b: rnd.gen_range(-1000..1001),
                };
                tree.add(line.clone());
                lines.push(line);
                for &x in xs.iter() {
                    let expected = lines.iter().map(|line| line.eval(x)).min().unwrap();
                    assert_eq!(tree.query(x), expected);
                }
            }
        }
    }
}
