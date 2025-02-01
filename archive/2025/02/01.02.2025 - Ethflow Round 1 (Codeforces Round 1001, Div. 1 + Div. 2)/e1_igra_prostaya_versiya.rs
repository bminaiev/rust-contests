//{"name":"E1. Игра (простая версия)","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/E1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n4\n2 2 4 3\n1 2\n1 3\n2 4\n5\n1 2 3 4 5\n1 2\n2 3\n3 4\n4 5\n3\n1 2 3\n1 2\n1 3\n5\n3 1 3 4 5\n1 2\n2 3\n3 4\n4 5\n10\n1 2 3 2 4 3 3 4 4 3\n1 4\n4 6\n7 4\n6 9\n6 5\n7 8\n1 2\n2 3\n2 10\n","output":"2\n0\n2\n2\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1IgraProstayaVersiya"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let w = input.vec::<usize>(n).sub_from_all(1);
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let u = input.usize() - 1;
            let v = input.usize() - 1;
            g[u].push(v);
            g[v].push(u);
        }
        let mut tin = vec![0; n];
        let mut tout = vec![0; n];
        let mut timer = 0;
        RecursiveFunction2::new(|f, v: usize, p: usize| {
            tin[v] = timer;
            timer += 1;
            for &to in g[v].iter() {
                if to == p {
                    continue;
                }
                f.call(to, v);
            }
            tout[v] = timer;
            timer += 1;
        })
        .call(0, 0);
        let mut by_w = vec![vec![]; n];
        for i in 0..n {
            by_w[w[i]].push(i);
        }
        let mut f = Fenwick::new(timer);
        let mut res_v = n;
        let mut cnt_alive = 0;
        for w in (0..n).rev() {
            for &v in by_w[w].iter() {
                let inside = f.get_range_sum(tin[v]..tout[v] + 1);
                if inside != cnt_alive {
                    res_v = v;
                    break;
                }
            }
            if res_v != n {
                break;
            }
            for &v in by_w[w].iter() {
                f.add(tin[v], 1);
                cnt_alive += 1;
            }
        }
        if res_v == n {
            out.println(0);
        } else {
            out.println(res_v + 1);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e1_igra_prostaya_versiya";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
