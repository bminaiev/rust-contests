//{"name":"E. Tokitsukaze и красивые подотрезки","group":"Codeforces - Codeforces Round #789 (Div. 1)","url":"https://codeforces.com/contest/1677/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"8 3\n1 3 5 2 4 7 6 8\n1 3\n1 1\n1 8\n","output":"2\n0\n10\n"},{"input":"10 10\n6 1 3 2 5 8 4 10 7 9\n1 8\n1 10\n1 2\n1 4\n2 4\n5 8\n4 10\n4 7\n8 10\n5 9\n","output":"17\n25\n1\n5\n2\n0\n4\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETokitsukazeIKrasiviePodotrezki"}}}

use std::cmp::{max, min};

use algo_lib::collections::sparse_table_max::SparseTableMax;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let p = input.vec::<i64>(n);

    let mut q_by_left = vec![vec![]; n];
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        q_by_left[l].push(Query { l, r, id });
    }

    let table_max = SparseTableMax::new(&p);

    let mut indexes = gen_vec(n, id);
    indexes.sort_by_key(|id| p[*id]);

    let mut good_ends = vec![vec![]; n];

    for i_pos in 0..n {
        let i = indexes[i_pos];
        let pi = p[i];
        for j_pos in i_pos + 1..n {
            let j = indexes[j_pos];
            let pj = p[j];
            if pi * pj > n as i64 {
                break;
            }
            let fr = min(i, j);
            let to = max(i, j) + 1;
            let cur_max = p[table_max.find_max_pos(fr..to)];
            if cur_max == pi * pj {
                good_ends[fr].push(to);
            }
        }
    }

    let mut res = vec![0; q];

    let mut fenw = Fenwick::<i32>::new(n + 1);

    for l in (0..n).rev() {
        for &end in good_ends[l].iter() {
            fenw.add(end, 1);
        }
        for query in q_by_left[l].iter() {
            res[query.id] = fenw.get_sum(query.r);
        }
    }

    out_line!(res);
}

#[derive(Clone)]
struct Query {
    l: usize,
    r: usize,
    id: usize,
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
