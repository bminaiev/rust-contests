//{"name":"E. Маша-забываша","group":"Codeforces - Codeforces Round #764 (Div. 3)","url":"https://codeforces.com/contest/1624/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n\n4 8\n12340219\n20215601\n56782022\n12300678\n12345678\n\n2 3\n134\n126\n123\n\n1 4\n1210\n1221\n\n4 3\n251\n064\n859\n957\n054\n\n4 7\n7968636\n9486033\n4614224\n5454197\n9482268\n","output":"3\n1 4 1\n5 6 2\n3 4 3\n-1\n2\n1 2 1\n2 3 1\n-1\n3\n1 3 2\n5 6 3\n3 4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMashaZabivasha"}}}

use algo_lib::graph::bfs::bfs;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};
use std::collections::BTreeMap;

struct Word {
    id: usize,
    fr: usize,
    to: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let known = input.usize();
    let word_len = input.usize();
    let words = gen_vec(known, |_| input.string_as_vec());
    let need = input.string_as_vec();
    let check_len = [2, 3];
    let mut seen = BTreeMap::new();
    for (id, word) in words.iter().enumerate() {
        for &len in check_len.iter() {
            for (fr, sub) in word.windows(len).enumerate() {
                seen.insert(
                    sub.to_vec(),
                    Word {
                        id,
                        fr,
                        to: fr + len,
                    },
                );
            }
        }
    }
    let mut graph = SimpleGraphT::new(word_len + 1);
    for pos in 0..word_len {
        for &len in check_len.iter() {
            if pos + len <= word_len {
                let need_sub = need[pos..pos + len].to_vec();
                if seen.contains_key(&need_sub) {
                    graph.add_edge(pos, SimpleEdge::new(pos + len));
                }
            }
        }
    }
    let bfs = bfs(0, &graph);
    if let Some(path) = bfs.get_path(word_len) {
        out_line!(path.len() - 1);
        for w in path.windows(2) {
            let need_sub = &need[w[0]..w[1]];
            let word = seen.get(need_sub).unwrap();
            out_line!(word.fr + 1, word.to, word.id + 1);
        }
    } else {
        out_line!(-1);
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
