//{"name":"A: Fourth Player","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n8\n1 2\n3 7\n6 8\n4 5\n16\n14 13 12 1\n15 5 6 7\n16 2 3 4\n8 9 10 11\n16\n15 13 11 6\n16 12 1 2\n5 9 7 8\n14 10 3 4\n8\n5 2\n8 4\n7 3\n6 1\n24\n7 6 14 22 18 12\n20 23 13 8 16 11\n24 21 4 9 1 19\n15 5 10 17 3 2\n","output":"Case #1: 2\nCase #2: 3\nCase #3: 0\nCase #4: 1\nCase #5: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"fourth_player_.*input[.]txt"},"output":{"type":"file","fileName":"fourth_player_output.txt","pattern":null},"languages":{"java":{"taskClass":"AFourthPlayer"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

#[derive(Clone, Copy)]
struct Elem {
    pos: usize,
    value: i32,
}

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        let n = input.usize() / 4;
        let mut a = vec![];
        for pos in 0..4 {
            for _j in 0..n {
                a.push(Elem {
                    pos,
                    value: input.read(),
                });
            }
        }
        a.sort_by_key(|e| e.value);
        a.reverse();
        let mut cnt = vec![0; 4];
        let mut score = vec![0; 2];
        for &e in a.iter() {
            let mut covered = false;
            if e.pos == 0 {
                if cnt[1] > 0 {
                    cnt[1] -= 1;
                    covered = true;
                } else if cnt[3] > 0 {
                    cnt[3] -= 1;
                    covered = true;
                }
            } else if e.pos == 1 {
                if cnt[2] > 0 {
                    cnt[2] -= 1;
                    covered = true;
                }
            } else if e.pos == 2 {
                if cnt[3] > 0 {
                    cnt[3] -= 1;
                    covered = true;
                }
            }
            cnt[e.pos] += 1;
            if !covered {
                score[e.pos % 2] += 1;
            }
            if score[0] + score[1] == n {
                break;
            }
        }
        out_line!(format!("Case #{}: {}", test_case, score[0]));
    }
    // run_parallel::<Job>(input, Some(1), &());
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
        output: TaskIoType::File("fourth_player_output.txt".to_string()),
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
