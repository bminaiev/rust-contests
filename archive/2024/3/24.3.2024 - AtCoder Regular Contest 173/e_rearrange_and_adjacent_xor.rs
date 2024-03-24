//{"name":"E - Rearrange and Adjacent XOR","group":"AtCoder - AtCoder Regular Contest 173","url":"https://atcoder.jp/contests/arc173/tasks/arc173_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 3 4\n","output":"7\n"},{"input":"13\n451745518671773958 43800508384422957 153019271028231120 577708532586013562 133532134450358663 619750463276496276 615201966367277237 943395749975730789 813856754125382728 705285621476908966 912241698686715427 951219919930656543 124032597374298654\n","output":"1152905479775702586\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERearrangeAndAdjacentXOR"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

fn solve_max_xor(a: &[u64]) -> u64 {
    let mut pairs = vec![];
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            pairs.push(a[i] ^ a[j]);
        }
    }
    let mut res = 0;
    let mut start = 0;
    for bit in (0..61).rev() {
        let mut best = usize::MAX;
        for i in start..pairs.len() {
            if (pairs[i] >> bit) & 1 == 1 {
                best = i;
                break;
            }
        }
        if best == usize::MAX {
            continue;
        }
        pairs.swap(start, best);
        for i in start + 1..pairs.len() {
            if (pairs[i] >> bit) & 1 == 1 {
                pairs[i] ^= pairs[start];
            }
        }
        if res & (1 << bit) == 0 {
            res ^= pairs[start];
        }
        start += 1;
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<u64>(n);
    let mut res = 0;
    if n > 2 && (n % 4 == 2) {
        for i in 0..n {
            let mut b = a.clone();
            b.remove(i);
            res = res.max(solve_max_xor(&b));
        }
    } else {
        res = solve_max_xor(&a);
    }
    out.println(res);
}

fn stress() {
    for n in 1..20 {
        let mut seen = vec![false; n + 1];
        for it in 1..1_000_000 {
            let mut rnd = Random::new(it);
            let mut a = gen_vec(n, |i| 1i32 << i);
            while a.len() > 1 {
                rnd.shuffle(&mut a);
                let b: Vec<_> = a.windows(2).map(|w| w[0] ^ w[1]).collect();
                a = b;
            }
            let res = a[0].count_ones() as usize;
            if !seen[res] {
                seen[res] = true;
                dbg!(n, it, res);
            }
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
    const PROBLEM_NAME: &str = "e_rearrange_and_adjacent_xor";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
