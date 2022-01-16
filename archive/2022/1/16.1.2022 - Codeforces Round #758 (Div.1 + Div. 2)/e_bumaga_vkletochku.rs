//{"name":"E. Бумага в клеточку","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n2 3 1\n4 1 2\n2 1 3\n3 4 1\n5 3 2\n4 4 3\n2 4 1\n5 2 2\n3 5 3\n","output":"6\n"},{"input":"3\n1 1 1\n2 2 2\n3 3 3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBumagaVKletochku"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::{binary_search_first_true, binary_search_last_true};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::cmp::max;

type Point = PointT<i32>;

struct PointWithColor {
    p: Point,
    color: usize,
}

fn solve_x_y(pts: &[PointWithColor]) -> usize {
    let res = solve_x_y_swap(pts);
    let pts2: Vec<_> = pts
        .iter()
        .map(|p_c| PointWithColor {
            p: Point::new(-p_c.p.x, p_c.p.y),
            color: p_c.color,
        })
        .collect();
    max(res, solve_x_y_swap(&pts2))
}

fn solve_xxx(a: &[Point], b: &[Point], c: &[Point]) -> usize {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    let mut c = c.to_vec();
    a.sort_by_key(|p| p.x);
    b.sort_by_key(|p| p.x);
    c.sort_by_key(|p| p.x);
    binary_search_last_true(1..a.len() + 1, |sz| -> bool {
        if let Some(p1) = a.get(sz - 1) {
            if let Some(p2) = b.iter().filter(|p| p.x > p1.x).skip(sz - 1).next() {
                if let Some(p3) = c.iter().filter(|p| p.x > p2.x).skip(sz - 1).next() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    })
    .unwrap_or(0)
}

fn solve_x_y_swap(pts: &[PointWithColor]) -> usize {
    let mut res = 0;
    let mut perm = Permutation::new(3);
    loop {
        let color_from_left = perm[0];
        let color_from_top = perm[1];
        let color_from_bottom = perm[2];

        let mut left: Vec<_> = pts
            .iter()
            .filter(|item| item.color == color_from_left)
            .map(|item| item.p)
            .collect();
        left.sort_by_key(|p| p.x);

        let mut top: Vec<_> = pts
            .iter()
            .filter(|item| item.color == color_from_top)
            .map(|item| item.p)
            .collect();
        top.sort_by_key(|p| p.y);
        top.reverse();

        let mut bottom: Vec<_> = pts
            .iter()
            .filter(|item| item.color == color_from_bottom)
            .map(|item| item.p)
            .collect();
        bottom.sort_by_key(|p| p.y);

        let cur_res = binary_search_last_true(1..pts.len() / 3 + 1, |sz| -> bool {
            if let Some(from_left) = left.get(sz - 1) {
                if let Some(from_top) = top.iter().filter(|p| p.x > from_left.x).skip(sz - 1).next()
                {
                    if let Some(from_bottom) = bottom
                        .iter()
                        .filter(|p| p.x > from_left.x)
                        .skip(sz - 1)
                        .next()
                    {
                        from_bottom.y < from_top.y
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        });
        res.update_max(cur_res.unwrap_or(0));
        res.update_max(solve_xxx(&left, &top, &bottom));

        if !perm.next() {
            break;
        }
    }

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| {
        let p = input.read();
        let color = input.usize() - 1;
        PointWithColor { p, color }
    });
    let mut res = solve_x_y(&a);
    let a_rot: Vec<_> = a
        .iter()
        .map(|item| PointWithColor {
            p: item.p.swap_x_y(),
            color: item.color,
        })
        .collect();
    res.update_max(solve_x_y(&a_rot));
    out_line!(res * 3);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
