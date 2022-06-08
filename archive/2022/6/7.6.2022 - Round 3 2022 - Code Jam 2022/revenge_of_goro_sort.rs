//{"name":"Revenge of GoroSort","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b45189","interactive":false,"timeLimit":20000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RevengeOfGoroSort"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, n: usize) -> i32 {
    let mut p = input.vec::<usize>(n).sub_from_all(1);
    let mut ops = 0;
    let mut first_iter = true;
    let mut rnd = Random::new(898899);
    loop {
        // dbg!("next iter");
        let mut seen = vec![false; n];
        let mut blocks = vec![];
        for i in 0..n {
            if seen[i] {
                continue;
            }
            let mut cycle = vec![];
            let mut cur = i;
            while !seen[cur] {
                cycle.push(cur);
                seen[cur] = true;
                cur = p[cur];
            }
            if cycle.len() == 1 {
                continue;
            }
            // dbg!(cycle.len());
            if cycle.len() >= 3 {
                const S: usize = 3;
                for i in (0..cycle.len()).step_by(S) {
                    blocks.push(cycle[i..min(cycle.len(), i + S)].iter().cloned().collect());
                }
            } else {
                // dbg!(cycle.len());
                for i in (0..cycle.len()).step_by(2) {
                    if i + 1 < cycle.len() {
                        blocks.push(vec![cycle[i], cycle[i + 1]]);
                    }
                }
            }
        }
        ops += 1;
        // if first_iter {
        //     first_iter = false;
        //     blocks = vec![vec![]; 10];
        //     for i in 0..n {
        //         blocks[rnd.gen(0..10)].push(i);
        //     }
        // }
        assert!(blocks.len() != 0);
        let mut colors = vec![0; n];
        let mut cur_color = 1;
        // dbg!(blocks);
        for b in blocks.iter() {
            for &pos in b.iter() {
                assert!(colors[pos] == 0);
                colors[pos] = cur_color;
            }
            cur_color += 1;
        }
        for i in 0..n {
            if colors[i] == 0 {
                colors[i] = cur_color;
                cur_color += 1;
            }
        }
        out_line!(colors);
        output().flush();
        let res = input.i32();
        assert_ne!(res, -1);
        if res == 1 {
            break;
        }
        assert_eq!(res, 0);
        let np = input.vec::<usize>(n).sub_from_all(1);
        for b in blocks.iter() {
            let mut aa = vec![];
            let mut bb = vec![];
            for &pos in b.iter() {
                aa.push(p[pos]);
                bb.push(np[pos]);
            }
            aa.sort();
            bb.sort();
            assert_eq!(aa, bb);
        }
        p = np;
    }
    ops
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let n = input.usize();
    let k = input.usize();
    let mut sum_ops = 0;
    for i in 0usize..t {
        // dbg!(n, i, t, sum_ops);
        sum_ops += solve(&mut input, n);
    }
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
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

fn stress() {
    let n = 100;
    let mut rnd = Random::new(7877881);
    let mut sum_ops = 0;
    for tc in 0..1000 {
        let mut p = rnd.gen_permutation(n);
        let mut ops = 0;
        loop {
            let mut seen = vec![false; n];
            let mut blocks = vec![];
            for i in 0..n {
                if seen[i] {
                    continue;
                }
                if p[i] == i {
                    continue;
                }
                blocks.push(vec![i, p[i]]);
                seen[i] = true;
                seen[p[i]] = true;
            }
            if blocks.len() == 0 {
                break;
            }
            ops += 1;
            for b in blocks.iter() {
                let sz = b.len();
                let p2 = rnd.gen_permutation(sz);
                let mut cur = vec![];
                for &pos in b.iter() {
                    cur.push(p[pos]);
                }
                for i in 0..b.len() {
                    p[b[i]] = cur[p2[i]];
                }
            }
        }
        for i in 0..n {
            assert_eq!(p[i], i);
        }

        sum_ops += ops;
        dbg!(tc, ops, sum_ops);
    }
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
