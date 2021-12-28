//{"name":"B. Игра на отрезках","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"http://codeforces.com/contest/1623/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n1 1\n3\n1 3\n2 3\n2 2\n6\n1 1\n3 5\n4 4\n3 6\n4 5\n1 6\n5\n1 5\n1 2\n4 5\n2 2\n4 4\n","output":"1 1 1\n\n1 3 1\n2 2 2\n2 3 3\n\n1 1 1\n3 5 3\n4 4 4\n3 6 6\n4 5 5\n1 6 2\n\n1 5 3\n1 2 1\n4 5 5\n2 2 2\n4 4 4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIgraNaOtrezkakh"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::vec_apply_delta::ApplyDelta2;
use algo_lib::{dbg, out, out_line};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Seg {
    fr: usize,
    to: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut segs: Vec<_> = (0..n)
        .map(|_| {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            Seg { fr, to }
        })
        .collect();
    segs.sort_by_key(|s| s.to - s.fr);
    let mut used = vec![0; n];
    for s in segs.iter() {
        used[s.fr..=s.to].add_to_all(1);
    }
    while let Some(seg) = segs.pop() {
        used[seg.fr..=seg.to].sub_from_all(1);
        let pos = seg.fr + used[seg.fr..].iter().position(|x| *x == 0).unwrap();
        out_line!(seg.fr + 1, seg.to + 1, pos + 1);
    }
    out_line!();
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
