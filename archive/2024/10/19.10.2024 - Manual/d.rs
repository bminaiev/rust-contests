//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let x = input.usize() - 1;
            let y = input.usize() - 1;
            g[x].push(y);
            g[y].push(x);
        }
        let mut perm = vec![];
        for i in 0..n {
            perm.push(i);
        }
        perm.sort_by_key(|i| g[*i].len());
        let mut alive = vec![false; n];
        let mut dsu = Dsu::new(n);
        let mut cnt = vec![0; n];
        let mut ok = true;
        let mut res = vec![];
        for &v in perm.iter() {
            let mut cur_res = vec![v + 1];
            for &to in g[v].iter() {
                if !alive[to] {
                    continue;
                }
                let root = dsu.get(to);
                cnt[root] += 1;
                if cnt[root] == 1 {
                    cur_res.push(to + 1);
                }
            }
            for &to in g[v].iter() {
                if !alive[to] {
                    continue;
                }
                let root = dsu.get(to);
                if cnt[root] != dsu.calc_size(root) {
                    ok = false;
                }
            }
            for &to in g[v].iter() {
                if !alive[to] {
                    continue;
                }
                dsu.unite(v, to);
            }
            for &to in g[v].iter() {
                if !alive[to] {
                    continue;
                }
                let root = dsu.get(to);
                cnt[root] = 0;
            }
            alive[v] = true;
            let sz = cur_res.len() - 1;
            cur_res.insert(1, sz);
            res.push(cur_res);
        }
        if ok {
            out.println("Yes");
            for r in res.into_iter() {
                out.println(r);
            }
        } else {
            out.println("No");
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
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
