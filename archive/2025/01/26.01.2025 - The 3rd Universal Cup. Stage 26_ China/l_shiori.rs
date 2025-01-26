//{"name":"L. Shiori","group":"Universal Cup - The 3rd Universal Cup. Stage 26: China","url":"https://contest.ucup.ac/contest/1894/problem/9986","interactive":false,"timeLimit":1000,"tests":[{"input":"5 8\n0 7 2 1 0\n1 2 4 0\n2 1 3\n2 3 4\n3 1 3\n1 2 3 4\n3 1 4\n2 1 5\n3 2 5\n","output":"5\n11\n22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LShiori"}}}

use std::time::Instant;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::sqrt_decomposition::{Part, SqrtDecomposition, SqrtNode};
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

const MX: usize = 500_210;
const REAL_BLOCK_SIZE: usize = 800;

#[derive(Clone)]
struct Block {
    bitset: BitSet,
    sum: i64,
    n: i64,
    set_to_value: Option<i64>,
    add_to_every: i64,
}

impl Block {
    pub fn new() -> Self {
        Self {
            bitset: BitSet::new(MX),
            sum: 0,
            n: 0,
            set_to_value: None,
            add_to_every: 0,
        }
    }
}

impl SqrtNode for Block {
    type Value = i64;

    fn relax(&mut self, raw_values: &mut [Self::Value]) {
        if let Some(v) = self.set_to_value {
            for x in raw_values.iter_mut() {
                *x = v;
            }
        } else if self.add_to_every != 0 {
            for x in raw_values.iter_mut() {
                *x += self.add_to_every;
            }
        }
        self.set_to_value = None;
        self.add_to_every = 0;
    }

    fn rebuild(&mut self, raw_values: &[Self::Value]) {
        self.bitset.clear();
        for &x in raw_values.iter() {
            if x < MX as i64 {
                self.bitset.set(x as usize, true);
            }
        }
        self.add_to_every = 0;
        self.set_to_value = None;
        self.sum = raw_values.iter().sum();
        self.n = raw_values.len() as i64;
    }
}

struct Solver {
    sqrt: SqrtDecomposition<Block>,
    query_bitset: BitSet,
    block_size: usize,
}

impl Solver {
    pub fn new(a: Vec<i64>, block_size: usize) -> Self {
        let block_size = block_size.min(a.len());
        let sqrt = SqrtDecomposition::new(a, block_size, Block::new());
        Self {
            sqrt,
            query_bitset: BitSet::new(MX),
            block_size,
        }
    }

    fn set(&mut self, l: usize, r: usize, v: i64) {
        self.sqrt.iter_mut(
            l..r,
            |block| match block {
                Part::Full(block) => {
                    block.set_to_value = Some(v);
                    block.add_to_every = 0;
                    block.sum = v * block.n;
                }
                Part::Single(_block, single) => {
                    *single = v;
                }
            },
            true,
        );
    }

    fn add_mex(&mut self, l: usize, r: usize) {
        self.query_bitset.clear();
        self.sqrt.iter_mut(
            l..r,
            |block| match block {
                Part::Full(_block) => {
                    // NOTHING
                }
                Part::Single(_block, single) => {
                    if *single < MX as i64 {
                        self.query_bitset.set_true(*single as usize);
                    }
                }
            },
            false,
        );
        let mut mex = 0;
        for base in (0..).step_by(64) {
            let mut my = self.query_bitset.get_u64(base);
            // dbg!(self.block_size, my, l, r);
            self.sqrt.iter_mut_only_full(l..r, |block| {
                // dbg!("l..r", l, r);
                if let Some(v) = block.set_to_value {
                    if v >= base as i64 && v < (base as i64 + 64) {
                        my |= 1 << (v as usize - base);
                    }
                } else if block.add_to_every < (base + 64) as i64 {
                    if block.add_to_every <= base as i64 {
                        my |= block.bitset.get_u64(base - block.add_to_every as usize);
                        // dbg!("!!", my, base - block.add_to_every as usize);
                    } else {
                        let from0_mask = block.bitset.get_u64(0);
                        let need_first_bits = block.add_to_every as usize - base;
                        let from0_mask = from0_mask << need_first_bits;
                        my |= from0_mask;
                    }
                }
                my == u64::MAX
            });
            if my == u64::MAX {
                mex += 64;
            } else {
                for bit in 0..64 {
                    if (my & (1 << bit)) == 0 {
                        mex += bit;
                        break;
                    }
                }
                break;
            }
        }
        // dbg!(mex);
        // if mex != 0 {
        self.sqrt.iter_mut(
            l..r,
            |block| match block {
                Part::Full(block) => {
                    if let Some(v) = block.set_to_value {
                        block.set_to_value = Some(v + mex as i64);
                        block.sum = block.n * (v + mex as i64);
                    } else {
                        block.add_to_every += mex as i64;
                        block.sum += block.n * mex as i64;
                    }
                }
                Part::Single(_block, single) => {
                    *single += mex as i64;
                }
            },
            true,
        );
        // }
    }

