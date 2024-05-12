//{"name":"A. Подсчет перестановок","group":"Codeforces - Codeforces Round 942 (Div. 1)","url":"https://codeforces.com/contest/1967/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n1 10\n1\n2 4\n8 4\n3 4\n6 1 8\n3 9\n7 6 2\n5 3\n6 6 7 4 6\n9 7\n7 6 1 7 6 2 4 3 3\n10 10\n1 3 1 2 1 9 3 5 7 5\n9 8\n5 8 7 5 1 3 2 9 8\n","output":"11\n15\n15\n22\n28\n32\n28\n36\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APodschetPerestanovok"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut k = input.i64();
    let mut cnt = input.vec::<i64>(n);
    cnt.sort();
    const MX: i64 = 4e12 as i64;
    let cant_do = binary_search_first_true(0..MX, |need| {
        let mut more = 0;
        for i in 0..n {
            if cnt[i] < need {
                more += need - cnt[i];
            }
        }
        more > k
    });
    let mut res = 0;
    for i in 0..n {
        if cnt[i] > cant_do {
            cnt[i] = cant_do;
        } else {
            let here = k.min(cant_do - cnt[i]);
            cnt[i] += here;
            k -= here;
        }
        res += cnt[i];
    }
    res += 1;
    res -= n as i64;
    res = res.max(0);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_podschet_perestanovok";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
