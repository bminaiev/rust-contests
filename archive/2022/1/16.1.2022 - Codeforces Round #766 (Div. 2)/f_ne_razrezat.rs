//{"name":"F. Не разрезать","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/F?locale=ru","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n8 4\n1 2 1 3\n2 2 2 3\n3 2 3 3\n4 2 4 3\n1 4 2 4\n2 1 3 1\n2 2 3 2\n4 1 4 2\n7 2\n1 1 1 2\n2 1 2 2\n1 1 1 2\n1 1 2 1\n1 2 2 2\n1 1 2 1\n1 2 2 2\n1 6\n3 3 3 4\n","output":"7\n4\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNeRazrezat"}}}

use algo_lib::flows::dinic::FlowDinic;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let cnt_edges = input.usize();
    let sz = input.usize();
    let conv = |x: usize, y: usize| x * sz + y;
    let mut read_id = || {
        let x = input.usize() - 1;
        let y = input.usize() - 1;
        if x < sz / 2 {
            conv(x, y)
        } else {
            conv(sz - x - 1, sz - y - 1)
        }
    };
    let edges = gen_vec(cnt_edges, |_| {
        let id1 = read_id();
        let id2 = read_id();
        assert_ne!(id1, id2);
        (id1, id2)
    });
    let mut res = cnt_edges;
    for col1 in 0..sz / 2 {
        let mut flow = FlowDinic::new(1 + sz * sz / 2 + 1);
        flow.add_edge(0, conv(sz / 2 - 1, col1) + 1, i64::MAX);
        flow.add_edge(conv(sz / 2 - 1, sz - 1 - col1) + 1, flow.n - 1, i64::MAX);
        for &(fr, to) in edges.iter() {
            flow.add_edge(fr + 1, to + 1, 1);
            flow.add_edge(to + 1, fr + 1, 1);
        }
        let min_cut = flow.find_flow() as usize;
        assert!(min_cut <= cnt_edges);
        res.update_min(min_cut);
    }
    out_line!(cnt_edges - res);
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
