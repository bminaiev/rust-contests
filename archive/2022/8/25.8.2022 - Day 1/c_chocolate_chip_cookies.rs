//{"name":"C. Chocolate Chip Cookies","group":"Yandex - Day 1","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39546/problems/C/","interactive":false,"timeLimit":8000,"tests":[{"input":"5 8\n1 1\n1 2\n1 3\n2 1\n3 1\n3 4\n4 1\n4 2\n","output":"0.375\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChocolateChipCookies"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

#[derive(Clone, Copy)]
struct Event {
    dir: Point,
    delta: i32,
}

fn cut_area(start: Point, dir: Point, side: i32) -> f64 {
    let square = PolygonT::new_rect(PointT::ZERO, PointT::new(side, side));
    let my_part = square.cut(start, start + dir).area().0;
    let side = side as f64;
    my_part / side / side
}

fn gen_points_to_check(start: Point, side: i32) -> Vec<Point> {
    let mut res = vec![];
    let x2 = if start.x * 2 < side {
        start.x * 2
    } else {
        start.x * 2 - side
    };
    let y2 = if start.y * 2 < side {
        start.y * 2
    } else {
        start.y * 2 - side
    };
    if start.y * 2 < side {
        res.push(Point::new(x2, 0));
    } else {
        res.push(Point::new(x2, side));
    }
    if start.x * 2 < side {
        res.push(Point::new(0, y2));
    } else {
        res.push(Point::new(side, y2));
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let side = input.i32();
    let n = input.usize();
    let a = gen_vec(n, |_| Point::new(input.read(), input.read()));
    let mut res = 0.0;

    let mut update_res = |start: Point, dir: Point, cnt_pts: usize| {
        let pts_part = (cnt_pts as f64) / (n as f64);
        let sq = cut_area(start, dir, side);
        let cur_res = pts_part - sq;
        if cur_res > res {
            res = cur_res;
        }
    };

    for i in 0..n {
        let mut events = vec![];
        let mut cur_ok = 1;
        for j in 0..n {
            if i == j {
                continue;
            }
            let dir = a[j] - a[i];
            if dir.side() == 0 {
                cur_ok += 1;
                events.push(Event { dir, delta: -1 });
                events.push(Event {
                    dir: Point::ZERO - dir,
                    delta: 1,
                });
            } else {
                events.push(Event {
                    dir: Point::ZERO - dir,
                    delta: 1,
                });
                events.push(Event { dir, delta: -1 });
            }
        }
        let cmp = |e1: &Event, e2: &Event| {
            let side1 = e1.dir.side();
            let side2 = e2.dir.side();
            return side1
                .cmp(&side2)
                .then(Point::vect_mul2(&e1.dir, &e2.dir).cmp(&0).reverse())
                .then(e1.delta.cmp(&e2.delta).reverse());
        };
        events.sort_by(cmp);
        for e in events.iter() {
            cur_ok += e.delta;
            if e.delta > 0 {
                update_res(a[i], e.dir, cur_ok as usize);
            }
        }
        let to_check = gen_points_to_check(a[i], side);
        for another_pt in to_check.into_iter() {
            let mut cnt_ok = 0;
            for j in 0..n {
                if Point::vect_mul(&a[i], &another_pt, &a[j]) >= 0 {
                    cnt_ok += 1;
                }
            }
            update_res(a[i], another_pt - a[i], cnt_ok);
        }
    }
    out_line!(res);
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
