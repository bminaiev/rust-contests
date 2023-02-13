//{"name":"D. Монстры (сложная версия)","group":"Codeforces - VK 2022 Finals","url":"https://codeforces.com/gym/425375/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"5 7\n5 5 5 1 3\n2 5\n2 5\n3 3\n3 1\n2 5\n5 1\n3 2\n","output":"4\n4\n2\n4\n4\n5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMonstriSlozhnayaVersiya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    value: i32,
    time: usize,
    id: usize,
}

#[target_feature(enable = "avx2")]
unsafe fn calc_res(a: &[i32]) -> i64 {
    let mut prev = 0;
    let mut res = 0;
    for &x in a.iter() {
        let cond = (x > prev) as i32;
        prev += cond;
        let add = (x - prev) & (-cond);
        res += add as i64;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let start_values = input.vec::<i32>(n);
    let mut elems = vec![];
    for i in 0..n {
        elems.push(Element {
            value: start_values[i],
            time: 0,
            id: i,
        })
    }
    for time in 0..q {
        let id = input.usize() - 1;
        elems.push(Element {
            value: input.i32(),
            time: time + 1,
            id,
        })
    }
    elems.sort();
    let mut last_set_elem = vec![0; n];
    let mut a = vec![0; elems.len()];
    let mut elem_by_time = vec![0; q];
    for i in 0..elems.len() {
        if elems[i].time == 0 {
            last_set_elem[elems[i].id] = i;
            a[i] = elems[i].value;
        } else {
            elem_by_time[elems[i].time - 1] = i;
        }
    }
    for time in 0..q {
        let e_id = elem_by_time[time];
        let id = elems[e_id].id;
        a[last_set_elem[id]] = 0;
        last_set_elem[id] = e_id;
        a[e_id] = elems[e_id].value;
        unsafe {
            out_line!(calc_res(&a));
        }
    }
}

fn stress() {
    let n = 400_000;
    let mut rnd = Random::new(787788);
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = rnd.gen(0..n) as i32;
    }
    let mut res = 0;
    for _ in 0..n / 2 {
        unsafe { res += calc_res(&a) };
    }
    eprintln!("res {res}");
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
