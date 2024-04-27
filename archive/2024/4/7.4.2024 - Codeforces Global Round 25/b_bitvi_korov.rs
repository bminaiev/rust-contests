//{"name":"B. Битвы коров","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 1\n12 10 14 11 8 3\n6 5\n7 2 727 10 12 13\n2 2\n1000000000 1\n","output":"1\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBitviKorov"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::lazy_seg_tree::SegTree;
use algo_lib::seg_trees::lazy_seg_tree_max::MaxValNode;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let pos = input.usize() - 1;
    let a = input.vec::<i64>(n);
    let mut res = 0;
    let mut st = SegTree::<MaxValNode<i64>>::new(n, |pos| MaxValNode {
        max_val: a[pos],
        pos,
    });
    for i in 0..n {
        st.update_point(
            i,
            MaxValNode {
                max_val: a[pos],
                pos: i,
            },
        );
        st.update_point(
            pos,
            MaxValNode {
                max_val: a[i],
                pos: i,
            },
        );
        {
            let first_bigger = binary_search_first_true(0..n, |check_pos| {
                st.get(0..check_pos + 1).max_val > a[pos]
            });
            if first_bigger > i {
                let mut cur_res = first_bigger - i;
                if i == 0 {
                    cur_res -= 1;
                }
                res = res.max(cur_res);
            }
        }
        st.update_point(
            pos,
            MaxValNode {
                max_val: a[pos],
                pos: i,
            },
        );
        st.update_point(
            i,
            MaxValNode {
                max_val: a[i],
                pos: i,
            },
        );
    }

    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_bitvi_korov";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
