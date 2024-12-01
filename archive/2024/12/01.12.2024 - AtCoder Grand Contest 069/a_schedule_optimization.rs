//{"name":"A - Schedule Optimization","group":"AtCoder - AtCoder Grand Contest 069","url":"https://atcoder.jp/contests/agc069/tasks/agc069_a","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n1 4\n1 3\n3 4\n2 2\n3 4\n4 4\n2 3\n3 4\n","output":"1\n"},{"input":"1\n1 1\n1000000000 1000000000\n","output":"999999999\n"},{"input":"4\n158260522 877914575\n24979445 602436426\n623690081 861648772\n433933447 476190629\n211047202 262703497\n628894325 971407775\n731963982 822804784\n430302156 450968417\n161735902 982631932\n880895728 923078537\n189330739 707723857\n802329211 910286918\n303238506 404539679\n317063340 492686568\n125660016 773361868\n650287940 839296263\n","output":"1088492036\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AScheduleOptimization"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type Point = PointT<i64>;

fn extrapolate(x: i64, a: Point, b: Point) -> i64 {
    let dy = (b.y - a.y) / (b.x - a.x);
    assert_eq!(a.y + dy * (b.x - a.x), b.y);
    a.y + dy * (x - a.x)
}

const MX: i64 = 1e9 as i64 + 10;

struct State {
    pts: Vec<Point>,
    max_r: i64,
}

fn sum_linear_functions(ast: &State, bst: &State) -> State {
    let mut res = vec![];
    let mut it1 = 0;
    let mut it2 = 0;
    let a = &ast.pts;
    let b = &bst.pts;
    res.push(Point::new(0, a[0].y + b[0].y));
    while it1 + 1 < a.len() && it2 + 1 < b.len() {
        let x = a[it1 + 1].x.min(b[it2 + 1].x);
        let mut y = extrapolate(x, a[it1], a[it1 + 1]) + extrapolate(x, b[it2], b[it2 + 1]);
        let mut pref = *res.last().unwrap();
        {
            let dx = x - pref.x;
            if ast.max_r < x && bst.max_r < x {
                pref.y += dx;
            }
            if pref.y < y {
                y = pref.y;
            }
        }
        res.push(Point::new(x, y));
        if x == a[it1 + 1].x {
            it1 += 1;
        }
        if x == b[it2 + 1].x {
            it2 += 1;
        }
    }
    assert_eq!(it1 + 1, a.len());
    assert_eq!(it2 + 1, b.len());
    assert_eq!(res.last().unwrap().x, MX);
    State {
        pts: res,
        max_r: ast.max_r.max(bst.max_r),
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let lvl = input.usize();
    let n = 1 << lvl;
    let mut funs = vec![];
    for _ in 0..n {
        let l = input.i64();
        let r = input.i64();
        let mut a = vec![];
        a.push(Point::new(0, l));
        a.push(Point::new(l, 0));
        if l != r {
            a.push(Point::new(r, 0));
        }
        a.push(Point::new(MX, MX - r));
        funs.push(State { pts: a, max_r: r });
    }
    while funs.len() > 1 {
        let mut new_funs = vec![];
        for i in 0..funs.len() / 2 {
            new_funs.push(sum_linear_functions(&funs[2 * i], &funs[2 * i + 1]));
        }
        funs = new_funs;
    }
    let ans = funs[0].pts.iter().map(|p| p.y).min().unwrap();
    out.println(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_schedule_optimization";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
