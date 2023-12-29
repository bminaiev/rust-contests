//{"name":"B. Интерактивный LowerBound","group":"Codeforces - AIM Tech Round 4 (Div. 1)","url":"https://codeforces.com/problemset/problem/843/B","interactive":true,"timeLimit":1000,"tests":[{"input":"5 3 80\n97 -1\n58 5\n16 2\n81 1\n79 4\n","output":"? 1\n? 2\n? 3\n? 4\n? 5\n! 81\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInteraktivniiLowerBound"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::tester::helper::run_interactive;

fn query(input: &mut Input, out: &mut Output, pos: usize) -> (i32, Option<usize>) {
    out.println(format!("? {}", pos + 1));
    out.flush();
    let x = input.i32();
    let next_pos = input.i32();
    (
        x,
        if next_pos == -1 {
            None
        } else {
            Some(next_pos as usize - 1)
        },
    )
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let start = input.usize() - 1;
    let need_x = input.i32();

    let mut rnd = Random::new(787788);
    let mut best_pos = start;
    let (mut best_x, _) = query(input, out, start);

    for _it in 0..500 {
        let pos = rnd.gen(0..n);
        let (cur_x, _) = query(input, out, pos);
        if cur_x < need_x && cur_x > best_x {
            best_pos = pos;
            best_x = cur_x;
        }
    }

    loop {
        let (cur_x, next_pos) = query(input, out, best_pos);
        if cur_x >= need_x {
            out.println(format!("! {}", cur_x));
            out.flush();
            return;
        }
        if let Some(next_pos) = next_pos {
            best_pos = next_pos;
        } else {
            break;
        }
    }

    out.println("! -1");
    out.flush();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn stress() {
    for it in 1.. {
        dbg!(it);

        let mut rnd = Random::new(it);

        let interactor = move |mut input: Input, mut out: Output| {
            let n = rnd.gen(1..50);
            let mx = rnd.gen(1..1e9 as i32);
            let perm = rnd.gen_permutation(n);
            let mut a = rnd.gen_vec(n, 1..mx);
            let mut sorted_a = a.clone();
            sorted_a.sort();
            for i in 0..n {
                a[perm[i]] = sorted_a[i];
            }

            let need_x = rnd.gen(1..mx);

            let res_x = *sorted_a.iter().find(|x| **x >= need_x).unwrap_or(&-1);

            let mut next = vec![n; n];
            for w in perm.windows(2) {
                next[w[0]] = w[1];
            }

            let start = perm[0];
            out.println(format!("{} {} {}", n, start + 1, need_x));
            out.flush();

            let mut queries = 0;
            loop {
                let q_type = input.string_as_string();
                queries += 1;
                assert!(queries <= 2000);
                if q_type == "!" {
                    let res = input.i32();
                    assert_eq!(res, res_x);
                    break;
                } else {
                    assert_eq!(q_type, "?");
                    let pos = input.usize() - 1;
                    let x = a[pos];
                    let next_pos = next[pos];
                    if next_pos == n {
                        out.println(format!("{} -1", x));
                    } else {
                        out.println(format!("{} {}", x, next_pos + 1));
                    }
                }
                out.flush();
            }
        };

        run_interactive(run, interactor, false);
    }
}

fn main() {
    const PROBLEM_NAME: &str = "b_interaktivnii_lower_bound";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
