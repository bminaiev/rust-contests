//{"name":"C. Дерево Фенвика","group":"Codeforces - Codeforces Round 942 (Div. 1)","url":"https://codeforces.com/contest/1967/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n8 1\n1 2 1 4 1 2 1 8\n6 2\n1 4 3 17 5 16\n","output":"1 1 1 1 1 1 1 1\n1 2 3 4 5 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDerevoFenvika"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

type Mod = Mod_998_244_353;

fn stress() {
    for it in 3.. {
        dbg!(it);
        const MX: usize = 50;
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..MX);
        let a = gen_vec(n, |_| Mod::new(rnd.gen(1..MX)));
        let k = rnd.gen(1..MX);
        let smart = solve_smart(a.clone(), k);
        let stupid = solve_stupid(a.clone(), k);
        if smart != stupid {
            dbg!(n, k, a, smart, stupid);
            break;
        }
    }
}

fn solve_stupid(mut a: Vec<Mod>, k: usize) -> Vec<Mod> {
    let n = a.len();
    let mut go = vec![vec![]; n];
    for nxt in 0..n {
        let offseted = nxt + 1;
        let mut lowbit = 1;
        while offseted % (lowbit * 2) == 0 {
            lowbit *= 2;
        }
        for from in nxt + 1 - lowbit..nxt {
            go[from].push(nxt);
        }
    }
    for i in 0..n {
        dbg!(i, &go[i]);
    }
    for _ in 0..k {
        for i in (0..n) {
            let cur = a[i];
            for &to in &go[i] {
                a[to] -= cur;
            }
        }
        // dbg!(a);
    }
    a
}

fn solve_smart(mut a: Vec<Mod>, k: usize) -> Vec<Mod> {
    let n = a.len();
    let mut go = vec![vec![]; n];
    for nxt in 0..n {
        let offseted = nxt + 1;
        let mut lowbit = 1;
        while offseted % (lowbit * 2) == 0 {
            lowbit *= 2;
        }
        for from in nxt + 1 - lowbit..nxt {
            go[from].push(nxt);
        }
    }
    // for i in 0..n {
    //     dbg!(i, &go[i]);
    // }
    // for _ in 0..k {
    // let mut b = vec![Mod::new(0); n];
    let mut mult = vec![Mod::ZERO; 20];
    for it in 0..20 {
        if it > k {
            break;
        }
        mult[it] = Mod::ONE;
        for i in 0..it {
            mult[it] *= Mod::new(k - i);
            mult[it] /= Mod::new(i + 1);
        }
    }
    // dbg!(k);
    // dbg!(mult);
    for i in (0..n).rev() {
        let cur = a[i];
        // if go[i].is_empty() {
        //     continue;
        // }
        for it in 0..go[i].len() {
            let to = go[i][it];
            // let cnt = Mod::new((k as usize - it) as i32);
            let cur = mult[it + 1] * a[i];
            if it % 2 == 0 {
                a[to] -= cur;
            } else {
                a[to] += cur;
            }
        }
        // for &to in &go[i][..1] {
        //     a[to] -= cur;
        // }
    }
    // dbg!(a);
    // a = b;
    // }
    a
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let k = input.i32();
    let mut a = gen_vec(n, |_| Mod::new(input.i32()));
    a = solve_smart(a, k as usize);
    out.println(a);
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
    const PROBLEM_NAME: &str = "c_derevo_fenvika";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
