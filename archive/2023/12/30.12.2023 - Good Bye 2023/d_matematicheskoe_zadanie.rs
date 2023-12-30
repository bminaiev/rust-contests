//{"name":"D.  Математическое задание","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n3\n5\n","output":"1\n169\n196\n961\n16384\n31684\n36481\n38416\n43681\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMatematicheskoeZadanie"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

fn sq(a: &[i32]) -> Vec<i32> {
    let mut res = vec![0; a.len() * 2 + 1];
    for i in 0..a.len() {
        for j in 0..a.len() {
            res[i + j] += a[i] * a[j];
        }
    }
    for i in 0..res.len() - 1 {
        res[i + 1] += res[i] / 10;
        res[i] %= 10;
    }
    while res.len() > 1 && res[res.len() - 1] == 0 {
        res.pop();
    }
    assert!(res[res.len() - 1] < 10);
    res
}

fn mask(a: &[i32]) -> Vec<i32> {
    let mut a = a.to_vec();
    a.sort();
    a
}

fn stress() {}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let mut seen = HashMap::<(usize, Vec<i32>), HashSet<_>>::new();
    let mut ok = vec![false; 100];
    let mut cnt = 0;
    for it in 1.. {
        // dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = rnd.gen(1..51);
        let mut a = vec![0; n];
        a[n - 1] = 1;
        for it in 0..rnd.gen(0..n) {
            a[rnd.gen(0..n)] = 1;
        }

        let a2 = sq(&a);
        if a2 == [0] {
            continue;
        }
        let mask = mask(&a2);
        let len = a2.len();
        if ok[len] {
            continue;
        }
        if seen.entry((len, mask.clone())).or_default().contains(&a2) {
            continue;
        }
        seen.entry((len, mask.clone()))
            .or_default()
            .insert(a2.clone());
        if seen.entry((len, mask.clone())).or_default().len() == len && !ok[len] && len % 2 == 1 {
            ok[len] = true;
            cnt += 1;
            // TODO:!!!

            // dbg!(len, cnt);
            if cnt == 44 {
                break;
            }
        }
    }
    for z in 1..=99 {
        if z % 2 == 1 && !ok[z] {
            // dbg!(z);
        }
    }

    for it in 1.. {
        // dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = rnd.gen(1..8);
        let mut a = vec![0; n];
        a[n - 1] = 1;
        for it in 0..rnd.gen(0..n) {
            a[rnd.gen(0..n)] = rnd.gen(0..4);
        }
        if rnd.gen_bool() {
            a = gen_vec(n, |_| rnd.gen(0..10));
        }

        let a2 = sq(&a);
        if a2 == [0] {
            continue;
        }
        let mask = mask(&a2);
        let len = a2.len();
        if ok[len] {
            continue;
        }
        if seen.entry((len, mask.clone())).or_default().contains(&a2) {
            continue;
        }
        seen.entry((len, mask.clone()))
            .or_default()
            .insert(a2.clone());
        if seen.entry((len, mask.clone())).or_default().len() == len && !ok[len] && len % 2 == 1 {
            ok[len] = true;
            cnt += 1;
            // TODO:!!!

            dbg!(len, cnt);
            if cnt == 50 {
                break;
            }
        }
    }

    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut found = false;
        dbg!(n);
        for ((len, mask), v) in seen.iter() {
            if *len == n && v.len() == n {
                for value in v.iter() {
                    for &z in value.iter().rev() {
                        out.print(z);
                    }
                    out.println("");
                }
                found = true;
                break;
            }
        }
        assert!(found);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_matematicheskoe_zadanie";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
