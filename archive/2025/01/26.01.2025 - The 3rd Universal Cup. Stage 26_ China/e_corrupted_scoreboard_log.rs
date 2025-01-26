//{"name":"E. Corrupted Scoreboard Log","group":"Universal Cup - The 3rd Universal Cup. Stage 26: China","url":"https://contest.ucup.ac/contest/1894/problem/9979","interactive":false,"timeLimit":1000,"tests":[{"input":"4 12\n99351583tries261try312tries231try4tries431try2412tries551try991try1791try\n912121482tries572tries392tries821try4tries431try521try2492tries1842tries2183tries\n912181082tries141try542tries922tries6tries302tries6tries502tries2441try1956tries1714tries\n913221241try261try542tries1331try2002tries621try2tries811try2401try2825tries\n","output":"9 935 158 3 tries 26 1 try 31 2 tries 23 1 try 4 tries 43 1 try 241 2 tries 55 1 try 99 1 try 179 1 try\n9 1212 148 2 tries 57 2 tries 39 2 tries 82 1 try 4 tries 43 1 try 52 1 try 249 2 tries 184 2 tries 218 3 tries\n9 1218 108 2 tries 14 1 try 54 2 tries 92 2 tries 6 tries 30 2 tries 6 tries 50 2 tries 244 1 try 195 6 tries 171 4 tries\n9 1322 124 1 try 26 1 try 54 2 tries 133 1 try 200 2 tries 62 1 try 2 tries 81 1 try 240 1 try 282 5 tries\n"},{"input":"5 2\n0022tries22tries\n12222tries22tries\n24422tries22tries\n284222tries222tries\n2844222tries222tries\n","output":"0 0 22 tries 22 tries\n1 22 2 2 tries 22 tries\n2 44 2 2 tries 2 2 tries\n2 84 22 2 tries 22 2 tries\n2 844 2 22 tries 2 22 tries\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECorruptedScoreboardLog"}}}

use std::collections::HashSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct State {
    expected_problems: i64,
    expected_time: i64,
    sum_problems: i64,
    sum_time: i64,
    parse_pos: usize,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    let max_problems = input.i64();
    for _ in 0..tc {
        let s = input.string();

        let parse_i64 = |from: usize, len: usize| -> Option<i64> {
            if from + len > s.len() {
                return None;
            }
            let mut res = 0i64;
            for i in from..from + len {
                if !s[i].is_ascii_digit() {
                    return None;
                }
                res = res * 10 + (s[i] as i64 - '0' as i64);
            }
            if s[from] == b'0' && len > 1 {
                return None;
            }
            Some(res)
        };

        let parse_tries = |from: usize, tries: &[u8]| -> bool {
            if from + tries.len() > s.len() {
                return false;
            }
            for i in 0..tries.len() {
                if s[from + i] != tries[i] {
                    return false;
                }
            }
            true
        };

        let tries = |attemts: i64| -> &[u8] {
            match attemts {
                1 => b"try",
                _ => b"tries",
            }
        };

        let mut seen_states = HashSet::<State>::new();
        let mut r = RecursiveFunction2::new(
            |f, state: State, parts: Vec<(usize, usize)>| -> Option<Vec<(usize, usize)>> {
                if seen_states.contains(&state) {
                    return None;
                }
                seen_states.insert(state);
                if state.sum_problems > max_problems {
                    return None;
                }
                if state.parse_pos == s.len() {
                    if state.sum_problems == state.expected_problems
                        && state.sum_time == state.expected_time
                    {
                        return Some(parts);
                    }
                    return None;
                }
                if state.sum_time > state.expected_time {
                    return None;
                }
                if state.sum_problems > state.expected_problems {
                    return None;
                }
                for time_sz in 1..=3 {
                    if let Some(time) = parse_i64(state.parse_pos, time_sz) {
                        for attempt_sz in 1..=3 {
                            if let Some(attempt) = parse_i64(state.parse_pos + time_sz, attempt_sz)
                            {
                                let expect_tries = tries(attempt);
                                if parse_tries(state.parse_pos + time_sz + attempt_sz, expect_tries)
                                {
                                    let mut ok = true;
                                    if time >= 300 {
                                        ok = false;
                                    }
                                    if attempt > 100 || attempt == 0 {
                                        ok = false;
                                    }
                                    if ok {
                                        let new_state = State {
                                            expected_problems: state.expected_problems,
                                            expected_time: state.expected_time,
                                            sum_problems: state.sum_problems + 1,
                                            sum_time: state.sum_time + time + (attempt - 1) * 20,
                                            parse_pos: state.parse_pos
                                                + time_sz
                                                + attempt_sz
                                                + expect_tries.len(),
                                        };
                                        let mut new_parts = parts.clone();
                                        new_parts.push((state.parse_pos, time_sz));
                                        new_parts.push((state.parse_pos + time_sz, attempt_sz));
                                        new_parts.push((
                                            state.parse_pos + time_sz + attempt_sz,
                                            expect_tries.len(),
                                        ));
                                        if let Some(p) = f.call(new_state, new_parts) {
                                            return Some(p);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                for attempt_sz in 1..=3 {
                    if let Some(attempt) = parse_i64(state.parse_pos, attempt_sz) {
                        let expect_tries = tries(attempt);
                        if parse_tries(state.parse_pos + attempt_sz, expect_tries) {
                            let mut ok = true;
                            if attempt > 100 || attempt == 0 {
                                ok = false;
                            }
                            if ok {
                                let new_state = State {
                                    expected_problems: state.expected_problems,
                                    expected_time: state.expected_time,
                                    sum_problems: state.sum_problems,
                                    sum_time: state.sum_time,
                                    parse_pos: state.parse_pos + attempt_sz + expect_tries.len(),
                                };
                                let mut new_parts = parts.clone();
                                new_parts.push((state.parse_pos, attempt_sz));
                                new_parts.push((state.parse_pos + attempt_sz, expect_tries.len()));
                                if let Some(p) = f.call(new_state, new_parts) {
                                    return Some(p);
                                }
                            }
                        }
                    }
                }
                None
            },
        );
        let mut res = None;
        for cnt_problems_sz in 1..=2 {
            for time_sz in 1..=5 {
                if res.is_some() {
                    break;
                }
                if let Some(cnt_probems) = parse_i64(0, cnt_problems_sz) {
                    if let Some(time) = parse_i64(cnt_problems_sz, time_sz) {
                        let state = State {
                            expected_problems: cnt_probems,
                            expected_time: time,
                            sum_problems: 0,
                            sum_time: 0,
                            parse_pos: cnt_problems_sz + time_sz,
                        };
                        let mut parts = Vec::new();
                        parts.push((0, cnt_problems_sz));
                        parts.push((cnt_problems_sz, time_sz));
                        if let Some(p) = r.call(state, parts) {
                            res = Some(p);
                            break;
                        }
                    }
                }
            }
        }
        let res = res.unwrap();
        for i in 0..res.len() {
            if i > 0 {
                out.print(" ");
            }
            let (from, len) = res[i];
            for j in from..from + len {
                out.print(s[j] as char);
            }
        }
        out.println("");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e_corrupted_scoreboard_log";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
