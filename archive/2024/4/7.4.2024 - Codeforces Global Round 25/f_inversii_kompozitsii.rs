//{"name":"F. Инверсии композиции","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4\n2 3 1\n5 5\n2 3 5 1 4\n6 11\n5 1 2 3 4 6\n9 51\n3 1 4 2 5 6 7 8 9\n1 0\n1\n","output":"YES\n3 2 1\nNO\nNO\nYES\n1 5 9 8 7 6 4 3 2\nYES\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FInversiiKompozitsii"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut need_inv = input.i64();
    let start_need_inv = need_inv;
    let p = input.vec::<usize>(n).sub_from_all(1);
    let mut will_be = vec![0; n];
    for i in 0..n {
        will_be[p[i]] = i;
    }
    // dbg!(will_be);
    let mut fenw = Fenwick::new(n);
    let mut base_inv = 0;
    for i in 0..n {
        base_inv += fenw.get_suffix_sum(will_be[i]);
        fenw.add(will_be[i], 1);
    }
    // dbg!(need_inv, base_inv);
    if base_inv > need_inv || (base_inv - need_inv) % 2 != 0 {
        out.println("NO");
        return;
    }
    need_inv -= base_inv;
    let mut res = vec![];
    fenw.clear();
    for i in 0..n {
        let max_here = fenw.get_sum(will_be[i]);
        // dbg!(i, max_here);
        if max_here * 2 <= need_inv {
            res.push(n - i - 1);
            need_inv -= max_here * 2;
        } else {
            let mut first = 0;
            while need_inv > 0 {
                if will_be[first] < will_be[i] {
                    need_inv -= 2;
                }
                first += 1;
            }
            assert_eq!(need_inv, 0);
            res.push(res[first]);
            for j in first..res.len() - 1 {
                res[j] -= 1;
            }
            let mut next = n;
            for _j in i + 1..n {
                res.push(next);
                next += 1;
            }
            let smallest = *res.iter().min().unwrap();
            for j in 0..n {
                res[j] -= smallest;
            }
            break;
        }
        fenw.add(will_be[i], 1);
    }
    assert_eq!(res.len(), n);
    let mut seen = vec![false; n];
    for &x in res.iter() {
        assert!(!seen[x]);
        seen[x] = true;
    }
    let mut pq = vec![0; n];
    for i in 0..n {
        pq[will_be[i]] = res[i];
    }
    if need_inv != 0 {
        out.println("NO");
        return;
    }
    let my_inv = cnt_inv(&pq) + cnt_inv(&res);
    assert_eq!(my_inv, start_need_inv);
    out.println("YES");
    for x in res {
        out.print(x + 1);
        out.print(" ");
    }
    out.println("");
}

fn cnt_inv(a: &[usize]) -> i64 {
    let mut fenw = Fenwick::new(a.len());
    let mut res = 0;
    for &x in a.iter() {
        res += fenw.get_suffix_sum(x);
        fenw.add(x, 1);
    }
    res
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
    const PROBLEM_NAME: &str = "f_inversii_kompozitsii";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
