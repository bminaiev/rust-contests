//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo_pair::ModPair998_007;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::strings::hash_string_context::HashContext;

type Mod = ModPair998_007;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let m = input.usize();
    let k = input.usize();
    let mut s = vec![];
    let hashes = HashContext::new(m + 10, Mod::new(239017));
    for _ in 0..n {
        let cur_s = input.string();
        let cur_s = hashes.make_string(&cur_s);
        s.push(cur_s);
    }
    for _ in 0..q {
        let cur_s = input.string();
        let cur_s = hashes.make_string(&cur_s);
        let mut res = 0;
        for i in 0..n {
            let mut fails = 0;
            let mut from = 0;
            while from < m && fails <= k {
                let to = binary_search_first_true(from..m + 1, |pos| {
                    s[i].calc_hash(from..pos) != cur_s.calc_hash(from..pos)
                });
                if to != m + 1 {
                    fails += 1;
                }
                from = to;
            }
            if fails <= k {
                res += 1;
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
