//{"name":"M. Short Question","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/M/","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n1 3 2\n1 2 3\n","output":"6\n"},{"input":"4\n1 1 1000000 1000000\n1000000 1000000 1 1\n","output":"7999992\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MShortQuestion"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Element {
    p: i64,
    q: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.vec::<i64>(n);
    let q = input.vec::<i64>(n);
    let mut elems = gen_vec(n, |id| Element { p: p[id], q: q[id] });

    let max_p = *p.iter().max().unwrap() + 1;
    let max_q = *q.iter().max().unwrap() + 1;
    let max_val = max_p + max_q;

    let mut res = 0;
    {
        elems.sort_by_key(|elem| elem.p);
        let mut fenw_p_plus_q = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_plus_q_cnt = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_minus_q = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_minus_q_cnt = Fenwick::<i64>::new(max_val as usize);

        for e in elems.iter() {
            let my_p_plus_q = (e.p + e.q) as usize;
            let my_p_minus_q = (e.p - e.q + max_q) as usize;

            res += e.p
                * (fenw_p_plus_q_cnt.get_suffix_sum(my_p_plus_q)
                    + fenw_p_minus_q_cnt.get_suffix_sum(my_p_minus_q));

            res -= fenw_p_plus_q.get_suffix_sum(my_p_plus_q)
                + fenw_p_minus_q.get_suffix_sum(my_p_minus_q);

            fenw_p_plus_q.add(my_p_plus_q, e.p);
            fenw_p_plus_q_cnt.add(my_p_plus_q, 1);
            fenw_p_minus_q.add(my_p_minus_q, e.p);
            fenw_p_minus_q_cnt.add(my_p_minus_q, 1);
        }
    }

    {
        elems.sort_by_key(|elem| elem.q);
        let mut fenw_p_plus_q = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_plus_q_cnt = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_minus_q = Fenwick::<i64>::new(max_val as usize);
        let mut fenw_p_minus_q_cnt = Fenwick::<i64>::new(max_val as usize);

        for e in elems.iter() {
            let my_p_plus_q = (e.p + e.q) as usize;
            let my_p_minus_q = (e.q - e.p + max_p) as usize;

            res += e.q
                * (fenw_p_plus_q_cnt.get_suffix_sum(my_p_plus_q + 1)
                    + fenw_p_minus_q_cnt.get_suffix_sum(my_p_minus_q + 1));

            res -= fenw_p_plus_q.get_suffix_sum(my_p_plus_q + 1)
                + fenw_p_minus_q.get_suffix_sum(my_p_minus_q + 1);

            fenw_p_plus_q.add(my_p_plus_q, e.q);
            fenw_p_plus_q_cnt.add(my_p_plus_q, 1);
            fenw_p_minus_q.add(my_p_minus_q, e.q);
            fenw_p_minus_q_cnt.add(my_p_minus_q, 1);
        }
    }

    out_line!(res * 2);
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
}
//END MAIN
