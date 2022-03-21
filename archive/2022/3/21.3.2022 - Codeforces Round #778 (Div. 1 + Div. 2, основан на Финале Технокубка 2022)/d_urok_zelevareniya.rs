//{"name":"D. Урок зельеварения","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4\n3 2 3 4\n1 2 4 3\n1 4 2 4\n8\n5 4 2 3\n6 4 5 4\n1 3 5 2\n6 8 2 1\n3 5 3 4\n3 2 2 5\n6 7 4 3\n17\n8 7 4 16\n9 17 4 5\n5 14 13 12\n11 1 17 14\n6 13 8 9\n2 11 3 11\n4 17 7 2\n17 16 8 6\n15 5 1 14\n16 7 1 10\n12 17 13 10\n11 16 7 2\n10 11 6 4\n13 17 14 6\n3 11 15 8\n15 6 12 8\n","output":"69\n359\n573672453\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUrokZelevareniya"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::edge_with_info::EdgeWithInfo;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::math::primes::{factorize, gen_largest_prime_table};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Mult {
    mul: usize,
    div: usize,
}

type Edge = EdgeWithInfo<Mult>;
type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut graph = SimpleGraphT::new(n);
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let x = input.usize();
        let y = input.usize();
        graph.add_complex_edge(fr, Edge::new(to, Mult { mul: y, div: x }));
        graph.add_complex_edge(to, Edge::new(fr, Mult { mul: x, div: y }));
    }
    let largest_prime_table = gen_largest_prime_table(n);

    let mut cur_state = vec![0; n + 1];
    let mut smallest_state = vec![0; n + 1];

    let mut res = Mod::ZERO;

    RecursiveFunction3::new(|f, v: usize, p: usize, cur_value: Mod| {
        res += cur_value;
        for e in graph.adj(v) {
            let mut next_value = cur_value;
            let to = e.to();
            if to == p {
                continue;
            }

            {
                let mut mul = e.info.mul;
                next_value *= Mod::new(mul);
                for prime in factorize(&largest_prime_table, mul) {
                    for _ in 0..prime.power {
                        cur_state[prime.value] += 1;
                        mul /= prime.value;
                    }
                }
            }
            {
                let mut mul = e.info.div;
                next_value /= Mod::new(mul);
                while mul != 1 {
                    let prime = largest_prime_table[mul];
                    cur_state[prime] -= 1;
                    smallest_state[prime].update_min(cur_state[prime]);
                    mul /= prime;
                }
            }
            f.call(to, v, next_value);
            {
                let mut mul = e.info.mul;
                while mul != 1 {
                    let prime = largest_prime_table[mul];
                    cur_state[prime] -= 1;
                    mul /= prime;
                }
            }
            {
                let mut mul = e.info.div;
                while mul != 1 {
                    let prime = largest_prime_table[mul];
                    cur_state[prime] += 1;
                    mul /= prime;
                }
            }
        }
    })
    .call(0, 0, Mod::ONE);

    let mut mult_all = Mod::ONE;
    for val in 1..=n {
        for _ in 0..-smallest_state[val] {
            mult_all *= Mod::new(val);
        }
    }

    out_line!(res * mult_all);
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
}
//END MAIN
