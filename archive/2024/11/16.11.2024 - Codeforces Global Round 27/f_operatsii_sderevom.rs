//{"name":"F. Операции с деревом","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n2 1\n1 2\n1 2\n3 2\n2 1 3\n2 1\n3 2\n4 1\n1 1 0 1\n1 2\n2 3\n1 4\n12 6\n14 4 5 6 12 9 5 11 6 2 1 12\n3 9\n10 6\n6 12\n4 3\n3 1\n5 11\n9 7\n5 6\n1 8\n2 8\n5 1\n1 1\n0\n","output":"3\n6\n5\n145\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOperatsiiSDerevom"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let root = input.usize() - 1;
    let mut g = vec![vec![]; n];
    let start = input.vec::<i64>(n);
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut queue = vec![root];
    let mut it = 0;
    let mut seen = vec![false; n];
    let mut children = vec![vec![]; n];
    while it < queue.len() {
        let v = queue[it];
        it += 1;
        seen[v] = true;
        for &to in &g[v] {
            if !seen[to] {
                children[v].push(to);
                queue.push(to);
            }
        }
    }
    let mut left = vec![0; n];
    let mut can = |tot_ops: i64| -> bool {
        let every_v_ops = tot_ops / n as i64;
        let remainder = tot_ops % n as i64;
        for &v in queue.iter().rev() {
            let mut ops_here = every_v_ops + if v < remainder as usize { 1 } else { 0 };
            let mut sum_needed = start[v];
            for &ch in children[v].iter() {
                sum_needed += left[ch];
            }

            let use_here = sum_needed.min(ops_here);
            ops_here -= use_here;
            sum_needed -= use_here;
            if sum_needed == 0 {
                left[v] = ops_here % 2;
            } else {
                left[v] = sum_needed;
            }
        }
        left[root] == 0
    };
    let max_start = *start.iter().max().unwrap();
    let definitely_can = (max_start + 10) * ((n + 5) as i64);
    let roughly_can = |ops: i64| -> bool {
        for shift in 0..2 * n as i64 {
            if can(ops + shift) {
                return true;
            }
        }
        false
    };
    let mut start_search = binary_search_first_true(0..definitely_can, roughly_can);
    while !can(start_search) {
        start_search += 1;
    }
    out.println(start_search);
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
    const PROBLEM_NAME: &str = "f_operatsii_sderevom";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
