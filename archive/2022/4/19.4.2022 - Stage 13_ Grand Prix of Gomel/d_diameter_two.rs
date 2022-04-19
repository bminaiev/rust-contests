//{"name":"D. Diameter Two","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 0\n5 2\n6 6\n","output":"3\n1 2\n1 3\n2 3\n5\n1 3\n2 3\n3 4\n3 5\n4 5\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDiameterTwo"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::{output, set_global_output_to_file, Writable};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn score(g: &Array2D<bool>) -> i32 {
    let n = g.len();
    let mut res = 0;
    for i in 0..n {
        for j in 1..n {
            let mut ok = g[i][j];
            for k in 0..n {
                if g[i][k] && g[k][j] {
                    ok = true;
                    break;
                }
            }
            if !ok {
                res += 1;
            }
        }
    }
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {
            if g[i][j] {
                cnt += 1;
            }
        }
        if cnt < 2 {
            res += 100;
        }
    }
    res
}

fn exist_sol(n: usize, mut m: usize, time: f64) -> Option<Array2D<bool>> {
    let mut g = Array2D::new(false, n, n);
    for i in 0..n {
        let j = (i + 1) % n;
        g[i][j] = true;
        g[j][i] = true;
        m -= 1;
    }
    for i in 0..n {
        for j in i + 1..n {
            if m > 0 && !g[i][j] {
                g[i][j] = true;
                g[j][i] = true;
                m -= 1;
            }
        }
    }
    let mut rnd = Random::new(787788);
    let mut sa = SimulatedAnnealing::new(time, SearchFor::MinimumScore, 10.0, 0.1);
    sa.set_silent(true);
    let mut prev_score = score(&g);
    if prev_score == 0 {
        return Some(g);
    }
    while sa.should_continue() {
        let mut i1 = 0;
        let mut j1 = 0;
        while i1 == j1 || g[i1][j1] {
            i1 = rnd.gen_in_range(0..n);
            j1 = rnd.gen_in_range(0..n);
        }
        let mut i2 = 0;
        let mut j2 = 0;
        while i2 == j2 || !g[i2][j2] {
            i2 = rnd.gen_in_range(0..n);
            j2 = rnd.gen_in_range(0..n);
        }
        g[i1][j1] = !g[i1][j1];
        g[i2][j2] = !g[i2][j2];
        g[j1][i1] = !g[j1][i1];
        g[j2][i2] = !g[j2][i2];
        let new_score = score(&g);
        if sa.should_go(prev_score, new_score) {
            prev_score = new_score;
        } else {
            g[i1][j1] = !g[i1][j1];
            g[i2][j2] = !g[i2][j2];
            g[j1][i1] = !g[j1][i1];
            g[j2][i2] = !g[j2][i2];
        }
        if prev_score == 0 {
            return Some(g);
        }
    }
    return None;
}

fn save_precalc(a: &[Vec<(usize, usize)>]) {
    set_global_output_to_file("precalc.txt");
    out_line!("let mut a = vec![];");
    for line in a.iter() {
        out!("a.push(vec![");
        for &(x, y) in line.iter() {
            out!(format!("({}, {}), ", x, y));
        }
        out_line!("]);");
    }
    output().flush();
}

fn expected(n: usize) -> usize {
    if n <= 5 {
        return n;
    }
    if n == 6 {
        return 7;
    }
    let mut cur_n = 7;
    let mut cur_m = 9;
    while cur_n != n {
        if cur_n % 2 == 1 {
            cur_m += 2;
        } else {
            cur_m += 1;
        }
        cur_n += 1;
    }
    cur_m
}

fn stress() {
    let mut res = vec![];
    for n in 3..=50 {
        dbg!(n);
        let mut time = 0.1;
        let mut m = expected(n);
        loop {
            if let Some(sol) = exist_sol(n, m, time) {
                let mut edges = vec![];
                for i in 0..n {
                    for j in i + 1..n {
                        if sol[i][j] {
                            edges.push((i, j));
                        }
                    }
                }
                res.push(edges);
                break;
            } else {
                time *= 2.0;
                // m += 1;
                // unreachable!();
            }
        }
        dbg!(n, m, time);
    }
    save_precalc(&res);
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    if n == k || n == k + 2 {
        out_line!(-1);
        return;
    }
    let mut edges = if k >= 1 {
        let mut res = vec![];
        for i in 0..k {
            res.push((i, k));
        }
        for i in k + 1..n {
            res.push((k, i));
        }
        for i in (k + 1..n).step_by(2) {
            if i + 1 < n {
                res.push((i, i + 1));
            } else {
                res.push((i - 1, i));
            }
        }
        res
    } else {
        let mut a: Vec<Vec<(usize, usize)>> = vec![];

        // run streess to get it.

        a[n - 3].clone()
    };
    out_line!(edges.len());
    for &(x, y) in edges.iter() {
        out_line!(x + 1, y + 1);
    }
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
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
