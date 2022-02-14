//{"name":"D. Большая кисть","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4 4\n5 5 3 3\n1 1 5 3\n2 2 5 4\n2 2 4 4\n","output":"6\n1 3 3\n3 3 4\n2 2 5\n1 1 5\n2 1 1\n3 1 2\n"},{"input":"3 4\n1 1 1 1\n2 2 3 1\n2 2 1 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBolshayaKist"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_8;
use algo_lib::iters::shifts_iter::ShiftsIterator;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Res {
    x: usize,
    y: usize,
    color: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = input.read_matrix::<i32>(n, m);
    let mut queue = vec![];
    let mut seen = Array2D::new(false, n, m);
    let mut check = |x: usize, y: usize, a: &Array2D<i32>, queue: &mut Vec<Res>| {
        if x + 1 < n && y + 1 < m {
            if seen[x][y] {
                return;
            }
            let mut all = vec![a[x][y], a[x][y + 1], a[x + 1][y], a[x + 1][y + 1]];
            all.sort();
            if all[3] != 0 {
                let mut first = 0;
                while all[first] == 0 {
                    first += 1;
                }
                if all[first] == all[3] {
                    seen[x][y] = true;
                    queue.push(Res {
                        x,
                        y,
                        color: all[3],
                    })
                }
            }
        }
    };
    for x in 0..n - 1 {
        for y in 0..m - 1 {
            check(x, y, &a, &mut queue);
        }
    }
    let mut res = vec![];

    let shifts_iterator = ShiftsIterator::new(&SHIFTS_8, n, m);

    while let Some(r) = queue.pop() {
        let x = r.x;
        let y = r.y;
        a[x][y] = 0;
        a[x + 1][y] = 0;
        a[x][y + 1] = 0;
        a[x + 1][y + 1] = 0;
        res.push(r);
        for (nx, ny) in shifts_iterator.iter(x, y) {
            check(nx, ny, &a, &mut queue);
        }
    }
    if a.iter().any(|&x| x != 0) {
        out_line!(-1);
        return;
    }
    out_line!(res.len());
    for r in res.iter().rev() {
        out_line!(r.x + 1, r.y + 1, r.color);
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
