//{"name":"B. Особые предпочтения в фильмах","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\nzx\nab\ncc\nzx\nba\n2\nab\nbad\n4\nco\ndef\norc\nes\n3\na\nb\nc\n3\nab\ncd\ncba\n2\nab\nab\n","output":"YES\nNO\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOsobiePredpochteniyaVFilmakh"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};
use std::collections::HashSet;

type String = Vec<u8>;

fn exist(words: &[String]) -> bool {
    if words.iter().any(|s| s[0] == *s.last_exn()) {
        return true;
    }
    let mut seen = HashSet::<String>::new();
    for s in words.iter() {
        if s.len() == 2 {
            if seen.contains(&s.reversed()) {
                return true;
            }
        } else {
            assert_eq!(s.len(), 3);
            if seen.contains(&vec![s[2], s[1]]) {
                return true;
            }
            if seen.contains(&s.reversed()) {
                return true;
            }
        }
        seen.insert(s.clone());
    }

    let mut seen_rev = HashSet::<String>::new();
    for s in words.iter().rev() {
        if s.len() == 3 {
            if seen_rev.contains(&vec![s[1], s[0]]) {
                return true;
            }
        }
        seen_rev.insert(s.clone());
    }

    false
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let words = gen_vec(n, |_| input.string_as_vec());
    if exist(&words) {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
