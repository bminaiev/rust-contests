//{"name":"G - Range Sort Query","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_g","interactive":false,"timeLimit":8000,"tests":[{"input":"5 2 1\n1 4 5 2 3\n1 3 5\n2 1 3\n","output":"3\n"},{"input":"7 3 3\n7 5 3 1 2 4 6\n1 1 7\n2 3 6\n2 5 7\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GRangeSortQuery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::LazySegTree;
use algo_lib::seg_trees::lazy_seg_tree_set_sum::{Node, SegTreeSetSum};
use algo_lib::{dbg, out, out_line};

struct Query {
    type_: usize,
    l: usize,
    r: usize,
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let q = input.usize();
    let x = input.usize() - 1;
    let p = input.read_vec::<usize>(n).sub_from_all(1);
    let queries = gen_vec(q, |_| Query {
        type_: input.usize(),
        l: input.usize() - 1,
        r: input.usize(),
    });
    let solve_less_val = |val: usize| -> SegTreeSetSum {
        let mut seg_tree = SegTreeSetSum::new_f(
            n,
            &|pos| Node {
                len: 1,
                sum: if p[pos] < val { 1 } else { 0 },
            },
            (),
        );
        for query in queries.iter() {
            let cnt = seg_tree.get(query.l, query.r).sum as usize;
            let more = query.r - query.l - cnt;
            if query.type_ == 2 {
                seg_tree.modify(query.l, query.l + more, 0);
                seg_tree.modify(query.l + more, query.r, 1);
            } else {
                assert_eq!(query.type_, 1);
                seg_tree.modify(query.l, query.l + cnt, 1);
                seg_tree.modify(query.l + cnt, query.r, 0);
            }
        }
        seg_tree
    };
    let mut s1 = solve_less_val(x);
    let mut s2 = solve_less_val(x + 1);
    let pos =
        binary_search_first_true(0..n, |pos| s2.get(0, pos + 1).sum != s1.get(0, pos + 1).sum);
    out_line!(pos + 1);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
