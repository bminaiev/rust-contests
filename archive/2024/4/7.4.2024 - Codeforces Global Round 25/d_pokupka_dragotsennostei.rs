//{"name":"D. Покупка драгоценностей","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n7 3\n6 4\n255 8\n","output":"YES\n10\n2 3 4 5 6 7 8 9 10 11\nNO\nYES\n8\n128 64 32 16 8 4 2 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPokupkaDragotsennostei"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let mut money = input.i64();
    let mut cnt_need = input.i64();
    let mut res = vec![];
    while res.len() < 60 && cnt_need > 0 {
        if money < cnt_need {
            break;
        }
        if cnt_need == money {
            res.push(1);
            cnt_need -= money;
            break;
        } else {
            let want_cost = money - cnt_need + 1;
            if want_cost * 2 <= money {
                break;
            }
            res.push(want_cost);
            money -= want_cost;
            cnt_need -= 1;
        }
    }
    if cnt_need == 0 {
        out.println("YES");
        out.println(res.len());
        out.println(res);
    } else {
        out.println("NO");
    }
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
    const PROBLEM_NAME: &str = "d_pokupka_dragotsennostei";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
