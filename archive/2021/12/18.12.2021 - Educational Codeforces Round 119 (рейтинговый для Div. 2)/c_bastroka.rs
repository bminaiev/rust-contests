//{"name":"C. BA-строка","group":"Codeforces - Educational Codeforces Round 119 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1620/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 4 3\na*\n4 1 3\na**a\n6 3 20\n**a***\n","output":"abb\nabba\nbabbbbbbbbb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBAStroka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i64();
    let mut x = input.i64() - 1;
    let s = input.string_as_vec();
    assert_eq!(s.len(), n);
    let mut groups = vec![];
    let mut i = 0;
    while i != s.len() {
        let mut j = i;
        while j != s.len() && s[i] == s[j] {
            j += 1;
        }
        groups.push(s[i..j].to_owned());
        i = j;
    }
    let mut res = vec![];
    for group in groups.iter().rev() {
        if group[0] == b'a' {
            res.append(&mut group.clone());
        } else {
            let max_cnt = k * (group.len() as i64);
            let use_cnt = x % (max_cnt + 1);
            res.append(&mut vec![b'b'; use_cnt as usize]);
            x /= (max_cnt + 1);
        }
    }
    assert_eq!(x, 0);
    res.reverse();
    out_line!(String::from_utf8(res).unwrap());
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
