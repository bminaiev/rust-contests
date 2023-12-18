//{"name":"F. Бешеная погоня","group":"Codeforces - NEF 23","url":"https://codeforces.com/gym/490499/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n1 2\n2\n","output":"1\n"},{"input":"3\n1 2\n1 3\n1\n","output":"2\n"},{"input":"4\n4 3\n4 1\n4 2\n4\n","output":"3.66667\n"},{"input":"7\n1 4\n4 5\n5 2\n4 6\n6 7\n7 3\n3\n","output":"8.3525\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBeshenayaPogonya"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Way {
    len: f64,
    ans: f64,
}

fn best(ways: &[Way]) -> f64 {
    let mut left = 0.0;
    let mut right = 1e9;
    for _it in 0..100 {
        let mid = (left + right) / 2.0;
        let mut sum_pr = 0.0;
        for w in ways.iter() {
            let pr = (w.len + w.ans - mid) / w.ans;
            if pr > 1.0 {
                sum_pr += 1.0;
            } else if pr > 0.0 {
                sum_pr += pr;
            }
        }
        if sum_pr < 1.0 {
            right = mid;
        } else {
            left = mid;
        }
    }
    left
}

fn solve_case(mut g: Array2D<i32>, root: usize) -> f64 {
    let n = g.len();
    let mut leafs = vec![];
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {
            if g[i][j] == 1 {
                cnt += 1;
            }
        }
        if cnt == 1 {
            leafs.push(i);
        }
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                g[j][k] = g[j][k].min(g[j][i] + g[i][k]);
            }
        }
    }
    let mut res = vec![1.0; leafs.len()];
    for _it in 0..5000 {
        for i in 0..leafs.len() {
            let mut ways = Vec::with_capacity(leafs.len() - 1);
            for j in 0..leafs.len() {
                if i == j {
                    continue;
                }
                let len = g[leafs[i]][leafs[j]] as f64;
                let ans = res[j];
                ways.push(Way { len, ans });
            }
            res[i] = best(&ways);
        }
    }
    let mut ways = vec![];
    for i in 0..leafs.len() {
        if leafs[i] == root {
            continue;
        }
        let len = g[root][leafs[i]] as f64;
        let ans = res[i];
        ways.push(Way { len, ans });
    }
    best(&ways)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut g = Array2D::new(std::i32::MAX / 10, n, n);
    for i in 0..n {
        g[i][i] = 0;
    }
    for _ in 0..n - 1 {
        let u = input.usize() - 1;
        let v = input.usize() - 1;
        g[u][v] = 1;
        g[v][u] = 1;
    }
    let root = input.usize() - 1;
    let ans = solve_case(g, root);
    out_line!(ans);
}

fn stress() -> bool {
    let n = 100;
    let mut rnd = Random::new(798778);
    let mut g = Array2D::new(std::i32::MAX / 10, n, n);
    for i in 0..n {
        for j in 0..n {
            g[i][j] = rnd.gen(1..100);
        }
    }
    for i in 0..n {
        g[i][i] = 0;
    }
    let root = rnd.gen(0..n);
    let ans = solve_case(g, root);
    println!("{}", ans);
    true
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
