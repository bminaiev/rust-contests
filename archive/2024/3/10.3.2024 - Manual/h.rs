//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

#[derive(Clone, Copy)]
struct F {
    xor: u64,
    matrix: [u64; 64],
}

impl Default for F {
    fn default() -> Self {
        Self::identity()
    }
}

impl F {
    pub fn identity() -> Self {
        let mut matrix = [0; 64];
        for i in 0..64 {
            matrix[i] = 1 << i;
        }
        Self { xor: 0, matrix }
    }

    pub fn join(&self, other: &Self) -> F {
        let xor = other.apply(self.xor);
        let mut matrix = [0; 64];
        for i in 0..64 {
            let mut tmp = 0;
            for j in 0..64 {
                // if (self.matrix[i] >> j) & 1 == 1 {
                //     matrix[i] ^= other.matrix[j];
                // }
                let coef = (self.matrix[i] >> j) & 1;
                tmp ^= other.matrix[j] * coef;
            }
            matrix[i] = tmp;
        }
        F { xor, matrix }
    }

    pub fn apply(&self, x: u64) -> u64 {
        let mut res = self.xor;
        for i in 0..64 {
            if (x >> i) & 1 == 1 {
                res ^= self.matrix[i];
            }
        }
        res
    }

    pub fn gen_random(rnd: &mut Random) -> Self {
        let mut xor = rnd.gen_u64();
        let mut matrix = [0; 64];
        for i in 0..64 {
            matrix[i] = rnd.gen_u64();
        }
        F { xor, matrix }
    }
}

fn read_f(input: &mut Input) -> F {
    let mut xor = 0;
    let mut matrix = [0; 64];
    let num_turms = input.usize();
    for _i in 0..num_turms {
        let shift = input.usize();
        let op = input.usize();
        let aj = input.u64();
        for bit in 0..64 {
            let nbit = (bit + shift) % 64;
            if op == 0 {
                // or
                if ((aj >> nbit) & 1) == 0 {
                    matrix[bit] ^= 1 << nbit;
                } else {
                    xor ^= 1 << nbit;
                }
            } else {
                assert_eq!(op, 1);
                // and
                if ((aj >> nbit) & 1) == 1 {
                    matrix[bit] ^= 1 << nbit;
                }
            }
        }
    }
    xor ^= input.u64();
    F { xor, matrix }
}

impl SegTreeNode for F {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self {
        l.join(r)
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = ();
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let _c = input.usize();
    let g = gen_vec(n, |_| read_f(input));
    let mut st = SegTree::new(n, |i| g[i]);
    for _ in 0..q {
        let op = input.usize();
        if op == 0 {
            let l = input.usize() - 1;
            let r = input.usize();
            // let base = st.get(l..r);
            // let mut base = F::identity();
            // for i in l..r {
            //     base = base.join(&g[i]);
            // }
            let mut x = input.u64();
            st.visit(l..r, &mut |f| {
                x = f.apply(x);
            });
            // let res = base.apply(x);
            out.println(x);
        } else {
            assert_eq!(op, 1);
            let pos = input.usize() - 1;
            st.update_point(pos, read_f(input));
            // g[pos] = read_f(input);
        }
    }
}

fn stress() {
    for it in 1.. {
        let mut rnd = Random::new(it);
        dbg!(it);
        let n = 20_000;

        let start = Instant::now();

        let g = gen_vec(n, |_| F::gen_random(&mut rnd));
        let mut st = SegTree::new(n, |i| g[i]);
        for _ in 0..n {
            let l = rnd.gen(0..n);
            let r = rnd.gen(l + 1..n + 1);
            // let base = st.get(l..r);
            let mut x = rnd.gen_u64();
            st.visit(l..r, &mut |f| {
                x = f.apply(x);
            });
        }
        dbg!(start.elapsed());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
