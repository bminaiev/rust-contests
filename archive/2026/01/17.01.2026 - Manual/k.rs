//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::sqrt_decomposition::{SqrtDecomposition, SqrtNode};
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

#[target_feature(enable = "avx2")]
fn add(a: &mut [i32], delta: i32) {
    for x in a.iter_mut() {
        *x += delta;
    }
}

#[target_feature(enable = "avx2")]
fn assign(a: &mut [i32], val: i32) {
    for x in a.iter_mut() {
        *x = val;
    }
}

#[derive(Clone, Default)]
struct Block {
    mins: Vec<i32>,
    maxs: Vec<i32>,
    set_to: Option<i32>,
    add: i32,
}

impl Block {
    fn len(&self) -> usize {
        self.mins.len()
    }
}

#[target_feature(enable = "avx2")]
fn calc_one_set(a: &[i32], set_value: i32) -> u64 {
    let mut res = 0u64;
    for &x in a {
        let tmp = (x as u64) * (set_value as u64);
        res = res.overflowing_add(tmp).0;
    }
    res
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn calc_avx2(a: &[i32], b: &[i32]) -> u64 {
    use core::arch::x86_64::*;

    let len = a.len();
    let mut i = 0usize;

    // Multiple accumulators reduce dependency-chain latency.
    let mut acc0 = _mm256_setzero_si256();
    let mut acc1 = _mm256_setzero_si256();
    let mut acc2 = _mm256_setzero_si256();
    let mut acc3 = _mm256_setzero_si256();

    // Unroll by 16 i32s per iteration.
    while i + 16 <= len {
        // First 8
        let va0 = _mm256_loadu_si256(a.as_ptr().add(i) as *const __m256i);
        let vb0 = _mm256_loadu_si256(b.as_ptr().add(i) as *const __m256i);

        let prod0_even = _mm256_mul_epi32(va0, vb0);
        let prod0_odd = _mm256_mul_epi32(_mm256_srli_epi64(va0, 32), _mm256_srli_epi64(vb0, 32));

        acc0 = _mm256_add_epi64(acc0, prod0_even);
        acc1 = _mm256_add_epi64(acc1, prod0_odd);

        // Next 8
        let va1 = _mm256_loadu_si256(a.as_ptr().add(i + 8) as *const __m256i);
        let vb1 = _mm256_loadu_si256(b.as_ptr().add(i + 8) as *const __m256i);

        let prod1_even = _mm256_mul_epi32(va1, vb1);
        let prod1_odd = _mm256_mul_epi32(_mm256_srli_epi64(va1, 32), _mm256_srli_epi64(vb1, 32));

        acc2 = _mm256_add_epi64(acc2, prod1_even);
        acc3 = _mm256_add_epi64(acc3, prod1_odd);

        i += 16;
    }

    // Combine accumulators
    let sum01 = _mm256_add_epi64(acc0, acc1);
    let sum23 = _mm256_add_epi64(acc2, acc3);
    let sum = _mm256_add_epi64(sum01, sum23);

    // Horizontal sum of 4x i64 lanes into u64 (wrapping)
    let mut lanes = [0i64; 4];
    _mm256_storeu_si256(lanes.as_mut_ptr() as *mut __m256i, sum);

    let mut res = 0u64;
    res = res.wrapping_add(lanes[0] as u64);
    res = res.wrapping_add(lanes[1] as u64);
    res = res.wrapping_add(lanes[2] as u64);
    res = res.wrapping_add(lanes[3] as u64);

    // Tail
    while i < len {
        let prod = (*a.get_unchecked(i) as i64) * (*b.get_unchecked(i) as i64);
        res = res.wrapping_add(prod as u64);
        i += 1;
    }

    res
}

#[target_feature(enable = "avx2")]
pub fn calc(a: &[i32], b: &[i32]) -> u64 {
    // let mut res = 0u64;
    // for (x, y) in a.iter().zip(b.iter()) {
    //     let tmp = (*x as u64) * (*y as u64);
    //     res = res.overflowing_add(tmp).0;
    // }
    // res
    unsafe { calc_avx2(a, b) }
}

impl SqrtNode for Block {
    type Value = i32;

    fn relax(&mut self, raw_values: &mut [Self::Value]) {
        if let Some(set_to) = self.set_to {
            unsafe {
                assign(raw_values, set_to);
            }
            self.set_to = None;
        }
        if self.add != 0 {
            unsafe {
                add(raw_values, self.add);
            }
            self.add = 0;
        }
    }

    fn rebuild(&mut self, raw_values: &[Self::Value]) {
        let mut cur_min = i32::MAX;
        let mut cur_max = i32::MIN;
        self.mins.resize(raw_values.len(), 0);
        self.maxs.resize(raw_values.len(), 0);
        for i in 0..raw_values.len() {
            cur_min = cur_min.min(raw_values[i]);
            cur_max = cur_max.max(raw_values[i]);
            self.mins[i] = cur_min;
            self.maxs[i] = cur_max;
        }
    }
}

struct Solver {
    sqrt: SqrtDecomposition<Block>,
    sum_ops: usize,
}

impl Solver {
    pub fn new(a: Vec<i32>, block_size: usize) -> Self {
        Self {
            sqrt: SqrtDecomposition::new(a, block_size, Block::default()),
            sum_ops: 0,
        }
    }

    pub fn add(&mut self, l: usize, r: usize, delta: i32) {
        self.sqrt.iter_mut(l..r, |part| match part {
            algo_lib::collections::sqrt_decomposition::Part::Full(block) => {
                for x in block.mins.iter_mut() {
                    *x += delta;
                }
                for x in block.maxs.iter_mut() {
                    *x += delta;
                }
                block.add += delta;
            }
            algo_lib::collections::sqrt_decomposition::Part::Single(_, value) => {
                *value += delta;
            }
        });
    }

    pub fn assign(&mut self, l: usize, r: usize, val: i32) {
        self.sqrt.iter_mut(l..r, |part| match part {
            algo_lib::collections::sqrt_decomposition::Part::Full(block) => {
                for x in block.mins.iter_mut() {
                    *x = val;
                }
                for x in block.maxs.iter_mut() {
                    *x = val;
                }
                block.set_to = Some(val);
                block.add = 0;
            }
            algo_lib::collections::sqrt_decomposition::Part::Single(_, value) => {
                *value = val;
            }
        });
    }

    pub fn query(&mut self, l: usize, r: usize) -> u64 {
        let mut cur_min = i32::MAX;
        let mut cur_max = i32::MIN;
        let mut res = 0u64;
        self.sqrt.iter_mut(l..r, |part| match part {
            algo_lib::collections::sqrt_decomposition::Part::Full(block) => {
                let len = block.len() as u64;

                let my_min_start =
                    binary_search_first_true(0..len as usize, |pos| block.mins[pos] < cur_min);

                let my_max_start =
                    binary_search_first_true(0..len as usize, |pos| block.maxs[pos] > cur_max);

                unsafe {
                    let len_global_min_max = my_min_start.min(my_max_start) as u64;
                    res = res
                        .overflowing_add(len_global_min_max * (cur_min as u64) * (cur_max as u64))
                        .0;
                    if my_min_start < my_max_start {
                        let tmp = calc_one_set(&block.mins[my_min_start..my_max_start], cur_max);
                        res = res.overflowing_add(tmp).0;
                    } else {
                        let tmp = calc_one_set(&block.maxs[my_max_start..my_min_start], cur_min);
                        res = res.overflowing_add(tmp).0;
                    }
                    {
                        let offset_my = my_min_start.max(my_max_start);
                        let tmp = calc(&block.mins[offset_my..], &block.maxs[offset_my..]);
                        res = res.overflowing_add(tmp).0;
                        self.sum_ops += block.len() - offset_my;
                    }
                }

                // for i in 0..len as usize {
                //     let now_min = cur_min.min(block.mins[i]);
                //     let now_max = cur_max.max(block.maxs[i]);
                //     res = res.overflowing_add((now_min as u64) * (now_max as u64)).0;
                // }

                let pos = block.len() - 1;
                cur_min = cur_min.min(block.mins[pos]);
                cur_max = cur_max.max(block.maxs[pos]);
            }
            algo_lib::collections::sqrt_decomposition::Part::Single(_, value) => {
                let now_min = cur_min.min(*value);
                let now_max = cur_max.max(*value);
                res = res.overflowing_add((now_min as u64) * (now_max as u64)).0;
                cur_min = now_min;
                cur_max = now_max;
            }
        });
        res
    }
}

fn test_speed3() {
    let n = 200_000;
    let mut rnd = Random::new(123);
    let mut a = rnd.gen_vec(n, 0..10000000);
    for i in 0..a.len() {
        if i % 2 == 0 {
            a[i] = 10000000 + i as i32;
        } else {
            a[i] = 10000000 - i as i32;
        }
    }
    let mut full_res = 0;
    for _ in 0..n {
        let res = unsafe { calc(&a, &a) };
        // let res2 = solver2.query(l, r);
        // assert_eq!(res, res2);
        full_res += res;
    }
    dbg!(full_res);
}

fn test_speed() {
    let n = 200_000;
    let mut rnd = Random::new(123);
    let mut a = rnd.gen_vec(n, 0..10000000);
    for i in 0..a.len() {
        if i % 2 == 0 {
            a[i] = 10000000 + i as i32;
        } else {
            a[i] = 10000000 - i as i32;
        }
    }
    let mut solver1 = Solver::new(a.clone(), 4096);
    // let mut solver2 = Solver::new(a, 128);
    let mut full_res = 0;
    for _ in 0..n {
        let q_type = rnd.gen_range(3..4);
        let l = rnd.gen_range(0..50);
        let r = rnd.gen_range(n - 50..n + 1);
        if q_type == 1 {
            let v = rnd.gen_range(-1000..1000);
            solver1.add(l, r, v);
            // solver2.add(l, r, v);
        } else if q_type == 2 {
            let v = rnd.gen_range(0..10000000);
            solver1.assign(l, r, v);
            // solver2.assign(l, r, v);
        } else {
            let res = solver1.query(l, r);
            // let res2 = solver2.query(l, r);
            // assert_eq!(res, res2);
            full_res ^= res;
        }
    }
    dbg!(full_res, solver1.sum_ops);
}

fn solve(input: &mut Input, out: &mut Output) {
    test_speed();
    // let n = input.usize();
    // let q = input.usize();
    // let a = input.vec::<i32>(n);
    // let mut solver = Solver::new(a, 1024);
    // for _ in 0..q {
    //     let type_q = input.usize();
    //     let l = input.usize() - 1;
    //     let r = input.usize();
    //     if type_q == 1 {
    //         let v = input.i32();
    //         solver.add(l, r, v);
    //     } else if type_q == 2 {
    //         let v = input.i32();
    //         solver.assign(l, r, v);
    //     } else {
    //         let res = solver.query(l, r);
    //         out.println(res);
    //     }
    // }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "k";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
