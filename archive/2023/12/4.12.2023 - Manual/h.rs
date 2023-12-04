//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use std::collections::{BTreeMap, HashSet};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn go(x: u64, seen: &mut HashSet<u64>) {
    if x > 1.5e15 as u64 {
        return;
    }
    if seen.contains(&x) {
        return;
    }
    seen.insert(x);
    for div in [2, 3, 4, 5, 7] {
        go(x * div, seen);
    }
}

fn stress() {
    let max_n = 1e15 as u64;
    let mut rnd = Random::new(787788);
    let mut queries = vec![];
    for it in 0..100_000 {
        let n = rnd.gen_u64() % max_n;
        // go(n, &mut hm);
        // dbg!(n, it, hm.len());
        queries.push(n);
    }
    let modu = 1e15 as u64;
    solve_case(123, 345, &queries, modu);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Query {
    x: u64,
    id: usize,
}

#[derive(Clone)]
pub struct Fenwick {
    values: Vec<[[u64; 2]; 2]>,
    modu: u64,
}

fn add(a: &mut [[u64; 2]; 2], b: &[[u64; 2]; 2], modu: u64) {
    for i in 0..2 {
        for j in 0..2 {
            a[i][j] += b[i][j];
            if a[i][j] >= modu {
                a[i][j] -= modu;
            }
        }
    }
}

impl Fenwick {
    #[allow(dead_code)]
    pub fn get_sum(&self, mut pos: usize) -> [[u64; 2]; 2] {
        let mut res = [[0u64; 2]; 2];
        loop {
            add(&mut res, &self.values[pos], self.modu);
            // dbg!("ADDING", pos, &self.values[pos]);
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: [[u64; 2]; 2]) {
        // dbg!("ADD", pos, change);
        while pos < self.values.len() {
            add(&mut self.values[pos], &change, self.modu);
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize, modu: u64) -> Self {
        let values = vec![[[0u64; 2]; 2]; n];
        Self { values, modu }
    }
}

fn solve_case(f0: u64, g0: u64, queries_prev: &[u64], modu: u64) -> Vec<[u64; 2]> {
    let mut queries = vec![];
    for id in 0..queries_prev.len() {
        queries.push(Query {
            x: queries_prev[id],
            id,
        });
    }
    queries.sort();
    let f_div = [2, 3, 5, 7];
    let g_div = [2, 3, 4, 5];
    let mut prec = vec![[f0, g0]];
    for i in 1..10 {
        let mut now = [0, 0];
        for &d in f_div.iter() {
            now[0] += prec[i / d][1]
        }
        for &d in g_div.iter() {
            now[1] += prec[i / d][0]
        }
        let nn = i as u64;
        now[0].update_max(nn);
        now[1].update_max(nn);
        prec.push(now);
    }
    for entry in prec.iter_mut() {
        for x in entry.iter_mut() {
            *x %= modu;
        }
    }
    for i in prec.len()..100 {
        let mut now = [0, 0];
        for &d in f_div.iter() {
            now[0] += prec[i / d][1]
        }
        for &d in g_div.iter() {
            now[1] += prec[i / d][0]
        }
        for x in now.iter_mut() {
            *x %= modu;
        }
        prec.push(now);
    }
    // for i in 0..10 {
    //     dbg!(i, prec[i]);
    // }

    let mut all_hm = HashSet::new();
    go(1, &mut all_hm);
    let mut all: Vec<_> = all_hm.into_iter().collect();
    all.sort();
    // dbg!(all.len());
    assert!(all[0] == 1);

    // let mut set = BTreeMap::new();
    let mut start = Array2D::new(0, 2, 2);

    let mut queue = vec![start.clone(); all.len()];
    start[0][0] = 1;
    start[1][1] = 1;
    queue[0] = start;
    let mut queue_it = 0;

    let mut fenw = Fenwick::new(all.len(), modu);
    fenw.add(0, [[1, 0], [0, 1]]);

    // set.insert(1u64, start);
    let mut res = vec![[0, 0]; queries.len()];
    for query in queries.iter() {
        loop {
            let x = all[queue_it];
            if (query.x / x) as usize >= 6 {
                let matrix = &queue[queue_it].clone();
                // set.remove(&x);
                for &d in f_div.iter() {
                    let d = d as u64;
                    for i in 0..2 {
                        let cur = matrix[0][i];
                        let npos = binary_search_first_true(0..all.len(), |pos| all[pos] >= x * d);
                        let entry = &mut queue[npos];
                        let mut zz = [[0; 2]; 2];
                        zz[1][i] = cur;
                        fenw.add(npos, zz);
                        entry[1][i] += cur;
                        entry[1][i] %= modu;
                    }
                }
                for &d in g_div.iter() {
                    let d = d as u64;
                    for i in 0..2 {
                        let cur = matrix[1][i];
                        let npos = binary_search_first_true(0..all.len(), |pos| all[pos] >= x * d);
                        let entry = &mut queue[npos];
                        entry[0][i] += cur;
                        entry[0][i] %= modu;
                        let mut zz = [[0; 2]; 2];
                        zz[0][i] = cur;
                        fenw.add(npos, zz);
                    }
                }
                queue_it += 1;
            } else {
                break;
            }
        }
        // dbg!(query);
        // dbg!(query, set.len());
        let mut cur_res = [0, 0];
        let mut from_queue_it = queue_it;
        while from_queue_it != all.len() {
            let x = all[from_queue_it];
            let next_queue_it = binary_search_first_true(from_queue_it..all.len(), |pos| {
                query.x / all[pos] != query.x / x
            });

            let mut matrix = fenw.get_sum(next_queue_it - 1);
            // dbg!("@@@", matrix);
            // if 2 + 2 == 4 {
            //     panic!();
            // }
            if from_queue_it != 0 {
                let sum_matrix2 = fenw.get_sum(from_queue_it - 1);
                for i in 0..2 {
                    for j in 0..2 {
                        matrix[i][j] += modu - sum_matrix2[i][j];
                        matrix[i][j] %= modu;
                    }
                }
            }
            // dbg!(from_queue_it, next_queue_it, matrix);
            // for idx in from_queue_it..next_queue_it {
            // let matrix = &queue[idx];
            for i in 0..2 {
                for j in 0..2 {
                    cur_res[i] += ((prec[(query.x / x) as usize][j] as u128)
                        * (matrix[j][i] as u128)
                        % (modu as u128)) as u64;
                }
            }
            // }

            from_queue_it = next_queue_it;
        }
        // for (&d, matrix) in set.iter() {
        //     for i in 0..2 {
        //         for j in 0..2 {
        //             cur_res[i] += ((prec[(query.x / d) as usize][j] as u128)
        //                 * (matrix[j][i] as u128)
        //                 % (modu as u128)) as u64;
        //         }
        //     }
        // }
        for x in cur_res.iter_mut() {
            *x %= modu;
        }
        res[query.id] = cur_res;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let f0 = input.u64();
    let g0 = input.u64();
    let tc = input.usize();
    let modu = input.u64();
    let queries = input.vec::<u64>(tc);
    let res = solve_case(f0, g0, &queries, modu);
    for &x in res.iter() {
        out_line!(x[0], x[1]);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
