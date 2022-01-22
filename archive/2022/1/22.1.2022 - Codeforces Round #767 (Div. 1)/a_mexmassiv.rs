//{"name":"A. MEX-массив","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n1 0 2 0 3\n8\n2 2 3 4 0 1 2 0\n1\n1\n5\n0 1 2 3 4\n4\n0 1 1 0\n10\n0 0 2 1 1 1 0 0 1 1\n","output":"1\n4\n2\n5 1\n1\n0\n1\n5\n2\n2 2\n4\n3 2 2 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEXMassiv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<usize>(n);
    let mut total_cnt = vec![0; n + 2];
    for &x in a.iter() {
        total_cnt[x] += 1;
    }

    let mut it = 0;
    let mut res = vec![];
    while it != a.len() {
        let mut cur_mex = 0;
        while total_cnt[cur_mex] != 0 {
            cur_mex += 1;
        }
        let mut seen = vec![false; cur_mex];
        let mut total_seen = 0;
        while total_seen != cur_mex {
            let val = a[it];
            total_cnt[val] -= 1;
            it += 1;
            if val < cur_mex && !seen[val] {
                seen[val] = true;
                total_seen += 1;
            }
        }
        if cur_mex == 0 {
            it += 1;
        }
        res.push(cur_mex);
    }
    out_line!(res.len());
    out_line!(res);
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
    tester::run_tests();
}
//END MAIN
