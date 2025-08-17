//{"name":"B. Удивительные перчатки","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 3\n1 1 1\n1 1 1\n1 1\n100\n1\n3 2\n100 1 1\n200 1 1\n5 2\n97 59 50 87 36\n95 77 33 13 74\n10 6\n97 59 50 87 36 95 77 33 13 74\n91 14 84 33 54 89 68 34 14 15\n","output":"6\n101\n303\n481\n1010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BUdivitelniePerchatki"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, Default)]
struct A {
    left: i64,
    right: i64,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut a = vec![A::default(); n];
        for i in 0..n {
            a[i].left = input.i64();
        }
        for i in 0..n {
            a[i].right = input.i64();
        }
        a.sort_by_key(|x| (x.left.min(x.right)));
        a.reverse();
        let mut sum = 0;
        for i in 0..k - 1 {
            sum += a[i].left + a[i].right;
        }
        for i in k - 1..n {
            sum += a[i].left.max(a[i].right);
        }
        // let mut res = 0;
        // for pos in 0..n {
        //     let mut cur = sum + 1;
        //     if pos < k - 1 {
        //         cur -= a[pos].left + a[pos].right;
        //         cur += a[k - 1].left + a[k - 1].right;
        //     }
        //     res = res.max(cur);
        // }
        out.println(sum + 1);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_udivitelnie_perchatki";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
