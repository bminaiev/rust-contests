//{"name":"A. И-сопоставление","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 0\n4 1\n4 2\n4 3\n","output":"0 3\n1 2\n0 2\n1 3\n0 1\n2 3\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AISopostavlenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

fn solve_n_k(n: usize, k: usize) {
    if k + 1 == n && n == 4 {
        out_line!(-1);
        return;
    }
    let mut pairs = gen_vec(n, |x| (n - 1 - x));
    let mut add_pair = |x: usize, y: usize| {
        pairs[x] = y;
        pairs[y] = x;
    };
    if n == k + 1 {
        add_pair(1, 3);
        add_pair(n - 1, n - 2);
        add_pair(0, n - 4);
    } else {
        add_pair(k, n - 1);
        add_pair(0, n - 1 - k);
    }
    let mut sum = 0;
    let mut used = vec![false; n];
    let mut cnt = 0;
    for x in 0..n {
        if pairs[x] > x {
            assert!(!used[x]);
            assert!(!used[pairs[x]]);
            used[x] = true;
            used[pairs[x]] = true;
            sum += x & pairs[x];
            cnt += 1;
            out_line!(x, pairs[x]);
        }
    }
    assert_eq!(cnt, n / 2);
    assert_eq!(sum, k);
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    solve_n_k(n, k);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    set_global_output_to_stdout();
    for n in (2..17).map(|x| 1 << x) {
        for k in 0..n {
            dbg!(n, k);
            solve_n_k(n, k);
        }
    }
    // tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
