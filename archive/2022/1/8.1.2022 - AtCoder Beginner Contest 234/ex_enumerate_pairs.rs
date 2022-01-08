//{"name":"Ex - Enumerate Pairs","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_h","interactive":false,"timeLimit":4000,"tests":[{"input":"6 5\n2 0\n2 2\n3 4\n0 0\n5 5\n8 3\n","output":"9\n1 2\n1 3\n1 4\n2 3\n2 4\n2 5\n3 4\n3 5\n5 6\n"},{"input":"2 1414213562\n0 0\n1000000000 1000000000\n","output":"0\n"},{"input":"10 150\n300 300\n300 400\n300 500\n400 300\n400 400\n400 400\n400 500\n500 300\n500 400\n500 500\n","output":"29\n1 2\n1 4\n1 5\n1 6\n2 3\n2 4\n2 5\n2 6\n2 7\n3 5\n3 6\n3 7\n4 5\n4 6\n4 8\n4 9\n5 6\n5 7\n5 8\n5 9\n5 10\n6 7\n6 8\n6 9\n6 10\n7 9\n7 10\n8 9\n9 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExEnumeratePairs"}}}

use algo_lib::geometry::point::{PointT, PointWithIdT};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::iters::shifts::SHIFTS_9;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};
use std::collections::HashMap;

type Point = PointT<i64>;
type PointWithId = PointWithIdT<i64>;

fn solve(input: &mut Input) {
    let n = input.usize();
    let k = input.i64();
    let k2 = k * k;
    let mut by_cell: HashMap<Point, Vec<PointWithId>> = HashMap::new();
    for id in 0..n {
        let x = input.i64();
        let y = input.i64();
        let c = Point {
            x: x / (k + 1),
            y: y / (k + 1),
        };
        by_cell
            .entry(c)
            .or_default()
            .push(PointWithId::new(Point::new(x, y), id + 1));
    }

    let mut res = vec![];

    for (&cell, points) in by_cell.iter() {
        for shift in SHIFTS_9.iter() {
            let cell2 = cell.apply_shift(shift);
            if cell2 >= cell {
                if let Some(points2) = by_cell.get(&cell2) {
                    for p1 in points.iter() {
                        for p2 in points2.iter() {
                            if p1.p.dist2(&p2.p) <= k2 {
                                if p1.id() >= p2.id() && cell == cell2 {
                                    continue;
                                }
                                let id1 = min(p1.id(), p2.id());
                                let id2 = max(p1.id(), p2.id());
                                res.push((id1, id2));
                            }
                        }
                    }
                }
            }
        }
    }
    res.sort();
    out_line!(res.len());
    for (id1, id2) in res.iter() {
        out_line!(*id1, *id2);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
