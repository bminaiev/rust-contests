//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::min_priority_queue::MinPriorityQueue;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let a = input.usize();
        let b = input.usize();
        let c = input.usize();
        let b_mask = b.next_power_of_two() - 1;
        let mut min_a = Array2D::new(usize::MAX, b + 1, b_mask + 1);
        let mut seen = Array2D::new(false, b + 1, b_mask + 1);
        let mut pq = MinPriorityQueue::new();
        min_a[a % b][a & b_mask] = a;
        pq.push(a);
        while let Some(cur_a) = pq.pop() {
            if seen[cur_a % b][cur_a & b_mask] {
                continue;
            }
            seen[cur_a % b][cur_a & b_mask] = true;
            {
                // + b
                let next_a = cur_a + b;
                let x = next_a % b;
                let y = next_a & b_mask;
                if next_a < min_a[x][y] {
                    // dbg!(cur_a, next_a, "+");

                    min_a[x][y] = next_a;
                    pq.push(next_a);
                }
            }
            {
                // ^ b_mask
                let next_a = cur_a ^ b;
                let x = next_a % b;
                let y = next_a & b_mask;
                if next_a < min_a[x][y] {
                    // dbg!(cur_a, next_a, "xor");
                    min_a[x][y] = next_a;
                    pq.push(next_a);
                }
            }
        }
        if min_a[c % b][c & b_mask] > c {
            out.println("NO");
        } else {
            out.println("YES");
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
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
