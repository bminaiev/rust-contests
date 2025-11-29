//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BTreeSet;
use std::i64;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

#[derive(Clone, Debug)]
struct State {
    cur: [i64; 2],
    queries: BTreeSet<(usize, usize)>,
    res: i64,
}

fn solve_case(x: i64, s: &[i64], p: &[i64], starts_here: &[Vec<(usize, usize)>]) -> Vec<i64> {
    let mut p = p.to_vec();
    let n = s.len();
    p.push(0);
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];
    for i in 0..n {
        let c = if i % 2 == 0 { &mut a } else { &mut b };
        if p[i] > 0 && p[i + 1] >= 0 {
            c[i] = x;
            c[i + 1] = x;
        }
        if p[i] < 0 && p[i + 1] <= 0 {
            c[i] = 1;
            c[i + 1] = 1;
        }
    }
    let mut q = 0;
    for v in starts_here.iter() {
        q += v.len();
    }
    let mut queries_res = vec![0i64; q];
    let mut states: Vec<State> = vec![];
    for i in 0..=n {
        // dbg!(i);
        // dbg!(queries_res);
        for st in states.iter_mut() {
            // dbg!(st);
            while st.queries.len() > 0 {
                let &(r, qid) = st.queries.iter().next().unwrap();
                if r <= i {
                    st.queries.remove(&(r, qid));
                    queries_res[qid] += st.res;
                } else {
                    break;
                }
            }
        }
        if i == n {
            break;
        }
        if starts_here[i].len() > 0 {
            if states.len() == 0 || states[0].cur != [1, 1] {
                states.insert(
                    0,
                    State {
                        cur: [1, 1],
                        queries: BTreeSet::new(),
                        res: 0,
                    },
                );
            }
        }
        for (r, q_id) in starts_here[i].iter() {
            queries_res[*q_id] -= states[0].res;
            // dbg!("Add", q_id, queries_res[*q_id]);
            states[0].queries.insert((*r, *q_id));
        }
        for st in states.iter_mut() {
            let cur = &mut st.cur;
            let c = if i % 2 == 0 { &a } else { &b };
            let idx = i % 2;
            if c[i] != 0 {
                cur[idx] = c[i];
            } else if p[i] > 0 {
                // want to maximize
                let need = (s[i] + cur[1 - idx] - 1) / cur[1 - idx];
                if need <= x {
                    cur[idx] = need;
                } else {
                    cur[idx] = 1;
                }
            } else if cur[1 - idx] < s[i] {
                cur[idx] = ((s[i] + cur[1 - idx] - 1) / cur[1 - idx] - 1).min(x);
            } else {
                cur[idx] = x;
            }
            if cur[0] * cur[1] >= s[i] {
                st.res += p[i];
            }
        }
        states.sort_by_key(|st| st.cur);
        let mut next_states: Vec<State> = vec![];
        for s in states {
            if s.queries.len() == 0 {
                continue;
            }
            if next_states.len() == 0 || next_states.last().unwrap().cur != s.cur {
                next_states.push(s);
            } else {
                let last = next_states.pop().unwrap();
                let (mut s1, s2) = if last.queries.len() > s.queries.len() {
                    (last, s)
                } else {
                    (s, last)
                };
                // s1 is bigger
                // dbg!("Merge", s1, s2);
                let delta = s2.res - s1.res;
                for (r, qid) in s2.queries.iter() {
                    queries_res[*qid] += delta;
                    s1.queries.insert((*r, *qid));
                }
                next_states.push(s1);
            }
        }
        states = next_states;
    }
    queries_res
}

fn solve_slow(x: i64, s: &[i64], p: &[i64], starts_here: &[Vec<(usize, usize)>]) -> Vec<i64> {
    let n = s.len();
    let mut q = 0;
    for v in starts_here.iter() {
        q += v.len();
    }
    let mut queries_res = vec![0i64; q];
    let x = x as usize;
    for start in 0..n {
        for &(r, q_id) in starts_here[start].iter() {
            let mut dp = Array2D::new(i64::MIN / 2, x + 1, x + 1);
            dp[1][1] = 0;
            for i in start..r {
                let mut ndp = Array2D::new(i64::MIN / 2, x + 1, x + 1);
                for a in 1..=x {
                    for b in 1..=x {
                        let prev = dp[a][b];
                        for next in 1..=x {
                            let (na, nb) = if i % 2 == 0 { (next, b) } else { (a, next) };
                            let mut nscore = prev;
                            if (na as i64) * (nb as i64) >= s[i] {
                                nscore += p[i];
                            }
                            if nscore > ndp[na][nb] {
                                ndp[na][nb] = nscore;
                            }
                        }
                    }
                }
                dp = ndp;
            }
            let mut best = i64::MIN;
            for a in 1..=x {
                for b in 1..=x {
                    if dp[a][b] > best {
                        best = dp[a][b];
                    }
                }
            }
            queries_res[q_id] = best;
        }
    }
    queries_res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let x = input.i64();
    let q = input.usize();
    let s = input.vec::<i64>(n);
    let p = input.vec::<i64>(n);
    let mut starts_here = vec![vec![]; n];
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        starts_here[l].push((r, id));
    }
    let queries_res = solve_case(x, &s, &p, &starts_here);
    for query in queries_res.iter() {
        out.println(*query);
    }
}

fn stress() {
    for it in 9.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 20;
        const MAX_X: i64 = 20;
        let n = rnd.gen_range(1..MAX_N);
        let x = rnd.gen_range(1..MAX_X);
        let s = rnd.gen_vec(n, 1..MAX_X);
        let mut p = vec![];
        for _ in 0..n {
            if rnd.gen_bool() {
                p.push(1);
            } else {
                p.push(-1);
            }
        }
        let mut starts_here = vec![vec![]; n];
        let q = rnd.gen_range(1..MAX_N);
        for id in 0..q {
            let l = rnd.gen_range(0..n);
            let r = rnd.gen_range(l + 1..n + 1);
            starts_here[l].push((r, id));
        }
        let res_slow = solve_slow(x, &s, &p, &starts_here);
        let res_fast = solve_case(x, &s, &p, &starts_here);
        if res_slow != res_fast {
            dbg!(n, x, &s, &p, &starts_here);
            dbg!(res_slow);
            dbg!(res_fast);
            panic!("Mismatch");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
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
