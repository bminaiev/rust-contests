//{"name":"A. Запрещённая подпоследовательность","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"http://codeforces.com/contest/1617/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\nabacaba\nabc\ncccba\nacb\ndbsic\nbac\nabracadabra\nabc\ndddddddddddd\ncba\nbbc\nabc\nac\nabc\n","output":"aaaacbb\nabccc\nbcdis\naaaaacbbdrr\ndddddddddddd\nbbc\nac\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZapreshchyonnayaPodposledovatelnost"}}}

use algo_lib::io::output::output;
use algo_lib::{collections::sorted::SortedTrait, strings::utils::byte2str};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
use algo_lib::{io::input::Input, strings::utils::vec2str};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let t = input.string();
    if t == "abc".as_bytes() && s.contains(&b'a') {
        for c in "acb".bytes().into_iter().chain(b'd'..=b'z') {
            for x in s.iter() {
                if *x == c {
                    out!(byte2str(c));
                }
            }
        }
        out_line!();
    } else {
        out_line!(vec2str(&s.sorted()));
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
    // tester::run_single_test("1");
}
//END MAIN
