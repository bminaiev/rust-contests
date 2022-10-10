//{"name":"D2: First Time - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n5 4\n2 1\n3 4\n1 4\n5 4\n3 4\n1 2\n2 1\n1 3\n3 1\n8 6\n1 4\n2 3\n4 3\n8 7\n6 5\n7 5\n","output":"Case #1: 13\nCase #2: 4\nCase #3: 6\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"first_time_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"first_time_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2FirstTimeChapter2"}}}

use std::collections::HashMap;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

#[derive(Clone, Copy, Default)]
struct Edge {
    cur_time: usize,
    to: usize,
    to_time: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    color: usize,
    pos: usize,
}

#[derive(Clone, Copy, Default)]
struct Go {
    time: usize,
    to: usize,
}

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        let n = input.usize();
        let m = input.usize();

        let mut g = vec![vec![]; n];
        for time in 1..=m {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            let e = Edge {
                cur_time: time,
                to,
                to_time: g[to].len(),
            };
            g[fr].push(e);
        }

        let mut all_pos = vec![];
        for i in 0..n {
            for j in 0..=g[i].len() {
                all_pos.push(Pos { color: i, pos: j });
            }
        }
        let mut pos_hm = HashMap::new();
        for i in 0..all_pos.len() {
            pos_hm.insert(all_pos[i], i);
        }

        const M: usize = 20;
        let mut go = Array2D::new(Go::default(), M, all_pos.len());
        for i in 0..all_pos.len() {
            let e = g[all_pos[i].color].get(all_pos[i].pos);
            if let Some(e) = e {
                let nxt = Pos {
                    color: e.to,
                    pos: e.to_time,
                };
                let id = *pos_hm.get(&nxt).unwrap();
                go[0][i] = Go {
                    to: id,
                    time: e.cur_time,
                };
            } else {
                go[0][i] = Go {
                    to: i,
                    time: std::usize::MAX,
                };
            }
        }
        for lvl in 0..M - 1 {
            for v in 0..all_pos.len() {
                go[lvl + 1][v] = go[lvl][go[lvl][v].to];
            }
        }
        let mut start = vec![0; n];
        for i in 0..n {
            start[i] = pos_hm[&Pos { color: i, pos: 0 }];
        }
        let get_color = |start_c: usize, time: usize| -> usize {
            let mut v = start[start_c];
            for lvl in (0..M).rev() {
                let g = go[lvl][v];
                if g.time <= time {
                    v = g.to;
                }
            }
            v
        };
        let mut first_ok = vec![0; n - 1];
        for i in 0..(n - 1) {
            first_ok[i] = binary_search_first_true(0..m + 2, |check_time| {
                let c1 = get_color(i, check_time);
                let c2 = get_color(i + 1, check_time);
                c1 == c2
            });
        }
        let mut all_first_ok = gen_vec(n - 1, |i| FirstOk {
            pos: i,
            time: first_ok[i],
        });
        all_first_ok.sort_by_key(|c| c.time);
        let mut ok_pos = vec![0; n - 1];
        for i in 0..n - 1 {
            ok_pos[all_first_ok[i].pos] = i;
        }

        let mut fw = Fenwick::<i32>::new(n - 1);
        for i in 0..(n - 1) {
            fw.add(i, 1);
        }

        let mut tot_res = 0;
        for k in 1..=n {
            for z in (k - 1..n - 1).step_by(k) {
                fw.add(ok_pos[z], -1);
            }
            let sum = fw.get_sum(n - 2);
            if sum != 0 {
                let last_one = binary_search_first_true(0..n - 2, |check| fw.get_sum(check) == sum);
                let res = all_first_ok[last_one].time;

                let res = if res > m { -1 } else { res as i64 };
                tot_res += res;
            }
            for z in (k - 1..n - 1).step_by(k) {
                fw.add(ok_pos[z], 1);
            }
        }
        out_line!(format!("Case #{}: {}", test_case, tot_res));
    }
    // run_parallel::<Job>(input, Some(1), &());
}

#[derive(Clone, Copy)]
struct FirstOk {
    pos: usize,
    time: usize,
}

#[derive(Clone, Default)]
struct Job {}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {}

    fn solve(&mut self, context: &Self::Context) {}

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: ", test_case));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("first_time_chapter__output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    tester::run_with_last_downloaded_file();
}
//END MAIN
