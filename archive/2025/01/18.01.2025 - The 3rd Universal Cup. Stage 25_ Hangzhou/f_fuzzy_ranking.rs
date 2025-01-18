//{"name":"F. Fuzzy Ranking","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9731","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 2 2\n1 2 3 4 5\n5 4 3 2 1\n1 0 2\n1 2 1\n5 3 3\n1 2 3 4 5\n1 3 2 4 5\n1 2 3 5 4\n0 0 2\n0 2 3\n1 0 3\n","output":"3\n10\n1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFuzzyRanking"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::fx_hash_map::FxHashSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

#[derive(Clone, Copy, Debug)]
struct Segment {
    l: usize,
    r: usize,
}

impl Segment {
    fn new(l: usize, r: usize) -> Self {
        Self { l, r }
    }
}

struct Solver {
    segs: Vec<Vec<Segment>>,
    dp: Vec<Vec<i64>>,
}

impl Solver {
    fn new(perms: &[Vec<usize>]) -> Self {
        let k = perms.len();
        let n = perms[0].len();
        let mut g = vec![vec![]; n];
        let mut seen = FxHashSet::default();
        for i in 0..k {
            for j in 0..n - 1 {
                let from = perms[i][j + 1];
                let to = perms[i][j];
                if seen.contains(&(from, to)) {
                    continue;
                }
                seen.insert((from, to));
                g[from].push(to);
            }
        }
        let mut segs: Vec<Vec<Segment>> = vec![vec![]; k];
        let mut dp = vec![vec![0i64; n + 1]; k];
        for i in 0..k {
            let mut seen = vec![false; n];
            let mut perm_inv = vec![0; n];
            for pos in 0..n {
                perm_inv[perms[i][pos]] = pos;
            }
            for pos in 0..n {
                let v = perms[i][pos];
                if seen[v] {
                    continue;
                }
                let mut max_pos = pos;
                let mut q = VecDeque::new();
                q.push_back(v);
                while let Some(v) = q.pop_front() {
                    seen[v] = true;
                    for &u in &g[v] {
                        if !seen[u] {
                            seen[u] = true;
                            q.push_back(u);
                            max_pos = max_pos.max(perm_inv[u]);
                        }
                    }
                }
                segs[i].push(Segment::new(pos, max_pos + 1));
            }
            {
                let mut sum_len = 0;
                for s in segs[i].iter() {
                    // dbg!(s);
                    sum_len += s.r - s.l;
                    for pos in s.l..s.r {
                        dp[i][pos + 1] = dp[i][pos];
                        dp[i][pos + 1] += (pos - s.l) as i64;
                    }
                }
                assert_eq!(sum_len, n);
            }
        }
        Self { segs, dp }
    }

    fn calc(&self, id: usize, mut l: usize, r: usize) -> i64 {
        let segs = &self.segs[id];
        let first_seg = binary_search_first_true(0..segs.len(), |i| segs[i].r > l);
        let mut v = 0;
        let first_seg_end = segs[first_seg].r.min(r);
        v += calc(l, first_seg_end);
        l = first_seg_end;
        if l != r {
            v += self.dp[id][r] - self.dp[id][l];
        }
        v
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let q = input.usize();
        let mut perms = vec![];
        for _ in 0..k {
            let p = input.vec::<usize>(n).sub_from_all(1);
            perms.push(p);
        }
        let mut solver = Solver::new(&perms);

        let mut prev_v = 0;
        for _ in 0..q {
            let id = (input.i64() + prev_v) % k as i64;
            let l = (input.i64() + prev_v) % n as i64;
            let r = (input.i64() + prev_v) % n as i64;
            let id = id as usize;
            let mut l = l as usize;
            let r = r as usize + 1;
            assert!(l < r);
            let v = solver.calc(id, l, r);
            out.println(v);
            prev_v = v;
        }
    }
}

fn calc(l: usize, r: usize) -> i64 {
    let r = r - 1;
    assert!(l <= r);
    let n = (r - l) as i64;
    n * (n + 1) / 2
}

fn stress() {
    const MX: usize = 10;
    for it in 418.. {
        dbg!(it);

        let mut rnd = Random::new(it);
        let k = rnd.gen(1..MX);
        let n = rnd.gen(1..MX);
        let mut perms = vec![];
        let mut g = Array2D::new(false, n, n);
        for _ in 0..k {
            let p = rnd.gen_permutation(n);
            dbg!(p);
            perms.push(p);
        }
        for i in 0..k {
            for j in 0..n - 1 {
                g[perms[i][j]][perms[i][j + 1]] = true;
            }
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if g[j][i] && g[i][k] {
                        g[j][k] = true;
                    }
                }
            }
        }
        let solver = Solver::new(&perms);
        for _ in 0..100 {
            let id = rnd.gen(0..k);
            let l = rnd.gen(0..n);
            let r = rnd.gen(l + 1..n + 1);
            let mut real_ans = 0;
            for p1 in l..r {
                for p2 in p1 + 1..r {
                    if g[perms[id][p1]][perms[id][p2]] && g[perms[id][p2]][perms[id][p1]] {
                        real_ans += 1;
                    }
                }
            }
            let my_ans = solver.calc(id, l, r);
            dbg!(id, l, r);
            dbg!(real_ans, my_ans);
            assert_eq!(real_ans, my_ans);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_fuzzy_ranking";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
