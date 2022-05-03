//{"name":"F. 4 positions","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 5 3 7\n2 3 3 6\n5 1 6 3\n4 2 7 3\n","output":"2\n6 3\n3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F4Positions"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::range_intersect::range_intersect;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_cnt = input.usize();
    let mut rnd = Random::new(78778811);

    let mut rx = vec![];
    let mut ry = vec![];

    for _ in 0..n {
        let x1 = input.i32();
        let y1 = input.i32();
        let x2 = input.i32();
        let y2 = input.i32();
        rx.push(x1..x2 + 1);
        ry.push(y1..y2 + 1);
    }

    let mut ids = gen_vec(n, id);

    const MAX: i32 = (1e9 + 1.0) as i32;

    loop {
        for i in 1..ids.len() {
            ids.swap(i, rnd.gen_in_range(0..i + 1));
        }
        let mut cur_x = vec![1..MAX; max_cnt];
        let mut cur_y = vec![1..MAX; max_cnt];
        let mut ok = true;
        let mut failed = n;
        let mut failed_iter = 0;
        for &v in ids.iter() {
            failed_iter += 1;
            let mut found = false;

            let mut best = (std::usize::MAX, max_cnt);

            for id in 0..max_cnt {
                let nx = range_intersect(cur_x[id].clone(), rx[v].clone());
                let ny = range_intersect(cur_y[id].clone(), ry[v].clone());
                if !nx.is_empty() && !ny.is_empty() {
                    // cur_x[id] = nx;
                    // cur_y[id] = ny;
                    found = true;
                    // break;
                    let prev_len = cur_x[id].len() + cur_y[id].len();
                    let next_len = nx.len() + ny.len();
                    let score = prev_len - next_len;
                    best.update_min((score, id));
                }
            }
            if !found {
                failed = v;
                ok = false;
                break;
            } else {
                let id = best.1;
                let nx = range_intersect(cur_x[id].clone(), rx[v].clone());
                let ny = range_intersect(cur_y[id].clone(), ry[v].clone());
                cur_x[id] = nx;
                cur_y[id] = ny;
            }
        }
        if ok {
            out_line!(max_cnt);
            for i in 0..max_cnt {
                out_line!(cur_x[i].start, cur_y[i].start);
            }
            return;
        }
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
}
//END MAIN
