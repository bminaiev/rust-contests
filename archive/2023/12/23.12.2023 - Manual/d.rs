//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::collections::HashSet;

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut g = vec![vec![]; n];
    let mut edges = vec![HashSet::new(); 2];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let color = input.usize();
        edges[color].insert((fr as i32, to as i32));
        edges[color].insert((to as i32, fr as i32));
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut queue = vec![];
    let mut seen = FxHashSet::default();
    let mut ans = FxHashSet::default();
    for v in 0..n {
        queue.push(vec![v as i32]);
        seen.insert(vec![v as i32]);
        ans.insert(vec![v as i32]);
    }
    while let Some(vertices) = queue.pop() {
        {
            let mut in_blue = vec![];
            let mut in_red = vec![];
            let mut count_blue = 0;
            let mut count_red = 0;
            for &x in &vertices {
                for &y in &vertices {
                    if x == y {
                        break;
                    }
                    if edges[0].contains(&(x, y)) {
                        in_red.push(x);
                        in_red.push(y);
                        count_red += 1;
                    }
                    if edges[1].contains(&(x, y)) {
                        in_blue.push(x);
                        in_blue.push(y);
                        count_blue += 1;
                    }
                }
            }
            in_blue.sort();
            in_red.sort();
            in_blue.dedup();
            in_red.dedup();
            let sz = vertices.len();
            if in_blue.len() == sz
                && in_red.len() == sz
                && count_red >= sz - 1
                && count_blue >= sz - 1
            {
                ans.insert(vertices.clone());
            }
        }
        if vertices.len() < 4 {
            for v in vertices.iter() {
                for &to in g[*v as usize].iter() {
                    if !vertices.contains(&(to as i32)) {
                        let mut new_vertices = vertices.clone();
                        new_vertices.push(to as i32);
                        new_vertices.sort();
                        if seen.insert(new_vertices.clone()) {
                            queue.push(new_vertices.clone());
                            seen.insert(new_vertices);
                        }
                    }
                }
            }
        }
    }
    out_line!(ans.len());
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
