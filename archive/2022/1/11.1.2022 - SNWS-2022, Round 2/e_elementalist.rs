//{"name":"E. Elementalist","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/E/","interactive":false,"timeLimit":3000,"tests":[{"input":"24\n0 0\n0 1\n0 2\n0 3\n0 4\n0 5\n1 0\n1 1\n1 2\n1 3\n1 4\n1 5\n2 0\n2 1\n2 2\n2 3\n2 4\n2 5\n3 0\n3 1\n3 2\n3 3\n3 4\n3 5\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EElementalist"}}}

use algo_lib::geometry::point::{PointT, PointWithIdT};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;
type PointWithId = PointWithIdT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let pts = gen_vec(n, |id| PointWithId::new(input.read(), id));
    let rotates = vec![
        Point::new(1, 1),
        Point::new(0, 1),
        Point::new(-1, 1),
        Point::new(-1, 0),
        Point::new(-1, -1),
        Point::new(0, -1),
        Point::new(1, -1),
        Point::new(1, 0),
    ];
    let sorted_by: Vec<Vec<_>> = rotates
        .iter()
        .map(|collinear| -> Vec<PointWithId> {
            let mut res = pts.clone();
            let dir = collinear.rotateCCW();
            res.sort_by_key(|p1| {
                let scal1 = Point::scal_mul2(&p1.p, &dir);
                let scal2 = Point::scal_mul2(&p1.p, collinear);
                (scal1, scal2)
            });
            res
        })
        .collect();
    let mut res = 0;
    for start in 0..n {
        let mut dp = vec![0i64; n];
        dp[start] = 1;
        for (collinear, pts) in rotates.iter().zip(sorted_by.iter()) {
            let mut ndp = vec![0; n];
            let dir = collinear.rotateCCW();
            let mut i = 0;
            while i != pts.len() {
                let mut j = i + 1;
                while j != pts.len() {
                    let scal1 = Point::scal_mul2(&pts[i].p, &dir);
                    let scal2 = Point::scal_mul2(&pts[j].p, &dir);
                    if scal1 != scal2 {
                        break;
                    }
                    j += 1;
                }
                let mut pref_sum = 0;
                while i != j {
                    ndp[pts[i].id()] = pref_sum;
                    pref_sum += dp[pts[i].id()];
                    i += 1;
                }
            }

            dp = ndp;
        }
        res += dp[start];
    }
    out_line!(res);
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
