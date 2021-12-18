//{"name":"E. Замена чисел","group":"Codeforces - Educational Codeforces Round 119 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1620/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1 3\n1 1\n2 1 2\n1 2\n1 1\n1 2\n2 1 3\n","output":"3 2 2 3 2\n"},{"input":"4\n1 1\n1 2\n1 1\n2 2 2\n","output":"1 2 1\n"},{"input":"8\n2 1 4\n1 1\n1 4\n1 2\n2 2 4\n2 4 3\n1 2\n2 2 7\n","output":"1 3 3 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZamenaChisel"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Query {
    Add(u32),
    Replace(u32, u32),
}

fn solve(input: &mut Input, _test_case: usize) {
    let queries = input.usize();
    let mut a = Vec::with_capacity(queries);
    for _ in 0..queries {
        if input.usize() == 1 {
            a.push(Query::Add(input.u32()))
        } else {
            a.push(Query::Replace(input.u32(), input.u32()))
        }
    }
    let mut ids: HashMap<u32, u32> = HashMap::new();
    let mut res = vec![];
    for &query in a.iter().rev() {
        match query {
            Query::Add(val) => res.push(*ids.get(&val).unwrap_or(&val)),
            Query::Replace(what, with) => {
                ids.insert(what, *ids.get(&with).unwrap_or(&with));
            },
        }
    }
    res.reverse();
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
