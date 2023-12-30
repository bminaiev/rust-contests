//{"name":"F. Разделение на группы","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 3\n2 3\n1 3\n1 2\n1 4 7\n2 5\n3 4\n2 4\n1 2\n3 5\n4 5\n1 5\n3 3 7\n1 2\n1 6\n2 3\n2 5\n3 4\n4 5\n4 6\n","output":"3\n1 2\n5\n1 2 3 4\n4 5 6\n1 2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRazdelenieNaGruppi"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

fn possible(mut need: usize, sz: &[usize], children: &[usize]) -> Vec<usize> {
    let mut sum = 0;
    for &x in children.iter() {
        sum += sz[x];
    }
    if sum < need {
        return vec![];
    }
    let mut dp = Array2D::new(false, children.len() + 1, need + 1);
    dp[0][0] = true;
    for i in 0..children.len() {
        for j in 0..=need {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + sz[children[i]] <= need {
                    dp[i + 1][j + sz[children[i]]] = true;
                }
            }
        }
    }
    if !dp[children.len()][need] {
        return vec![];
    }
    let mut res = vec![];
    let mut cur = need;
    for i in (0..children.len()).rev() {
        if cur >= sz[children[i]] && dp[i][cur - sz[children[i]]] {
            res.push(children[i]);
            cur -= sz[children[i]];
        }
    }
    assert_eq!(cur, 0);
    res

    // let mut res = vec![];
    // for &x in children.iter() {
    //     if sz[x] <= need {
    //         res.push(x);
    //         need -= sz[x];
    //     }
    // }
    // if need == 0 {
    //     res
    // } else {
    //     vec![]
    // }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n1 = input.usize();
    let n2 = input.usize();
    let m = input.usize();
    let n = n1 + n2;
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut rnd = Random::new(88987328);
    let mut sz = vec![0; n];
    let mut seen = vec![false; n];
    let mut res_xor = vec![0; n];
    loop {
        for v in 0..n {
            rnd.shuffle(&mut g[v]);
        }
        let root = rnd.gen(0..n);
        for x in seen.iter_mut() {
            *x = false;
        }
        let ok = RecursiveFunction2::new(|f, v: usize, _p: usize| -> bool {
            sz[v] = 1;
            seen[v] = true;
            let mut children = vec![];
            for &to in g[v].iter() {
                if !seen[to] {
                    children.push(to);
                    if f.call(to, v) {
                        return true;
                    }
                    sz[v] += sz[to];
                }
            }
            children.sort_by_key(|&x| sz[x]);
            children.reverse();
            for need in [n1 - 1, n2 - 1].iter() {
                let res = possible(*need, &sz, &children);
                if !res.is_empty() {
                    res_xor[v] = 1;
                    for to in res.iter() {
                        res_xor[*to] ^= 1;
                    }
                    for c in children.iter() {
                        res_xor[*c] ^= 1;
                    }
                    return true;
                }
            }
            false
        })
        .call(root, root);
        if ok {
            let mut g1 = vec![];
            let mut g2 = vec![];
            for x in seen.iter_mut() {
                *x = false;
            }
            RecursiveFunction2::new(|f, v: usize, mut color: usize| {
                color ^= res_xor[v];
                seen[v] = true;
                if color == 0 {
                    g1.push(v + 1);
                } else {
                    g2.push(v + 1);
                }
                for &to in g[v].iter() {
                    if !seen[to] {
                        f.call(to, color);
                    }
                }
            })
            .call(root, 0);
            if g1.len() == n2 {
                std::mem::swap(&mut g1, &mut g2);
            }
            assert_eq!(g1.len(), n1);
            assert_eq!(g2.len(), n2);
            out.println(g1);
            out.println(g2);
            break;
        }
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
    const PROBLEM_NAME: &str = "f_razdelenie_na_gruppi";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
