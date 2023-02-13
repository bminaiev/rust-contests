//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree_add_sum::{Node, SegTreeAddSum};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone)]
struct Event {
    who: usize,
    delta: i64,
}

type ST = SegTreeAddSum<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let events = gen_vec(q, |_| Event {
        who: input.usize() - 1,
        delta: input.i64(),
    });
    let mut cur_score = vec![0; n];
    let mut all_possible_scores = vec![0];
    for ev in events.iter() {
        cur_score[ev.who] += ev.delta;
        all_possible_scores.push(cur_score[ev.who]);
    }
    all_possible_scores.sort();
    all_possible_scores.dedup();
    let pos_of_zero = all_possible_scores.binary_search(&0).unwrap();
    let mut cur_index = vec![pos_of_zero; n];
    let mut st = ST::new(&Node::new(0), all_possible_scores.len(), ());
    let mut st_alive = ST::new(&Node::new(0), all_possible_scores.len(), ());
    st_alive.update(pos_of_zero..pos_of_zero + 1, n as i64);
    let mut res = vec![0; n];
    for ev in events.iter() {
        let id = ev.who;
        let prev_index = cur_index[id];
        let next_score = all_possible_scores[prev_index] + ev.delta;
        let next_index = all_possible_scores.binary_search(&next_score).unwrap();
        cur_index[id] = next_index;
        let (fr, to) = if next_index > prev_index {
            (prev_index, next_index)
        } else {
            (next_index, prev_index)
        };
        res[id] += st.get(prev_index..prev_index + 1).sum;
        st_alive.update(prev_index..prev_index + 1, -1);
        if fr != to {
            st.update(fr..to, 1);
            let cnt_inside = st_alive.get(fr + 1..to + 1).sum;
            res[id] += cnt_inside;
        }
        res[id] -= st.get(next_index..next_index + 1).sum;
        st_alive.update(next_index..next_index + 1, 1);
    }
    for id in 0..n {
        let index = cur_index[id];
        res[id] += st.get(index..index + 1).sum;
    }
    for &x in res.iter() {
        out_line!(x);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
