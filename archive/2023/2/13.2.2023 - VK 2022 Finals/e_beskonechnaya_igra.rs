//{"name":"E. Бесконечная игра","group":"Codeforces - VK 2022 Finals","url":"https://codeforces.com/gym/425375/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"??\n","output":"1\n2\n1\n"},{"input":"?aa?b\n","output":"1\n3\n0\n"},{"input":"??????\n","output":"22\n20\n22\n"},{"input":"a????a??????b??abbababbbb?a?aaa????bb\n","output":"97833\n28387\n135924\n"},{"input":"??????????????a????????????????b?????????????a???????????aa?????????bb?????????????a?????????a??????\n","output":"56481313\n275940576\n406286832\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBeskonechnayaIgra"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[derive(Clone, Default, Copy)]
struct Edge {
    new_state: usize,
    score_delta: i32,
}

impl Edge {
    pub fn new(new_state: usize, score_delta: i32) -> Self {
        Self {
            new_state,
            score_delta,
        }
    }
}

fn count_mask_by_state(s: &[usize; 4]) -> usize {
    let mut cnt = [0; 4];
    let mut pos = 0;
    for _ in 0..100 {
        cnt[pos] += 1;
        pos = s[pos];
    }
    let mut res = 0;
    for i in 0..4 {
        if cnt[i] > 10 {
            res |= 1 << i;
        }
    }
    res
}

fn solve_string(s: &[u8]) -> [Mod; 3] {
    let conv_state = |s: &[usize; 4]| -> usize { s[0] + s[1] * 4 + s[2] * 16 + s[3] * 64 };
    const MAX_STATES: usize = 256;

    let next_a = [
        Edge::new(2, 0),
        Edge::new(3, 0),
        Edge::new(0, 1),
        Edge::new(0, 1),
    ];
    let next_b = [
        Edge::new(1, 0),
        Edge::new(0, -1),
        Edge::new(3, 0),
        Edge::new(0, -1),
    ];
    let next = [next_a, next_b];
    let mut all_states = vec![[0; 4]; MAX_STATES];
    for g0 in 0..4 {
        for g01 in 0..4 {
            for g10 in 0..4 {
                for g11 in 0..4 {
                    let state = [g0, g01, g10, g11];
                    all_states[conv_state(&state)] = state;
                }
            }
        }
    }

    let mut res = [Mod::ZERO; 3];

    let shift = s.len() * 2 + 5;
    let max_score = shift * 2;
    for counts_mask in 1..16 {
        let mut go = Array2D::new(Edge::default(), MAX_STATES, 2);
        for state in all_states.iter() {
            let state_id = conv_state(state);
            for next_c in 0..2 {
                let mut next_state = state.clone();
                let mut sum_delta = 0;
                for i in 0..state.len() {
                    let cur = state[i];
                    let e = next[next_c][cur];
                    if ((1 << i) & counts_mask) != 0 {
                        sum_delta += e.score_delta;
                    }
                    next_state[i] = e.new_state;
                }
                go[state_id][next_c] = Edge {
                    new_state: conv_state(&next_state),
                    score_delta: sum_delta,
                }
            }
        }

        let start_state = [0, 1, 2, 3];
        // dp[cur_state][SHIFT + alive - bob]
        let mut dp = Array2D::new(Mod::ZERO, MAX_STATES, max_score);
        dp[conv_state(&start_state)][shift] = Mod::ONE;
        for &c in s.iter() {
            let c_id = if c == b'a' {
                0
            } else if c == b'b' {
                1
            } else {
                2
            };
            let mut ndp = Array2D::new(Mod::ZERO, MAX_STATES, max_score);
            for next_c in 0..2 {
                if next_c != c_id && c_id != 2 {
                    continue;
                }
                for state in 0..MAX_STATES {
                    for score in 0..max_score {
                        let cur = dp[state][score];
                        if cur == Mod::ZERO {
                            continue;
                        }

                        let e = go[state][next_c];
                        ndp[e.new_state][(score as i32 + e.score_delta) as usize] += cur;
                    }
                }
            }
            dp = ndp;
        }
        for (s_id, state) in all_states.iter().enumerate() {
            if count_mask_by_state(state) == counts_mask {
                for score in 0..max_score {
                    let res_pos = if score > shift {
                        0
                    } else if score < shift {
                        2
                    } else {
                        1
                    };
                    res[res_pos] += dp[s_id][score];
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let res = solve_string(&s);
    for &r in res.iter() {
        out_line!(r);
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
