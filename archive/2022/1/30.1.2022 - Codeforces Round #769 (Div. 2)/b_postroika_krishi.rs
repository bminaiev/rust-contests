//{"name":"B. Постройка крыши","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n3\n5\n10\n","output":"0 1\n2 0 1\n3 2 1 0 4\n4 6 3 2 0 8 9 1 7 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPostroikaKrishi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut pow2 = n.next_power_of_two();
    while pow2 >= n {
        pow2 /= 2;
    }
    let mut res = gen_vec(n, id);
    res.swap(0, pow2 - 1);
    for w in res.windows(2) {
        assert!(w[0] ^ w[1] <= pow2);
    }
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
    // tester::run_single_test("1");
    // tester::run_locally();
}
//END MAIN
