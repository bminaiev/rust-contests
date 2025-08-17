//{"name":"C. Манхэттенские пары","group":"Codeforces - Order Capital Round 1 (Codeforces Round 1038, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2122/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4\n1 1\n3 0\n4 2\n3 4\n10\n-1 -1\n-1 2\n-2 -2\n-2 0\n0 2\n2 -3\n-4 -4\n-4 -2\n0 1\n-4 -2\n","output":"4 1\n2 3\n8 1\n9 10\n7 5\n2 3\n6 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMankhettenskiePari"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    id: usize,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            let x = input.i64();
            let y = input.i64();
            points.push(Point { x, y, id: i });
        }
        let mut x_type = vec![0; n];
        let mut y_type = vec![0; n];
        points.sort_by_key(|p| p.x);
        for i in n / 2..n {
            x_type[points[i].id] = 1; // right half
        }
        points.sort_by_key(|p| p.y);
        for i in n / 2..n {
            y_type[points[i].id] = 1; // upper half
        }
        points.sort_by_key(|p| p.id);
        let mut by_type = vec![vec![vec![]; 2]; 2];
        for i in 0..n {
            by_type[x_type[i]][y_type[i]].push(points[i]);
        }
        assert_eq!(by_type[0][0].len(), by_type[1][1].len());
        assert_eq!(by_type[0][1].len(), by_type[1][0].len());
        for i in 0..by_type[0][0].len() {
            let p1 = by_type[0][0][i];
            let p2 = by_type[1][1][i];
            out.println(vec![p1.id + 1, p2.id + 1]);
        }
        for i in 0..by_type[0][1].len() {
            let p1 = by_type[0][1][i];
            let p2 = by_type[1][0][i];
            out.println(vec![p1.id + 1, p2.id + 1]);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_mankhettenskie_pari";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
