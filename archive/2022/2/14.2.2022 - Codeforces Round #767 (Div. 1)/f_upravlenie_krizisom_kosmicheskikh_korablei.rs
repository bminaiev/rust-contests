//{"name":"F. Управление кризисом космических кораблей","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/F","interactive":false,"timeLimit":8000,"tests":[{"input":"3\n0 1 2 4\n1 3 -1 6\n0 -1 1 -1\n14\n-2 10\n-1 10\n0 10\n1 10\n2 10\n3 10\n4 10\n5 10\n6 10\n-1 -2\n0 -2\n1 -2\n2 -2\n3 -2\n","output":"YES\nYES\nYES\nYES\nYES\nNO\nNO\nNO\nYES\nYES\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FUpravlenieKrizisomKosmicheskikhKorablei"}}}

use std::cmp::min;
use std::f64::consts::PI;

use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment::SegmentT;
use algo_lib::geometry::segment_intersection::segment_intersection;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search_float::float_binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn abs_angle_diff(a: OrdF64, b: OrdF64) -> OrdF64 {
    let delta = (a - b).abs();
    let two_pi: OrdF64 = OrdF64::PI * OrdF64::TWO;
    let not_big_delta = delta % two_pi;
    min(not_big_delta, two_pi - not_big_delta)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let segs = gen_vec(n, |_| {
        let from: Point = input.read();
        let to = input.read();
        Segment::new(from, to)
    });
    let q = input.usize();
    const FINISH: Point = Point::ZERO;
    const OK_ANGLE_DIFF: OrdF64 = OrdF64(PI / 4.0);
    for _ in 0..q {
        let start: Point = input.read();

        #[derive(Debug)]
        struct Result {
            p: Point,
            err: bool,
        }

        let go = |angle: OrdF64| -> Result {
            #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
            struct Opt {
                dist2: OrdF64,
                is_err: bool,
                end: bool,
                next: Point,
            }

            let mut err = false;
            let mut cur_pos = start;

            let dir = Point::new(OrdF64(f64::cos(angle.0)), OrdF64(f64::sin(angle.0)));
            let dir_rev = dir.rotate_ccw();
            let final_line = Line::new(&FINISH, &(FINISH + dir_rev));

            loop {
                let mut options = vec![];

                let my_line = Line::new(&cur_pos, &(cur_pos + dir));
                let on_line = final_line.intersect(&my_line).unwrap();

                let my_seg = Segment::new(cur_pos, on_line);

                if on_line.dist2(&FINISH).sqrt() <= OrdF64::EPS / OrdF64(100.0) {
                    options.push(Opt {
                        dist2: cur_pos.dist2(&FINISH),
                        is_err: false,
                        next: FINISH,
                        end: true,
                    });
                } else {
                    options.push(Opt {
                        dist2: cur_pos.dist2(&on_line),
                        is_err: true,
                        next: on_line,
                        end: true,
                    })
                }
                for seg in segs.iter() {
                    if let Some(intersection) = segment_intersection(seg, &my_seg) {
                        let dist2 = cur_pos.dist2(&intersection);
                        if dist2 <= OrdF64::EPS {
                            continue;
                        }
                        let fr_to_angle = seg.from.angle_to(&seg.to);
                        let to_fr_angle = seg.to.angle_to(&seg.from);
                        let a1 = abs_angle_diff(fr_to_angle, angle);
                        let a2 = abs_angle_diff(to_fr_angle, angle);
                        if a1 < OK_ANGLE_DIFF - OrdF64::EPS {
                            options.push(Opt {
                                dist2,
                                is_err: false,
                                next: seg.to,
                                end: false,
                            })
                        } else if a2 < OK_ANGLE_DIFF - OrdF64::EPS {
                            options.push(Opt {
                                dist2,
                                is_err: false,
                                next: seg.from,
                                end: false,
                            })
                        } else {
                            options.push(Opt {
                                dist2,
                                is_err: true,
                                next: if a1 < a2 { seg.to } else { seg.from },
                                end: false,
                            })
                        }
                    }
                }
                let next = options.iter().min().unwrap();
                err |= next.is_err;
                cur_pos = next.next;
                if next.end {
                    return Result { err, p: cur_pos };
                }
            }
        };

        let forw_angle = start.angle_to(&FINISH);
        let mut saw_good = false;
        float_binary_search_first_true(
            forw_angle - OK_ANGLE_DIFF,
            forw_angle + OK_ANGLE_DIFF,
            50,
            |angle: OrdF64| -> bool {
                let expected = go(angle);
                // dbg!(angle, expected);
                if expected.p == FINISH && !expected.err {
                    saw_good = true;
                }
                Point::vect_mul(&start, &expected.p, &FINISH) <= OrdF64::ZERO
            },
        );
        // let res = go(best_angle);
        if saw_good {
            out_line!("YES");
        } else {
            out_line!("NO");
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
