//{"name":"D. В поисках нуля","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/D","interactive":true,"timeLimit":1000,"tests":[{"input":"1\n\n4\n\n2\n\n3\n\n3\n\n2\n","output":"\n\n? 1 2 3\n\n? 2 3 4\n\n? 3 4 1\n\n? 4 1 2\n\n! 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVPoiskakhNulya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn query(input: &mut Input, vals: &[usize]) -> i64 {
    out_line!("?", vals[0] + 1, vals[1] + 1, vals[2] + 1);
    output().flush();
    input.i64()
}

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut best = vec![0, 1, 2];
        let mut cur_max = query(input, &best);
        for it in 3..n {
            let m1 = query(input, &vec![best[0], best[1], it]);
            let m2 = query(input, &vec![best[0], best[2], it]);
            if m1 > cur_max && (m1 >= m2) {
                best[2] = it;
            } else if m2 > cur_max {
                best[1] = it;
            }
            cur_max.update_max(m1);
            cur_max.update_max(m2);
        }
        let mut another = 0;
        while best[0] == another || best[1] == another || best[2] == another {
            another += 1;
        }

        let mut possible_ans = vec![];
        for it in 0..3 {
            let mut check = vec![another];
            for i in 0..3 {
                if i != it {
                    check.push(best[i]);
                }
            }
            if query(input, &check) != cur_max {
                possible_ans.push(best[it]);
            }
        }
        assert!(possible_ans.len() < 3);
        assert!(possible_ans.len() > 0);
        if possible_ans.len() == 1 {
            let cur = possible_ans[0];
            possible_ans.push(cur);
        }
        out_line!("!", possible_ans[0] + 1, possible_ans[1] + 1);
        output().flush();
    }
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_locally();
}
//END MAIN
