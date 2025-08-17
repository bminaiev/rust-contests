//{"name":"E. Удивительные Мишки Тедди","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\nPPP\n3\nBPP\n3\nPPB\n7\nPPBPPBB\n15\nBPBPBBBBBPBBBBB\n","output":"0\n0\n1\n5\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EUdivitelnieMishkiTeddi"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    // assert that usize is 8 bytes
    assert_eq!(std::mem::size_of::<usize>(), 8);
    for _ in 0..tc {
        let n = input.usize();
        let s = input.string();
        let mut cnt_evens = 0;
        let mut cnt_odds = 0;
        let mut res = 0;
        for i in 0..n {
            if s[i] == b'B' {
                if i % 2 == 0 {
                    let to_pos = cnt_evens * 2;
                    res += (i - to_pos) / 2;
                    cnt_evens += 1;
                } else {
                    let to_pos = cnt_odds * 2 + 1;
                    res += (i - to_pos) / 2;
                    cnt_odds += 1;
                }
            }
        }
        let sum = cnt_evens + cnt_odds;
        let expected_evens = (sum + 1) / 2;
        let expected_odds = sum / 2;
        if cnt_evens > expected_evens {
            let diff = cnt_evens - expected_evens;
            res += diff;
            let start_pos = expected_evens * 2 - 1;
            let need_pos = cnt_odds * 2 + 1;
            assert!(need_pos <= start_pos);
            res += (start_pos - need_pos) * diff / 2;
        }
        if cnt_odds > expected_odds {
            let diff = cnt_odds - expected_odds;
            res += diff;
            let start_pos = expected_odds * 2;
            let need_pos = cnt_evens * 2;
            assert!(need_pos <= start_pos);
            res += (start_pos - need_pos) * diff / 2;
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
    const PROBLEM_NAME: &str = "e_udivitelnie_mishki_teddi";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
