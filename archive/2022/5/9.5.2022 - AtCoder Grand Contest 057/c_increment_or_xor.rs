//{"name":"C - Increment or Xor","group":"AtCoder - AtCoder Grand Contest 057","url":"https://atcoder.jp/contests/agc057/tasks/agc057_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 0 3 6 1 4 7 2\n","output":"Yes\n4\n-1 6 -1 1\n"},{"input":"3\n2 5 4 3 6 1 0 7\n","output":"No\n"},{"input":"3\n0 1 2 3 4 5 6 7\n","output":"Yes\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIncrementOrXor"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {}

#[derive(Clone, Hash, PartialEq, PartialOrd, Ord, Eq, Debug)]

struct Perm(Vec<i32>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Xor(i32),
    Add,
}

pub fn stress() {
    for pw in 1..5 {
        let n = 1 << pw;
        let p = Perm(gen_vec(n as usize, |x| x as i32));
        let mut seen = HashMap::new();
        seen.insert(p.clone(), vec![]);
        let mut queue = vec![p];
        let mut good = vec![];
        while let Some(p) = queue.pop() {
            let cur_ops = seen.get(&p).unwrap().clone();
            if p.0[0] == 0 && p.0[1] == 1 {
                good.push((p.clone(), cur_ops.clone()));
            }
            {
                let subp = Perm(p.0.iter().map(|&x| (x + n - 1) % n).collect());
                if !seen.contains_key(&subp) {
                    let mut nops = cur_ops.clone();
                    nops.push(Op::Add);
                    seen.insert(subp.clone(), nops);
                    queue.push(subp);
                }
            }
            for x in 0..n {
                let np = Perm(p.0.iter().map(|&y| y ^ x).collect());
                if !seen.contains_key(&np) {
                    let mut nops = cur_ops.clone();
                    nops.push(Op::Xor(x));
                    seen.insert(np.clone(), nops);
                    queue.push(np);
                }
            }
        }
        dbg!(n, seen.len());
        good.sort();
        for (p, mut ops) in good.into_iter() {
            // if ops.len() > 15 {
            //     continue;
            // }
            dbg!(p);
            let z = &p.0;
            let mut pairs_xor = vec![];
            for i in (0..n as usize).step_by(2) {
                pairs_xor.push(z[i] ^ z[i + 1]);
            }
            //dbg!(pairs_xor);
            ops.reverse();
            // dbg!(ops);
            for i in 0..(n as usize) / 2 {
                assert_eq!(z[i], z[i + (n as usize) / 2] ^ (n / 2));
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
}
//END MAIN
