//{"name":"F. Бурёнка, массив и запросы","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5 5 5 3\n1 2 3 2 5\n1 1\n2 4\n4 5\n","output":"5\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBuryonkaMassivIZaprosi"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::primes::gen_largest_prime_table;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::simd::apply_fast::fast_apply;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Query {
    r: usize,
    id: usize,
}

fn prefetch(ptr: *const i8) {
    unsafe {
        core::arch::x86_64::_mm_prefetch::<{ core::arch::x86_64::_MM_HINT_T0 }>(ptr);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn update_first(first: &mut [u32], mask: &[bool], new_value: u32) {
    let n = first.len();
    let first_bad = &mut first[..n];
    let cache = &mask[..n];
    const C: usize = 1024;
    for start in (0..n).step_by(C) {
        for i in start..min(start + C, n) {
            if cache[i] {
                first_bad[i] = new_value;
            }
        }
        for it in 0..C / 64 {
            prefetch(cache.as_ptr().add(start + C + it * 64) as *const i8);
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn update_first_two_masks(first: &mut [u32], pos: u32, mask1: &[bool], mask2: &[bool]) {
    let n = first.len();
    let first_bad = &mut first[..n];
    let mask1 = &mask1[..n];
    let mask2 = &mask2[..n];
    const C: usize = 2048;
    for start in (0..n).step_by(C) {
        for i in start..min(n, start + C) {
            if mask1[i] | mask2[i] {
                first_bad[i] = pos;
            }
        }
        for it in 0..C / 64 {
            prefetch(mask1.as_ptr().add(start + C + it * 64) as *const i8);
            prefetch(mask2.as_ptr().add(start + C + it * 64) as *const i8);
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn add_single_prime(cache: &mut [bool], prime: usize) {
    let last = 1 + (cache.len() - 1) / prime;
    for i in 1..last {
        *cache.get_unchecked_mut(i * prime) = true
    }
}

#[target_feature(enable = "avx2")]
unsafe fn or(cache: &mut [bool], cache2: &[bool]) {
    for (c1, c2) in cache.iter_mut().zip(cache2.iter()) {
        *c1 |= c2;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn calc_res(first: &[u32], r: u32) -> i32 {
    first.iter().map(|x| (*x <= r) as i32).sum()
}

fn remove_same_primes(mut x: usize, largest_prime: &[usize]) -> usize {
    let mut res = 1;
    while x != 1 {
        let p = largest_prime[x];
        res *= p;
        while x % p == 0 {
            x /= p;
        }
    }
    res
}

fn solve_case(queries: &[Vec<Query>], a: &[usize], max_v: usize) -> Vec<i32> {
    let n = a.len();
    let mut first = vec![n as u32; max_v + 1];
    let m = *a.iter().max().unwrap();

    const MAX_MASK: usize = 2400;
    let largest_prime = gen_largest_prime_table(max(MAX_MASK, m) + 1);

    let mut mask = vec![vec![false; max_v + 1]; MAX_MASK];
    for v in 2..mask.len() {
        if remove_same_primes(v, &largest_prime) != v {
            continue;
        }
        let p = largest_prime[v];
        unsafe {
            add_single_prime(&mut mask[v], p);
            let (c1, c2) = mask.split_at_mut(v);
            or(&mut c2[0], &c1[v / p]);
        }
    }
    let mut q = 0;
    for it in queries.iter() {
        for it in it.iter() {
            q.update_max(it.id + 1);
        }
    }
    let mut res = vec![0; q];
    let mut a = a.to_vec();
    fast_apply(&mut a, |x| remove_same_primes(x, &largest_prime));

    for l in (0..n).rev() {
        let cur = a[l];

        unsafe {
            let l = l as u32;
            if cur < mask.len() {
                update_first(&mut first, &mask[cur], l)
            } else {
                let p = largest_prime[cur];

                if p < 30 {
                    update_first_two_masks(&mut first, l, &mask[p], &mask[cur / p])
                } else {
                    update_first(&mut first, &mask[cur / p], l);
                    for i in (p..first.len()).step_by(p) {
                        first[i] = l;
                    }
                }
            }
        }

        for query in queries[l].iter() {
            res[query.id] = max_v as i32 - unsafe { calc_res(&first, query.r as u32) };
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    // stress();
    // if true {
    //     return;
    // }

    let n = input.usize();
    let m = input.usize();
    let max_v = input.usize();
    let q = input.usize();
    let mut queries = vec![vec![]; n];
    let a = input.vec::<usize>(n);
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize() - 1;
        queries[l].push(Query { id, r });
    }
    let res_cnt_coprime = solve_case(&queries, &a, max_v);
    out_line!(res_cnt_coprime);
}

fn stress() {
    let n = 100_000;
    let max_v = 100_000;
    let mut rnd = Random::new(333);
    let a = gen_vec(n, |_| rnd.gen(1..20_000));
    let mut queries = vec![vec![]; a.len()];
    for id in 0..100_000 {
        let range = rnd.gen_nonempty_range(n);
        queries[range.start].push(Query {
            id,
            r: range.end - 1,
        });
    }
    solve_case(&queries, &a, max_v);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_singleetest("1");
    // tester::run_stress(stress);
}
//END MAIN