    fn sum(&mut self, l: usize, r: usize) -> i64 {
        let mut res = 0;
        self.sqrt.iter_mut(
            l..r,
            |block| match block {
                Part::Full(block) => {
                    res += block.sum;
                }
                Part::Single(_block, single) => {
                    res += *single;
                }
            },
            true,
        );
        res
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i64>(n);
    const BLOCK_SIZE: usize = REAL_BLOCK_SIZE;
    let mut solver = Solver::new(a, BLOCK_SIZE);
    for _ in 0..m {
        let op_type = input.usize();
        let l = input.usize() - 1;
        let r = input.usize();
        if op_type == 1 {
            // set
            let v = input.i64();
            solver.set(l, r, v);
        } else if op_type == 2 {
            // calc mex and add
            solver.add_mex(l, r);
        } else if op_type == 3 {
            let res = solver.sum(l, r);
            out.println(res);
        } else {
            unreachable!();
        }
    }
}

fn stress2() {
    for it in 54.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 100;
        const MAX_V: i64 = 500;
        const MAX_M: usize = 100;
        let n = rnd.gen(1..MAX_N);
        let a = rnd.gen_vec(n, 0..MAX_V);
        // dbg!(a);
        let block_size1 = rnd.gen(1..MAX_N);
        let block_size2 = rnd.gen(1..MAX_N);
        // dbg!(block_size1, block_size2);
        let mut solver1 = Solver::new(a.clone(), block_size1);
        let mut solver2 = Solver::new(a.clone(), block_size2);
        for _ in 0..MAX_M {
            let op_type = rnd.gen(1..4);
            let l = rnd.gen(0..n);
            let r = rnd.gen(l + 1..n + 1);
            if op_type == 1 {
                // set
                let v = rnd.gen(0..MAX_V);
                // dbg!("set", l, r, v);
                solver1.set(l, r, v);
                solver2.set(l, r, v);
            } else if op_type == 2 {
                // calc mex and add
                // dbg!("add_mex", l, r);
                solver1.add_mex(l, r);
                solver2.add_mex(l, r);
            } else if op_type == 3 {
                // dbg!("sum", l, r);
                let res1 = solver1.sum(l, r);
                let res2 = solver2.sum(l, r);
                // dbg!(res1, res2);
                assert_eq!(res1, res2);
            } else {
                unreachable!();
            }
        }
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 500_000;
        const MAX_V: i64 = 500_000;
        const MAX_M: usize = 500_000;
        let n = MAX_N;
        let start = Instant::now();
        let a = rnd.gen_vec(n, 0..MAX_V);
        let mut solver = Solver::new(a.clone(), REAL_BLOCK_SIZE);
        for _ in 0..MAX_M {
            let op_type = rnd.gen(1..4);
            let l = rnd.gen(0..n);
            let r = rnd.gen(l + 1..n + 1);
            if op_type == 1 {
                // set
                let v = rnd.gen(0..MAX_V);
                solver.set(l, r, v);
            } else if op_type == 2 {
                solver.add_mex(l, r);
            } else if op_type == 3 {
                solver.sum(l, r);
            } else {
                unreachable!();
            }
        }
        dbg!(start.elapsed())
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "l_shiori";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress2);
    // run_locally(run)
}
//END MAIN
