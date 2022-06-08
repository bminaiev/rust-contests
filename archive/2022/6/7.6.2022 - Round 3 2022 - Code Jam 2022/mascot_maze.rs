//{"name":"Mascot Maze","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b44a4f","interactive":false,"timeLimit":20000,"tests":[{"input":"4\n3\n2 1 1\n3 3 2\n6\n3 1 4 1 2 3\n5 3 5 2 4 5\n20\n2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 1 1\n3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 20 2\n19\n2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 1 1\n3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 19 3\n","output":"Case #1: IMPOSSIBLE\nCase #2: TSHIRT\nCase #3: HCJKSHCJKSHCJKSHCJKS\nCase #4: CODEJAMROCKSTHEMOST\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MascotMaze"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::strings::utils::byte2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const M: usize = 13;

fn solve_case(left: &[usize], right: &[usize]) -> Option<Vec<usize>> {
    let n = left.len();
    for i in 0..n {
        if left[left[i]] == i {
            return None;
        }
        if left[right[i]] == i {
            return None;
        }
        if right[left[i]] == i {
            return None;
        }
        if right[right[i]] == i {
            return None;
        }
    }
    let mut res = vec![0; n];
    let mut rnd = Random::new(787788);
    for v in 0..n {
        res[v] = rnd.gen(0..M);
    }
    loop {
        let mut ok = true;
        for i in 0..n {
            let mut cur_ok = true;
            if res[i] == res[left[i]] {
                cur_ok = false;
            }
            if res[i] == res[right[i]] {
                cur_ok = false;
            }
            if res[i] == res[left[left[i]]] {
                cur_ok = false;
            }
            if res[i] == res[left[right[i]]] {
                cur_ok = false;
            }

            if res[i] == res[right[left[i]]] {
                cur_ok = false;
            }

            if res[i] == res[right[right[i]]] {
                cur_ok = false;
            }
            if !cur_ok {
                ok = false;
                res[i] = rnd.gen(0..M);
            }
        }
        if ok {
            break;
        }
    }
    Some(res)
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let left = input.vec::<usize>(n).sub_from_all(1);
    let right = input.vec::<usize>(n).sub_from_all(1);
    let res = solve_case(&left, &right);
    out!(format!("Case #{}: ", test_case));
    if let Some(res) = res {
        let str = "ACDEHIJKMORST".as_bytes();
        for i in res.into_iter() {
            out!(byte2str(str[i]));
        }
        out_line!();
    } else {
        out_line!("IMPOSSIBLE");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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

fn stress() {
    let n = 100_000;
    let mut rnd = Random::new(787788);
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for i in 0..n {
        loop {
            left[i] = rnd.gen(0..n);
            right[i] = rnd.gen(0..n);
            if left[i] != i && right[i] != i {
                break;
            }
        }
    }
    let res = solve_case(&left, &right);
    dbg!("found!");
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
