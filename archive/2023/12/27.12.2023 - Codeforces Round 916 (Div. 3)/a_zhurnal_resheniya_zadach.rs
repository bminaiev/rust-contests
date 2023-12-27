//{"name":"A. Журнал решения задач","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6\nACBCBC\n7\nAAAAFPC\n22\nFEADBBDFFEDFFFDHHHADCC\n","output":"3\n1\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZhurnalResheniyaZadach"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.usize();
    let s = input.string();
    let mut cnt = vec![0; 26];
    for &c in s.iter() {
        cnt[(c - b'A') as usize] += 1;
    }
    let mut res = 0;
    for i in 0..26 {
        if cnt[i] > i {
            res += 1;
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
