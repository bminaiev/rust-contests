#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {}

#[derive(Clone, Copy, Debug)]
struct Node {
    q: i64,
    x: i64,
}

fn dist(a: Node, b: Node) -> f64 {
    let dx = (a.x - b.x) as f64;
    dx * dx / a.q as f64 / b.q as f64
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);

        const MAX_X: i64 = 10;

        let n = rnd.gen_range(2..5);
        let mut a = vec![];
        for _ in 0..n {
            a.push(Node {
                q: rnd.gen_range(1..MAX_X),
                x: rnd.gen_range(1..MAX_X),
            });
        }
        a.sort_by_key(|p| p.x);
        let mut ok = true;
        for i in 0..n - 1 {
            if a[i].x == a[i + 1].x {
                ok = false;
            }
        }
        if !ok {
            continue;
        }
        check(&a);
    }
}

fn check(a: &[Node]) {
    let mut max_dist = 0.0;
    let n = a.len();
    let mut best_id = 0;
    let mut best_id2 = 0;
    for i in 0..n {
        for j in i + 1..n {
            let d = dist(a[i], a[j]);
            if d > max_dist {
                max_dist = d;
                best_id = i;
                best_id2 = j;
            }
        }
    }
    const EPS: f64 = 1e-7;
    for i in 0..n {
        let d1 = dist(a[best_id], a[i]);
        let d2 = dist(a[best_id2], a[i]);
        for j in 0..n {
            let d = dist(a[i], a[j]);
            if d > d1 + EPS && d > d2 + EPS {
                dbg!(d, d1, d2);
                dbg!(a[best_id], a[best_id2], a[i], a[j]);
                for aa in a {
                    dbg!(aa);
                }
                panic!();
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "h";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
