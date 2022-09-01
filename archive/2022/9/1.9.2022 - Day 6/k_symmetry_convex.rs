//{"name":"K. Symmetry: Convex","group":"Yandex - Day 6","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39551/problems/K/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n0 0\n1 0\n1 1\n0 1\n3\n0 0\n3 0\n1 1\n4\n-1000000000 -1000000000\n1000000000 -1000000000\n1000000000 1000000000\n-1000000000 1000000000\n","output":"1\n1 1 -1\n4\n1 -1 0\n0 2 -1\n2 0 -1\n1 1 -1\n0\n1\n1 1 0\n4\n1 -1 0\n0 1 0\n1 0 0\n1 1 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KSymmetryConvex"}}}

use std::cmp::min;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn add_mid_points(pts: &[Point]) -> Vec<Point> {
    let mut res = vec![];
    // let x0 = pts[0].x * 2;
    // let y0 = pts[0].y * 2;
    for i in 0..pts.len() {
        let cur = pts[i];
        let next = pts[(i + 1) % pts.len()];
        res.push(Point::new(cur.x * 2, cur.y * 2));
        let ff = cur + next;
        res.push(Point::new(ff.x, ff.y));
    }
    res
}

#[derive(Clone, Copy)]
struct SymLine {
    a: i64,
    b: i64,
    c: i64,
}

impl SymLine {
    pub fn new(a: i64, b: i64, c: i64) -> Self {
        let g = gcd(a.abs(), b.abs());
        let g = gcd(g, c.abs());
        Self {
            a: a / g,
            b: b / g,
            c: c / g,
        }
    }
}

#[derive(Clone, Copy)]
struct ToCheck {
    i: usize,
    checked: usize,
}

fn solve_case(a: &[Point]) -> Vec<Vec<SymLine>> {
    let a = add_mid_points(&a);
    let mut cur_pts = vec![];
    let mut res = vec![];
    let mut to_check = vec![];
    let mut to_check_it = 0;
    for n in (5..=a.len()).step_by(2) {
        while cur_pts.len() != n {
            cur_pts.push(a[cur_pts.len()]);
        }

        let sum = cur_pts[0] + *cur_pts.last_exn();
        cur_pts.push(Point::new(sum.x / 2, sum.y / 2));

        // dbg!(cur_pts);

        let mut sym = vec![];
        assert!(cur_pts.len() % 2 == 0);
        let half = cur_pts.len() / 2;

        while to_check_it < cur_pts.len() / 2 {
            to_check.push(ToCheck {
                i: to_check_it,
                checked: 0,
            });
            to_check_it += 1;
        }

        let mut to_delete = vec![];

        for (idx, to_check) in to_check.iter_mut().enumerate() {
            let i = to_check.i;
            let start = cur_pts[i];
            let next = cur_pts[(i + 1) % cur_pts.len()];
            let prev = cur_pts[(i + cur_pts.len() - 1) % cur_pts.len()];
            let mut in_dir = start + (next - start) + (prev - start);
            if in_dir == start {
                assert_ne!(start, next);
                let delta = next - start;
                in_dir = start + delta.rotate_ccw();
            }
            assert_ne!(start, in_dir);
            let finish = cur_pts[(i + half) % cur_pts.len()];

            let mut ok = true;
            let max_to_check = min(i + 1, cur_pts.len() - 1 - i);

            let ok_shift = |shift: usize| -> bool {
                let right = cur_pts[(i + shift) % cur_pts.len()];
                let left = cur_pts[(i + cur_pts.len() - shift) % cur_pts.len()];
                if Point::scal_mul(&start, &in_dir, &left)
                    != Point::scal_mul(&start, &in_dir, &right)
                {
                    return false;
                }
                if Point::vect_mul(&start, &in_dir, &left)
                    != Point::vect_mul(&start, &in_dir, &right) * -1
                {
                    return false;
                }
                return true;
            };

            while to_check.checked < max_to_check {
                let shift = to_check.checked;
                if ok_shift(shift) {
                    to_check.checked += 1;
                } else {
                    ok = false;
                    break;
                }
            }
            if !ok {
                to_delete.push(idx);
                continue;
            }
            if Point::vect_mul(&start, &in_dir, &finish) != 0 {
                continue;
            }
            for shift in to_check.checked..half {
                if !ok_shift(shift) {
                    ok = false;
                    break;
                }
            }

            if ok {
                let p1 = start;
                let p2 = in_dir;
                // dbg!(p1, p2);
                let line_a = p2.y - p1.y;
                let line_b = p1.x - p2.x;
                let line_c = -(p1.x * line_a + p1.y * line_b);
                assert!(p1.x * line_a + p1.y * line_b + line_c == 0);
                assert!(p2.x * line_a + p2.y * line_b + line_c == 0);
                let line = SymLine::new(line_a * 2, line_b * 2, line_c);
                assert!(line.c.abs() <= 2 * (1e18 as i64));
                sym.push(line);
            }
        }

        for idx in to_delete.iter().rev() {
            to_check.swap_remove(*idx);
        }

        res.push(sym);
        cur_pts.pop();
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
    let res = solve_case(&a);
    for one_case in res.iter() {
        out_line!(one_case.len());
        for line in one_case.iter() {
            out_line!(line.a, line.b, line.c);
        }
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

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
