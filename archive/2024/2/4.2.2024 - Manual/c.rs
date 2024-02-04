//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type Point = PointT<i64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        a.push(Point::new(x, y));
        let c = input.string()[0];
        b.push(c);
    }
    let mut same_line = true;
    for i in 2..n {
        if Point::vect_mul(&a[0], &a[1], &a[i]) != 0 {
            same_line = false;
            break;
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            if b[i] == b[j] {
                if same_line {
                    out.println(2);
                } else {
                    out.println("Infinity");
                }
                return;
            }
        }
    }
    out.println(1);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
