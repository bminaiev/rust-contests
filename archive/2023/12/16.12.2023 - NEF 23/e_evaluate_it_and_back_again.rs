//{"name":"E. Evaluate It and Back Again","group":"Codeforces - NEF 23","url":"https://codeforces.com/gym/490499/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"1998 -3192\n","output":"2023-12-13\n"},{"input":"413 908\n","output":"12*34+5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEvaluateItAndBackAgain"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Score {
    left_to_right: i64,
    right_to_left: i64,
}

fn stress123() {
    let mut hm = HashMap::new();
    let mut queue = vec![];

    for x in 0..10000 {
        let s = x.to_string();
        let s = s.as_bytes();
        let mut s_rev = s.to_owned();
        s_rev.reverse();
        if s_rev[0] == b'0' {
            continue;
        }
        let right_to_left: i64 = String::from_utf8(s_rev).unwrap().parse().unwrap();
        let score = Score {
            left_to_right: x,
            right_to_left,
        };
        if !hm.contains_key(&score) {
            hm.insert(score, s.to_vec());
            queue.push((score, s.to_vec()));
        }
    }
    let mut it = 0;
    while it < queue.len() {
        let (score1, str1) = queue[it].clone();
        it += 1;
        if score1.left_to_right.abs() > 10 || score1.right_to_left.abs() > 10 {
            continue;
        }
        if score1.left_to_right.abs() < 10 && score1.right_to_left.abs() < 10 {
            dbg!(score1, String::from_utf8(str1.clone()).unwrap());
        }
        for it2 in 0..=it {
            let (score2, str2) = queue[it2].clone();
            {
                let nscore = Score {
                    left_to_right: score1.left_to_right + score2.left_to_right,
                    right_to_left: score1.right_to_left + score2.right_to_left,
                };
                if !hm.contains_key(&nscore) {
                    let mut nstr = vec![b'('];
                    nstr.extend_from_slice(&str1.clone());
                    nstr.push(b'+');
                    nstr.extend_from_slice(&str2);
                    nstr.push(b')');
                    hm.insert(nscore, nstr.clone());
                    queue.push((nscore, nstr));
                }
            }
            {
                // a - b
                let nscore = Score {
                    left_to_right: score1.left_to_right - score2.left_to_right,
                    right_to_left: score2.right_to_left - score1.right_to_left,
                };
                if !hm.contains_key(&nscore) {
                    let mut nstr = vec![b'('];
                    nstr.extend_from_slice(&str1.clone());
                    nstr.push(b'-');
                    nstr.extend_from_slice(&str2);
                    nstr.push(b')');
                    hm.insert(nscore, nstr.clone());
                    queue.push((nscore, nstr));
                }
            }

            {
                // a * b
                let nscore = Score {
                    left_to_right: score1.left_to_right * score2.left_to_right,
                    right_to_left: score2.right_to_left * score1.right_to_left,
                };
                if !hm.contains_key(&nscore) {
                    let mut nstr = vec![b'('];
                    nstr.extend_from_slice(&str1.clone());
                    nstr.push(b'*');
                    nstr.extend_from_slice(&str2);
                    nstr.push(b')');
                    hm.insert(nscore, nstr.clone());
                    queue.push((nscore, nstr));
                }
            }
        }
    }
}

fn rev(s: &str) -> String {
    let mut bytes = s.as_bytes().to_vec();
    bytes.reverse();
    for x in bytes.iter_mut() {
        if *x == b'(' {
            *x = b')';
        } else if *x == b')' {
            *x = b'(';
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn solve(input: &mut Input, _test_case: usize) {
    let sub3 = "((4-1)*(1-2))";
    let expr_1_0 = "((((8+9)-12)+5)+((4-1)*(1-2))+((4-1)*(1-2))+((4-1)*(1-2)))";
    let expr_0_1 = rev(expr_1_0);
    // dbg!(expr_0_1);
    let p = input.i128();
    let q = input.i128();
    for digit in 1..=9 {
        let add = vec![b'0' + digit; 20];
        let add_str = String::from_utf8(add.clone()).unwrap();
        let add_val = String::from_utf8(add).unwrap().parse::<i128>().unwrap();
        let real_add_val = add_val * 3;
        let np = p + real_add_val;
        let nq = q + real_add_val;
        let nps = np.to_string();
        let nps = nps.as_bytes();
        let nqs = nq.to_string();
        let nqs = nqs.as_bytes();
        let mut nqs = nqs.to_vec();
        nqs.reverse();
        if nps[nps.len() - 1] != b'0' && nqs[0] != b'0' {
            let res = format!(
                "{}*{}+{}*{}+{sub3}*{add_str}",
                np,
                expr_1_0,
                String::from_utf8(nqs.to_vec()).unwrap(),
                expr_0_1
            );
            // let res_rev = rev(&res);
            // dbg!(res_rev);
            out_line!(res);
            return;
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
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
