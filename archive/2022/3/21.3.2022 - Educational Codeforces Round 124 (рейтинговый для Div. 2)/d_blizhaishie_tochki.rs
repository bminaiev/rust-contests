//{"name":"D. Ближайшие точки","group":"Codeforces - Educational Codeforces Round 124 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1651/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n2 2\n1 2\n2 1\n3 2\n2 3\n5 5\n","output":"1 1\n1 1\n2 0\n3 1\n2 4\n5 4\n"},{"input":"8\n4 4\n2 4\n2 2\n2 3\n1 4\n4 2\n1 3\n3 3\n","output":"4 3\n2 5\n2 1\n2 5\n1 5\n4 1\n1 2\n3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBlizhaishieTochki"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve_a(a: &[Point]) {
    let n = a.len();
    let mut map = FxHashMap::<Point, usize>::default();
    for (id, p) in a.iter().enumerate() {
        map.insert(p.clone(), id);
    }
    let shifts = SHIFTS_4;
    let mut ans = gen_vec(n, |id| Point {
        x: a[id].x + n as i32 + 1,
        y: a[id].y,
    });
    let mut edges = vec![];
    for i in 0..n {
        for shift in shifts.iter() {
            let next_p = a[i].apply_shift(shift);
            if let Some(&next_id) = map.get(&next_p) {
                edges.push((i, next_id));
            } else {
                ans[i] = next_p;
            }
        }
    }
    loop {
        let mut changed = false;
        for &(fr, to) in edges.iter() {
            if a[fr].dist_manh(&ans[fr]) > a[fr].dist_manh(&ans[to]) {
                ans[fr] = ans[to].clone();
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for p in ans.iter() {
        out_line!(p.x, p.y);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a: Vec<Point> = input.vec(n);
    solve_a(&a);
}

fn stress() {
    let mut a = vec![];
    for x in 0..200 {
        for y in 0..1000 {
            a.push(Point { x, y });
        }
    }
    solve_a(&a);
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
    // tester::run_stress(stress);
}
//END MAIN
