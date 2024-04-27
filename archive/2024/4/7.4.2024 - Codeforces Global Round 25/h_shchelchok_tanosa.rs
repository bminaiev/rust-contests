//{"name":"H. Щелчок Таноса","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1\n1 2\n2\n4 3 2 1\n3\n5 1 6 4 7 2 8 3\n4\n10 15 6 12 1 3 4 9 13 5 7 16 14 11 2 8\n5\n32 2 5 23 19 17 31 7 29 3 4 16 13 9 30 24 14 1 8 20 6 15 26 18 10 27 22 12 25 21 28 11\n","output":"1\n3 1\n7 5 1\n15 13 9 1\n31 28 25 17 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HShchelchokTanosa"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

#[derive(Clone, Copy, Debug)]
struct Elem {
    plus: i32,
    minus: i32,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let k = input.usize();
    let n = 1 << k;
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut res = vec![];
    for cnt_moves in 1..=k {
        let part_len = n >> cnt_moves;
        let cant_get = binary_search_first_true(0..n, |need_value| {
            let mut elems = vec![];
            for i in (0..n).step_by(part_len) {
                let cnt_plus = a[i..i + part_len]
                    .iter()
                    .copied()
                    .filter(|x| *x >= need_value)
                    .count() as i32;
                if cnt_plus > 0 {
                    elems.push(Elem {
                        plus: cnt_plus - 1,
                        minus: 0,
                    })
                } else {
                    elems.push(Elem { plus: 0, minus: 1 })
                }
            }
            while elems.len() > 1 {
                let mut new_elems = vec![];
                for w in elems.chunks_exact(2) {
                    let first = w[0];
                    let second = w[1];
                    let mut new_elem = Elem {
                        plus: first.plus + second.plus,
                        minus: first.minus + second.minus,
                    };
                    if new_elem.plus > 0 && new_elem.minus > 0 {
                        new_elem.plus -= 1;
                        new_elem.minus -= 1;
                    }
                    new_elems.push(new_elem);
                }
                elems = new_elems;
            }
            elems[0].minus > 0
        });
        res.push(cant_get);
    }
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
    const PROBLEM_NAME: &str = "h_shchelchok_tanosa";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
