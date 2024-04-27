//{"name":"F. Конференция","group":"Codeforces - Codeforces Round 941 (Div. 1)","url":"https://codeforces.com/contest/1965/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n1 2\n3 4\n5 6\n","output":"6\n2\n0\n"},{"input":"5\n1 3\n1 3\n1 3\n1 3\n1 3\n","output":"3\n2\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKonferentsiya"}}}

use std::cmp::min;
use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::flows::dinic::FlowDinic;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve_fast(n: usize, segs: &[(usize, usize)]) -> Vec<i64> {
    let max_pos = segs.iter().map(|&(_l, r)| r).max().unwrap() + 10;
    let mut start_here = vec![0; max_pos];
    let mut end_here = vec![0; max_pos];
    for &(l, r) in segs.iter() {
        start_here[l] += 1i64;
        end_here[r] += 1i64;
    }
    let mut f = vec![-1i64; max_pos];
    let mut f_fenw = Fenwick::new(max_pos);
    let mut start_score = 0;
    let mut bad_from = vec![0; max_pos];
    let mut ways = vec![0i64; n + 1];
    let mut queue = VecDeque::new();
    // queue.push_front(max_pos);
    for start_day in (0..max_pos).rev() {
        start_score += end_here[start_day];
        let score = start_score + f[start_day];
        bad_from[start_day] = max_pos;
        dbg!(start_day, score);
        if score < 0 {
            bad_from[start_day] = start_day;
        } else {
            let first_bad_index = binary_search_first_true(0..queue.len(), |idx| {
                let day = queue[idx];
                f_fenw.get_sum(day) + score < 0
            });
            if first_bad_index != queue.len() {
                let day = queue[first_bad_index];
                bad_from[start_day] = day;
            }
        }
        if start_day + 1 < max_pos {
            bad_from[start_day] = min(bad_from[start_day], bad_from[start_day + 1]);
        }
        let max_len = bad_from[start_day] - start_day;
        ways[min(n, max_len)] += 1;
        f[start_day] += start_here[start_day];
        f_fenw.add(start_day, f[start_day]);
        start_score -= start_here[start_day];
        {
            let next_sum = f[start_day];
            while !queue.is_empty() {
                let first = queue[0];
                let fenw_sum = f_fenw.get_sum(first);
                if fenw_sum >= next_sum {
                    queue.pop_front();
                } else {
                    break;
                }
            }
            queue.push_front(start_day);
        }
    }
    dbg!(bad_from);
    for i in (0..n).rev() {
        ways[i] += ways[i + 1];
    }
    let mut res = vec![];
    for len in 1..=n {
        // out.println(ways[len]);
        res.push(ways[len]);
    }
    res
}

fn solve_slow(n: usize, segs: &[(usize, usize)]) -> Vec<i64> {
    let max_pos = segs.iter().map(|&(_l, r)| r).max().unwrap() + 2;
    let mut start_here = vec![0; max_pos];
    let mut end_here = vec![0; max_pos];
    for &(l, r) in segs.iter() {
        start_here[l] += 1;
        end_here[r] += 1;
    }
    let mut f = vec![-1i32; max_pos];
    let mut start_score = 0;
    let mut bad_from = vec![0; max_pos];
    let mut ways = vec![0; n + 1];
    for start_day in (0..max_pos).rev() {
        start_score += end_here[start_day];
        let mut score = start_score;
        bad_from[start_day] = max_pos;
        for day in start_day..max_pos {
            score += f[day];
            if score < 0 {
                bad_from[start_day] = day;
                break;
            }
        }
        if score > 0 {
            bad_from[start_day] = max_pos + score as usize;
        }
        if start_day + 1 < max_pos {
            bad_from[start_day] = min(bad_from[start_day], bad_from[start_day + 1]);
        }
        let max_len = bad_from[start_day] - start_day;
        ways[min(n, max_len)] += 1;
        f[start_day] += start_here[start_day];
        start_score -= start_here[start_day];
    }
    for i in (0..n).rev() {
        ways[i] += ways[i + 1];
    }
    let mut res = vec![];
    for len in 1..=n {
        // out.println(ways[len]);
        res.push(ways[len] as i64);
    }
    res
}

fn solve_very_slow(n: usize, segs: &[(usize, usize)]) -> Vec<i64> {
    let max_x = segs.iter().map(|&(_l, r)| r).max().unwrap() + 3;
    let mut res = vec![];
    for len in 1..=n {
        let mut ways = 0;
        for start in 0..max_x {
            let mut can = Array2D::new(false, len, segs.len());
            for i in 0..len {
                let day = start + i;
                for j in 0..segs.len() {
                    if day >= segs[j].0 && day <= segs[j].1 {
                        can[i][j] = true;
                    }
                }
            }
            let mut flow = FlowDinic::new(1 + len + segs.len() + 1);
            for i in 0..len {
                flow.add_edge(0, 1 + i, 1);
            }
            for i in 0..segs.len() {
                flow.add_edge(1 + len + i, 1 + len + segs.len(), 1);
            }
            for i in 0..len {
                for j in 0..segs.len() {
                    if can[i][j] {
                        flow.add_edge(1 + i, 1 + len + j, 1);
                    }
                }
            }
            if flow.find_flow() == len as i64 {
                ways += 1;
            }
        }
        res.push(ways);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let segs = gen_vec(n, |_| (input.usize(), input.usize()));
    let res = solve_fast(n, &segs);
    for &x in res.iter() {
        out.println(x);
    }
}

fn stress() {
    for it in 313.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..4);
        let mut segs = vec![];
        const MAX_X: usize = 4;
        for _ in 0..n {
            let mut l = rnd.gen(0..MAX_X);
            let mut r = rnd.gen(l..MAX_X);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            segs.push((l, r));
        }
        let slow = solve_very_slow(n, &segs);
        let fast = solve_fast(n, &segs);
        if slow != fast {
            dbg!(segs);
        }
        assert_eq!(slow, fast);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_konferentsiya";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
