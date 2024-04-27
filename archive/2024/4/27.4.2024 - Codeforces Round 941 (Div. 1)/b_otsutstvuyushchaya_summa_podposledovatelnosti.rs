//{"name":"B. Отсутствующая сумма подпоследовательности","group":"Codeforces - Codeforces Round 941 (Div. 1)","url":"https://codeforces.com/contest/1965/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 2\n6 1\n8 8\n9 3\n10 7\n","output":"1\n1\n5\n2 3 4 5 6\n7\n1 1 1 1 1 1 1\n4\n7 1 4 1\n4\n1 2 8 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOtsutstvuyushchayaSummaPodposledovatelnosti"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_case(n: i64, k: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut cur_sum = 0;
    for val in (0..).map(|i| 1 << i) {
        if cur_sum + val >= k {
            break;
        }
        res.push(val);
        cur_sum += val;
    }
    let diff = k - cur_sum - 1;
    if diff > 0 {
        res.push(diff);
        cur_sum += diff;
    }
    let mut can = can_fill(&res, n);
    for add in k + 1..=n {
        let add = add as usize;
        if !can[add] {
            res.push(add as i64);
            for from in (0..can.len()).rev() {
                if from + add < can.len() && can[from] {
                    can[from + add] = true;
                }
            }
        }
    }
    // res.push(k + 1);
    // res.push(k + 2);
    // res.push(k + 3);
    // cur_sum += k + 1 + k + 2;
    // while cur_sum < n {
    //     res.push(cur_sum + 1);
    //     res.push(cur_sum + 2);
    //     cur_sum += cur_sum + 1 + cur_sum + 2;
    // }
    // let mut add = 2 * k;
    // while add <= n {
    //     res.push(add);
    //     add *= 2;
    // }
    // dbg!(res.len());
    assert!(res.len() <= 25);
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.i64();
    let k = input.i64();
    let res = solve_case(n, k);
    out.println(res.len());
    out.println(res);
}

fn can_fill(a: &[i64], n: i64) -> Vec<bool> {
    let mut res = vec![false; n as usize + 1];
    res[0] = true;
    for &x in a.iter() {
        let x = x as usize;
        for from in (0..res.len()).rev() {
            if from + x < res.len() && res[from] {
                res[from + x] = true;
            }
        }
        // dbg!(x, res);
    }
    res
}

fn can(a: &[i64], n: i64, k: i64) {
    let mut res = vec![false; n as usize + 1];
    res[0] = true;
    for &x in a.iter() {
        let x = x as usize;
        for from in (0..res.len()).rev() {
            if from + x < res.len() && res[from] {
                res[from + x] = true;
            }
        }
        // dbg!(x, res);
    }
    // dbg!(a);
    // dbg!(res);
    for i in 1..=n {
        let need = i != k;
        let real = res[i as usize];
        assert_eq!(need, real);
    }
}

fn stress() {
    for n in 1..=1000 {
        for k in 1..=n {
            let res = solve_case(n, k);
            dbg!(n, k, res.len());
            can(&res, n, k);
        }
    }

    for n in 1000000..=1000000 {
        for k in 10..=n {
            let res = solve_case(n, k);
            dbg!(n, k, res.len());
            can(&res, n, k);
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
    const PROBLEM_NAME: &str = "b_otsutstvuyushchaya_summa_podposledovatelnosti";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
