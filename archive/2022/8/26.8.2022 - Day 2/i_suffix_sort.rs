//{"name":"I. Suffix Sort","group":"Yandex - Day 2","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39547/problems/I/","interactive":false,"timeLimit":6000,"tests":[{"input":"6\naadead\n","output":"6 1 5 4 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ISuffixSort"}}}

use std::cmp::{min, Ordering};
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::suffix_array::SuffixArray;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Next {
    pos: usize,
    value: usize,
}

const C: usize = 26;

fn solve_fast(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut next = Array2D::new(n, C, n + 1);
    for i in (0..n).rev() {
        for j in 0..C {
            next[j][i] = next[j][i + 1];
        }
        next[a[i]][i] = i;
    }

    let get_mapping = |from: usize| -> Vec<usize> {
        let mut nexts = gen_vec(C, |id| Next {
            pos: next[id][from],
            value: id,
        });
        nexts.sort_unstable();

        let mut id = vec![0; C];
        for i in 0..C {
            id[nexts[i].value] = i;
        }
        id
    };

    let mut mappings = Array2D::new(0, n, C);
    let mut rev_mappings = Array2D::new(0, n, C);
    for pos in 0..n {
        let mapp = get_mapping(pos);
        for j in 0..C {
            mappings[pos][j] = mapp[j];
            rev_mappings[pos][mapp[j]] = j;
        }
    }

    let mut dists = vec![];
    let mut index_in_dists = vec![0; n];
    let mut fake = n + 10;
    for c in 0..C {
        let mut positions = vec![];
        for i in 0..n {
            if a[i] == c {
                positions.push(i);
            }
        }
        positions.push(n);
        for w in positions.windows(2) {
            index_in_dists[w[0]] = dists.len();
            dists.push(w[1] - w[0]);
        }
        dists.push(fake);
        fake += 1;
    }
    let mut dists_pref_sum = vec![0];
    for x in dists.iter() {
        let nxt = x + dists_pref_sum.last_exn();
        dists_pref_sum.push(nxt);
    }
    let sa = SuffixArray::new(dists.clone());

    let calc_lcp_two_chars = |c1: usize, c2: usize, p1: usize, p2: usize| {
        if p1 == n || p2 == n {
            return 0;
        }
        assert!(a[p1] == c1);
        assert!(a[p2] == c2);
        let i1 = index_in_dists[p1];
        let i2 = index_in_dists[p2];
        let lcp = sa.lcp(sa.get_pos_in_array(i1), sa.get_pos_in_array(i2));
        let sum = dists_pref_sum[i1 + lcp] - dists_pref_sum[i1];
        let d1 = dists[i1 + lcp];
        let d2 = dists[i2 + lcp];
        if d1 > n || d2 > n {
            return sum;
        }
        let d = min(d1, d2);
        sum + d
    };

    let calc_lcp = |p1: usize, p2: usize, map1: &[usize], map2: &[usize]| -> usize {
        assert!(p1 != p2);
        let max_len = min(n - p1, n - p2);
        let mut res = max_len;
        for i in 0..C {
            let c1 = map1[i];
            let c2 = map2[i];
            let d1 = next[c1][p1] - p1;
            let d2 = next[c2][p2] - p2;
            if d1 != d2 {
                let lcp = min(d1, d2);
                if lcp < res {
                    res = lcp;
                }
            } else {
                let lcp = calc_lcp_two_chars(c1, c2, p1 + d1, p2 + d2) + d1;
                if lcp < res {
                    res = lcp;
                }
            }
        }
        res
    };

    let cmp = |p1: &usize, p2: &usize| -> Ordering {
        let p1 = *p1;
        let p2 = *p2;
        let map1 = &mappings[p1];
        let map2 = &mappings[p2];
        let max_len = min(n - p1, n - p2);

        let lcp = calc_lcp(p1, p2, &rev_mappings[p1], &rev_mappings[p2]);

        if lcp == max_len {
            return p1.cmp(&p2).reverse();
        }
        let val1 = map1[a[p1 + lcp]];
        let val2 = map2[a[p2 + lcp]];

        val1.cmp(&val2)
    };
    let mut ids = gen_vec(n, id);
    ids.sort_by(cmp);
    ids
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let a: Vec<_> = s.iter().map(|x| (*x - b'a') as usize).collect();
    let ids = solve_fast(&a);
    for i in ids.into_iter() {
        out!(i + 1, "");
    }
    out_line!();
}

fn stress() {
    const MAX_C: usize = 26;
    const MAX_N: usize = 200_000;
    for it in 1..2 {
        dbg!(it);
        let mut rnd = Random::new(787788);
        let n = rnd.gen(1..MAX_N);
        let a = gen_vec(n, |_| rnd.gen(0..MAX_C));
        // let slow = solve_slow(&a);
        let start = Instant::now();
        let fast = solve_fast(&a);
        dbg!(start.elapsed());
        // if slow != fast {
        //     dbg!(a);
        //     dbg!(slow);
        //     dbg!(fast);
        // }
        // assert!(slow == fast);
    }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
