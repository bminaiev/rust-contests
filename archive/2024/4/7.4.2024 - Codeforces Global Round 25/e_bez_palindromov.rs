//{"name":"E. Без палиндромов","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nsinktheyacht\nlllllllll\nuwuowouwu\n","output":"YES\n1\nsinktheyacht\nNO\nYES\n3\nuw uow ouwu\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBezPalindromov"}}}

use std::cmp::min;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::strings::hash_string_context::{HashContext, HashString};

type Mod = Mod_998_244_353;

fn solve_case(s: &[u8]) -> Option<Vec<&[u8]>> {
    let n = s.len();
    let s_rev = s.iter().rev().copied().collect::<Vec<_>>();
    if s != s_rev {
        return Some(vec![s]);
    }
    let mut hashes = HashContext::new(s.len() + 1, Mod::new(239017));
    let mut hash = hashes.make_string(&s);
    let mut hash_rev = hashes.make_string(&s_rev);
    let is_palindrome = |start: usize, end: usize| {
        hash.calc_hash(start..end) == hash_rev.calc_hash(n - end..n - start)
    };
    for i in 1..n {
        if !is_palindrome(0, i) && !is_palindrome(i, n) {
            return Some(vec![&s[..i], &s[i..]]);
        }
    }
    None
}

fn is_polindrome(s: &[u8]) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

fn ok_stupid(s: &[u8]) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for start in 0..n {
        if dp[start] {
            for end in start + 1..=n {
                if !is_polindrome(&s[start..end]) {
                    dp[end] = true;
                }
            }
        }
    }
    dp[n]
}

fn ok_two(s: &[u8]) -> bool {
    if !is_polindrome(s) {
        return true;
    }
    for half in 1..s.len() {
        if !is_polindrome(&s[..half]) && !is_polindrome(&s[half..]) {
            return true;
        }
    }
    false
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..20);
        let max_c = rnd.gen(2..5);
        let a = rnd.gen_vec(n, 0..max_c);
        let r1 = ok_stupid(&a);
        let r2 = ok_two(&a);
        if r1 != r2 {
            dbg!(a, r1, r2);
            assert!(false);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let s = input.string();
    if let Some(res) = solve_case(&s) {
        out.println("YES");
        out.println(res.len());
        for &part in res.iter() {
            let str = String::from_utf8(part.to_vec()).unwrap();
            out.print(str);
            out.print(" ");
        }
        out.println("");
    } else {
        out.println("NO");
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
    const PROBLEM_NAME: &str = "e_bez_palindromov";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
