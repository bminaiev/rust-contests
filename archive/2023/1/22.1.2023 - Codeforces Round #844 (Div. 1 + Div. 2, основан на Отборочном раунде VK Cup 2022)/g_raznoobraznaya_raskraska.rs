//{"name":"G. Разнообразная раскраска","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, основан на Отборочном раунде VK Cup 2022)","url":"https://codeforces.com/contest/1782/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n7\n1 2 2 1 5 5\n8\n1 2 3 4 5 6 7\n","output":"0\n1\n2\n1\n0\n1\nwbwwwbb\n0\n1\n0\n1\n0\n1\n0\nwbwbwbwb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GRaznoobraznayaRaskraska"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Solution {
    colors: Vec<usize>,
    score: usize,
}

#[derive(Default, Clone)]
struct DP {
    c1: usize,
    c2: usize,
    score1: usize,
    score2: usize,
}

fn solve_dp(p: &[usize]) -> Solution {
    let n = p.len();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[p[i]].push(i);
    }
    const SHIFT: usize = 1;
    // dp[v][my_color][parent_color - 0/1/2][white - blue + SHIFT] -> Option<(color1, color2, shift1, shift2)>

    let max_score = SHIFT * 2 + 1;
    let mut dp = vec![vec![Array2D::new(None, 3, max_score); 2]; n];
    for v in (0..n).rev() {
        for my_color in 0..2 {
            let score_delta = if my_color == 0 { 1i32 } else { -1 };
            for parent_color in 0..3 {
                let child = &g[v];
                if child.is_empty() {
                    if parent_color != 2 && parent_color != my_color {
                        dp[v][my_color][parent_color][(SHIFT as i32 + score_delta) as usize] =
                            Some(DP::default());
                    }
                } else if child.len() == 1 {
                    let to1 = child[0];
                    for c1 in 0..2 {
                        for score1 in 0..max_score {
                            if dp[to1][c1][my_color][score1].is_some() {
                                if c1 != my_color
                                    || ((parent_color != 2) && parent_color != my_color)
                                {
                                    let new_score = score1 as i32 + score_delta;
                                    if new_score >= 0 && new_score < max_score as i32 {
                                        dp[v][my_color][parent_color][new_score as usize] =
                                            Some(DP {
                                                c1,
                                                c2: 0,
                                                score1,
                                                score2: 0,
                                            });
                                    }
                                }
                            }
                        }
                    }
                } else if child.len() == 2 {
                    let to1 = child[0];
                    let to2 = child[1];
                    for c1 in 0..2 {
                        for score1 in 0..max_score {
                            if dp[to1][c1][my_color][score1].is_some() {
                                for c2 in 0..2 {
                                    for score2 in 0..max_score {
                                        if dp[to2][c2][my_color][score2].is_some() {
                                            if c1 != my_color
                                                || ((parent_color != 2) && parent_color != my_color)
                                                || c2 != my_color
                                            {
                                                let new_score =
                                                    score1 as i32 + score2 as i32 + score_delta
                                                        - SHIFT as i32;
                                                if new_score >= 0 && new_score < max_score as i32 {
                                                    dp[v][my_color][parent_color]
                                                        [new_score as usize] = Some(DP {
                                                        c1,
                                                        c2,
                                                        score1,
                                                        score2,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if n >= 5 {
        if n % 2 == 0 {
            assert!(dp[0][0][2][SHIFT].is_some());
        } else {
            assert!(dp[0][0][2][SHIFT + 1].is_some());
        }
    }
    let mut colors = vec![0; n];
    let mut scores = vec![0; n];
    let mut parent_colors = vec![2; n];
    scores[0] = SHIFT + (n % 2);
    for v in 0..n {
        let dp = dp[v][colors[v]][parent_colors[v]][scores[v]]
            .as_ref()
            .unwrap();
        if g[v].len() >= 1 {
            let to = g[v][0];
            parent_colors[to] = colors[v];
            scores[to] = dp.score1;
            colors[to] = dp.c1;
        }
        if g[v].len() >= 2 {
            let to = g[v][1];
            parent_colors[to] = colors[v];
            scores[to] = dp.score2;
            colors[to] = dp.c2;
        }
    }
    Solution {
        colors,
        score: scores[0] - SHIFT,
    }
}

fn solve_slow(p: &[usize]) -> Solution {
    let n = p.len();
    assert!(n <= 5);
    let mut sol = Solution {
        colors: vec![0; n],
        score: n + 10,
    };
    for mask in 0i32..(1 << n) {
        let mut good = vec![false; n];
        for i in 1..n {
            let m1 = ((1 << i) & mask) != 0;
            let m2 = ((1 << p[i]) & mask) != 0;
            if m1 != m2 {
                good[i] = true;
                good[p[i]] = true;
            }
        }
        if good.iter().all(|g| *g) {
            let cnt1 = mask.count_ones() as i32;
            let cnt2 = (n as i32) - cnt1;
            let cur_res = (cnt1 - cnt2).abs();
            if cur_res < sol.score as i32 {
                sol.score = cur_res as usize;
                for i in 0..n {
                    if ((1 << i) & mask) != 0 {
                        sol.colors[i] = 0;
                    } else {
                        sol.colors[i] = 1;
                    }
                }
            }
        }
    }
    sol
}

fn stress() {
    let mut rnd = Random::new(787788);
    for it in 1.. {
        dbg!(it);
        let n = rnd.gen(5..16);
        let mut p = vec![0; n];
        let mut cnt_child = vec![0; n];
        for i in 1..n {
            loop {
                p[i] = rnd.gen(0..i);
                if cnt_child[p[i]] < 2 {
                    cnt_child[p[i]] += 1;
                    break;
                }
            }
        }
        if cnt_child.iter().any(|x| *x > 2) {
            assert!(false);
        }
        let mut res = i32::MAX;
        for mask in 0i32..(1 << n) {
            let mut good = vec![false; n];
            for i in 1..n {
                let m1 = ((1 << i) & mask) != 0;
                let m2 = ((1 << p[i]) & mask) != 0;
                if m1 != m2 {
                    good[i] = true;
                    good[p[i]] = true;
                }
            }
            if good.iter().all(|g| *g) {
                let cnt1 = mask.count_ones() as i32;
                let cnt2 = (n as i32) - cnt1;
                res.update_min((cnt1 - cnt2).abs());
            }
        }
        assert!(res == n as i32 % 2);
        solve_dp(&p);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut p = vec![0];
    for _i in 1..n {
        let prev = input.usize() - 1;
        p.push(prev);
        if p.len() <= 5 {
            let sol = solve_slow(&p);
            out_line!(sol.score)
        } else {
            out_line!(p.len() % 2);
        }
    }
    let sol = if p.len() <= 5 {
        solve_slow(&p)
    } else {
        solve_dp(&p)
    };
    for i in 0..n {
        if sol.colors[i] == 0 {
            out!('w');
        } else {
            out!('b');
        }
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
