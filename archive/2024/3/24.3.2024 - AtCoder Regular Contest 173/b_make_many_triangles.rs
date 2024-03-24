//{"name":"B - Make Many Triangles","group":"AtCoder - AtCoder Regular Contest 173","url":"https://atcoder.jp/contests/arc173/tasks/arc173_b","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n0 0\n1 1\n0 3\n5 2\n3 4\n2 0\n2 2\n","output":"2\n"},{"input":"3\n0 0\n0 1000000000\n0 -1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMakeManyTriangles"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;

type Point = PointT<i64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |_| Point::new(input.read(), input.read()));
    let mut max_same_line = 2;
    for i in 0..n {
        for j in i + 1..n {
            let mut cnt_same_line = 0;
            for k in 0..n {
                if Point::vect_mul(&a[i], &a[j], &a[k]) == 0 {
                    cnt_same_line += 1;
                }
            }
            max_same_line = max_same_line.max(cnt_same_line);
        }
    }
    let mut res = n / 3;
    res = res.min(n - max_same_line);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_make_many_triangles";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
