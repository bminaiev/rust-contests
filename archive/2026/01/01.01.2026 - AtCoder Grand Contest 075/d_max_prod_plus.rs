//{"name":"D - Max Prod Plus","group":"AtCoder - AtCoder Grand Contest 075","url":"https://atcoder.jp/contests/agc075/tasks/agc075_d","interactive":false,"timeLimit":3000,"tests":[{"input":"3 5 4\n","output":"9\n"},{"input":"4 5 7\n","output":"66\n"},{"input":"123 456 789\n","output":"436486661\n"},{"input":"1000000000 10000 10000\n","output":"855626126\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    pos_of_smallest: usize,
    smallest_max: usize,
    suf_max: usize,
}

fn get_matrix(cache_key: &CacheKey) -> Array2D<Mod> {
    const SZ: usize = 5;
    let mut matrix = Array2D::new(Mod::ZERO, SZ, SZ);
    for seen in 0..SZ {
        if seen + 1 < matrix.len() {
            matrix[seen][seen + 1] = Mod::ONE;
        }
        if seen == 3 && cache_key.suf_max == 0 {
            continue;
        }
        let mut at_most = cache_key.smallest_max;
        if seen == 4 {
            at_most = cache_key.suf_max;
        } else if seen == 3 {
            at_most = cache_key.suf_max - 1;
        } else if seen <= cache_key.pos_of_smallest {
            at_most -= 1;
        }
        matrix[seen][seen] = Mod::new(at_most);
    }
    matrix
}

fn pos(key: &CacheKey) -> usize {
    key.pos_of_smallest * 1000000 + key.smallest_max * 10000 + key.suf_max
}

struct MyCache {
    map: Vec<Option<Mod>>,
}

impl MyCache {
    fn entry(&mut self, key: CacheKey) -> &mut Option<Mod> {
        &mut self.map[pos(&key)]
    }
}

fn calc_ways2(n: usize, maxes: &[usize], cache: &mut MyCache) -> Mod {
    let smallest_max = maxes[0].min(maxes[1]).min(maxes[2]);
    let mut pos_of_smallest = maxes.len() - 2;
    while maxes[pos_of_smallest] != smallest_max {
        pos_of_smallest -= 1;
    }
    let key = CacheKey {
        pos_of_smallest,
        smallest_max,
        suf_max: maxes[3],
    };
    let entry = cache.entry(key);
    if entry.is_none() {
        let matrix = get_matrix(&key);
        let powered = matrix.pown(n);
        let value = if maxes[3] == 0 {
            powered[0][3]
        } else {
            powered[0][maxes.len()]
        };
        *entry = Some(value);
    }
    entry.unwrap()
}

fn solve_smartish(n: usize, m: usize, max_res: usize) -> Mod {
    let mut cnt = Mod::ZERO;
    let mut cache = MyCache {
        map: vec![None; 5 * 100 * 10000],
    };

    for m1 in 1..=m {
        for m2 in 1..=m {
            if m1 * m2 > max_res {
                break;
            }
            for m3 in 1..=m {
                if m1 * m2 + m3 > max_res {
                    break;
                }
                let at_most = m1.min(m2).min(m3);
                for max_suf in 0..=at_most {
                    if m1.max(m2) * m3 + max_suf > max_res && max_suf != 0 {
                        break;
                    }
                    cnt += calc_ways2(n, &[m1, m2, m3, max_suf], &mut cache);
                }
            }
        }
    }
    // dbg!(cc, minn);
    cnt
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let max_res = input.usize();
    let cnt = solve_smartish(n, m, max_res);
    out.println(cnt);
}

// fn stress() {
//     const MAX_N: usize = 8;
//     for it in 17.. {
//         dbg!(it);
//         let mut rnd = Random::new(it);
//         let n = rnd.gen_range(3..MAX_N);
//         let m = rnd.gen_range(1..MAX_N);
//         let max_res = rnd.gen_range(1..MAX_N * MAX_N);
//         let smart = solve_smart(n, m, max_res);
//         let stupid = solve_stupid(n, m, max_res);
//         if smart != stupid {
//             dbg!(n, m, max_res, smart, stupid);
//             break;
//         }
//     }
// }

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_max_prod_plus";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
