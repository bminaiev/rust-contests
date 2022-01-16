//{"name":"A. Astronomy and Bugs","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/?nc=lcCYLmYM","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\n2 2\n3 3\n1 2\n","output":"8\n"},{"input":"10\n19 11\n8 22\n9 7\n8 17\n20 24\n11 11\n16 5\n22 9\n19 22\n20 1\n","output":"53\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAstronomyAndBugs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, dbg};
use algo_lib::geometry::point::PointT;
use std::collections::BTreeMap;

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a : Vec<Point> = input.read_vec(n);
    let mut by_x : BTreeMap<i64, i64> = BTreeMap::new();
    let mut by_y : BTreeMap<i64, i64> = BTreeMap::new();
    for p in a.iter() {
        *by_x.entry(p.x).or_default() += 1;
        *by_y.entry(p.y).or_default() += 1;
    }
    let tot_x = by_x.len() as i64;
    let tot_y = by_y.len() as i64;
    let mut res = tot_x * tot_y;
    for p in a.iter() {
        if by_x.get(&p.x).unwrap() == &1 && by_y.get(&p.y).unwrap() == &1 {
            res -= 1;
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


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
