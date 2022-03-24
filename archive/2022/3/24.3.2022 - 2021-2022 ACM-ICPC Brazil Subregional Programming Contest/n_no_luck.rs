//{"name":"N. No Luck","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/N","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n1 2 3 4 5\n1 3 4\n2 6 3\n3 4 1\n","output":"3\n0\n1\n"},{"input":"4 1\n8 8 8 8\n1 7 3\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NNoLuck"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Person {
    id: usize,
    last_year: usize,
    position: i32,
    after: usize,
}

struct Qualified {
    year: usize,
    cnt: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let num_years = input.usize();
    let n = input.usize();
    let ok_slots = input.vec::<i32>(num_years);
    let mut a = gen_vec(n, |id| Person {
        id,
        last_year: input.usize() - 1,
        position: input.i32() - 1,
        after: input.usize(),
    });
    a.sort_by_key(|p| p.position);
    a.reverse();
    let mut qualified = gen_vec(num_years, |year| Qualified {
        year,
        cnt: ok_slots[year],
    });
    qualified.sort_by_key(|q| q.cnt);
    qualified.reverse();
    let mut q_it = 0;
    let mut fenw = Fenwick::new(num_years);
    let mut res = vec![0; n];
    for p in a.iter() {
        while q_it != qualified.len() && qualified[q_it].cnt > p.position {
            fenw.add(qualified[q_it].year, 1);
            q_it += 1;
        }
        if ok_slots[p.last_year] <= p.position {
            res[p.id] = fenw.get_range_sum(p.last_year + 1..p.last_year + p.after + 1);
        }
    }
    out_line!(res);
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
