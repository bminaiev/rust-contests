//{"name":"D. Разделение рёбер","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 7\n1 2\n2 3\n3 4\n4 5\n5 1\n1 3\n3 5\n4 4\n1 2\n2 3\n1 4\n3 4\n6 7\n1 2\n1 3\n3 4\n4 5\n1 4\n5 6\n6 2\n2 1\n1 2\n","output":"0111010\n1001\n0001111\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRazdelenieRyober"}}}

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Edge {
    fr: usize,
    to: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let edges = gen_vec(m, |_| Edge {
        fr: input.usize() - 1,
        to: input.usize() - 1,
    });
    let mut rnd = Random::new(4444);
    let mut res = vec![b'0'; m];
    loop {
        let mut dsu1 = Dsu::new(n);
        let mut dsu2 = Dsu::new(n);
        let p = rnd.gen_permutation(m);
        let mut ok = true;

        for &e_id in p.iter() {
            let e = edges[e_id];
            if dsu1.get(e.fr) != dsu1.get(e.to) {
                dsu1.unite(e.fr, e.to);
                res[e_id] = b'0';
            } else if dsu2.get(e.fr) != dsu2.get(e.to) {
                dsu2.unite(e.fr, e.to);
                res[e_id] = b'1';
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
    }
    out_line!(vec2str(&res));
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
    // tester::run_stress(stress);
}
//END MAIN
