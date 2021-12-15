//{"name":"B. Потерянная биграмма","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\nab bb ba aa ba\n7\nab ba aa ab ba\n3\naa\n5\nbb ab bb\n","output":"abbaaba\nabaabaa\nbaa\nbbabb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPoteryannayaBigramma"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let bigrams: Vec<_> = input
        .read_vec::<String>(n - 2)
        .into_iter()
        .map(|s| s.into_bytes())
        .collect();
    let mut res = vec![bigrams[0][0]];
    for window in bigrams.windows(2) {
        if window[0][1] != window[1][0] {
            res.push(window[0][1]);
        }
        res.push(window[1][0]);
    }
    res.push(bigrams.last().unwrap()[1]);
    if res.len() != n {
        assert_eq!(res.len(), n - 1);
        res.push(b'a');
    }
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
