//{"name":"icpc2015_f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"icpc2015_f"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::iters::shifts_iter::ShiftsIterator;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct State {
    r: usize,
    c: usize,
    done: usize,
}

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = gen_vec(n, |_| input.string());

    let shifts = ShiftsIterator::new(&SHIFTS_4, n, m);

    let mut near = Array2D::new(vec![], n, m);
    for r in 0..n {
        for c in 0..m {
            for shift in SHIFTS_4.iter() {
                let mut p = Point::new(r as i32, c as i32);
                loop {
                    p = p.apply_shift(shift);
                    if let Some(val) = p.index_vec2d(&s) {
                        if *val == s[r][c] {
                            continue;
                        }
                        near[r][c].push((p.x as usize, p.y as usize));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut need = input.string();
    need.push(b'*');

    let mut queue = VecDeque::new();
    let mut dist = vec![std::i32::MAX; n * m * (1 + need.len())];

    let get_id = |s: &State| -> usize { (s.r * m + s.c) * (need.len() + 1) + s.done };

    let start = State {
        r: 0,
        c: 0,
        done: 0,
    };
    queue.push_back(start);
    dist[get_id(&start)] = 0;

    while let Some(cur) = queue.pop_front() {
        if cur.done == need.len() {
            out_line!(dist[get_id(&cur)]);
            return;
        }
        if s[cur.r][cur.c] == need[cur.done] {
            let next = State {
                r: cur.r,
                c: cur.c,
                done: cur.done + 1,
            };
            let id = get_id(&next);
            if dist[id] == std::i32::MAX {
                dist[id] = dist[get_id(&cur)] + 1;
                queue.push_back(next);
            }
        }
        for &(nr, nc) in near[cur.r][cur.c].iter() {
            let next = State {
                r: nr,
                c: nc,
                done: cur.done,
            };
            let id = get_id(&next);
            if dist[id] == std::i32::MAX {
                dist[id] = dist[get_id(&cur)] + 1;
                queue.push_back(next);
            }
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
