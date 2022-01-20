//{"name":"E. Ecofuel","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 1 3\n1 2 2\n3 2 3\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEcofuel"}}}

use algo_lib::flows::dinic::FlowDinic;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let finish = input.usize() - 1;
    let s1 = input.usize() - 1;
    let s2 = input.usize() - 1;
    let edges = gen_vec(m, |_| (input.usize() - 1, input.usize() - 1, input.i64()));
    let res = binary_search_last_true(0..10_000_000, |each| -> bool {
        let mut flow = FlowDinic::new(n + 2);
        flow.add_edge(0, 1 + s1, each);
        flow.add_edge(0, 1 + s2, each);
        flow.add_edge(1 + finish, flow.n - 1, each * 2);
        for &(fr, to, cap) in edges.iter() {
            flow.add_edge(fr + 1, to + 1, cap * 2);
            flow.add_edge(to + 1, fr + 1, cap * 2);
        }
        flow.find_flow() == each * 2
    })
    .unwrap();
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
