//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_builder::GraphBuilder;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::rand::Random;

#[derive(Clone, Copy)]
struct Bucket {
    from: i64,
    to: i64,
    value: f64,
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.i64();
    let buckets = gen_buckets();
    if tc < 0 {
        // first turn
        let tc = -tc;
        for _ in 0..tc {
            let m = input.usize();
            out.println(12 * m);
            for _ in 0..m {
                let x = input.i64();
                let b_id = binary_search_first_true(0..buckets.len(), |i| buckets[i].to > x);
                assert!(b_id < buckets.len());
                assert!(buckets[b_id].from <= x && x < buckets[b_id].to);
                for i in 0..12 {
                    if ((1 << i) & b_id) != 0 {
                        out.print("1");
                    } else {
                        out.print("0");
                    }
                }
            }
            out.println("");
        }
    } else {
        for _ in 0..tc {
            let n = input.usize();
            let m = input.usize();
            let q = input.usize();
            let b = input.usize();
            assert_eq!(b, 12 * m);
            let s = input.string();
            let mut weights = vec![];
            for i in 0..m {
                let mut b_id = 0;
                for j in 0..12 {
                    if s[i * 12 + j] == b'1' {
                        b_id |= 1 << j;
                    } else {
                        assert_eq!(s[i * 12 + j], b'0');
                    }
                }
                weights.push(buckets[b_id].value);
            }
            let mut g = GraphBuilder::new(n);
            for id in 0..m {
                let fr = input.usize() - 1;
                let to = input.usize() - 1;
                g.add_edge(fr, WeightedEdge::new(to, OrdF64(weights[id])));
                g.add_edge(to, WeightedEdge::new(fr, OrdF64(weights[id])));
            }
            let g = g.build();
            for _ in 0..q {
                let s = input.usize() - 1;
                let t = input.usize() - 1;
                let d = dijkstra(&g, s)[t].dist;
                if d.0 > 1e18 {
                    out.println(-1);
                } else {
                    out.println(format!("{:.15}", d.0));
                }
            }
        }
    }
}

const EPS: f64 = 1e-6;
const Z: f64 = 1.001;
const FROM: f64 = 1.0 / Z + EPS;
const TO: f64 = Z - EPS;

const MAX_W: i64 = 1_000_000;

fn gen_buckets() -> Vec<Bucket> {
    let mut from = 1;
    let mut buckets = vec![];
    while from <= MAX_W {
        let value = (from as f64) * TO;
        let till = (value / FROM) as i64;
        assert!((till as f64) * FROM <= value);
        buckets.push(Bucket {
            from,
            to: till + 1,
            value,
        });
        from = till + 1;
    }
    // dbg!(buckets.len());
    assert!(buckets.len() <= 4096);
    buckets
}

fn stress() {
    let buckets = gen_buckets();
    for x in 1..=1_000_000 {
        let b_id = binary_search_first_true(0..buckets.len(), |i| buckets[i].to > x);
        assert!(b_id < buckets.len());
        let ok_from = (x as f64) * (0.999 - EPS / 2.0);
        let ok_to = (x as f64) * (1.001 + EPS / 2.0);
        let value = buckets[b_id].value;
        assert!(ok_from <= value && value <= ok_to, "x={}", x);
    }
    for x in 1..=1000 {
        for y in 1..=1000 {
            let b_id_1 = binary_search_first_true(0..buckets.len(), |i| buckets[i].to > x);
            let b_id_2 = binary_search_first_true(0..buckets.len(), |i| buckets[i].to > y);
            let value = buckets[b_id_1].value + buckets[b_id_2].value;
            let ok_from = (x as f64 + y as f64) * 0.999;
            let ok_to = (x as f64 + y as f64) * 1.001;
            assert!(ok_from <= value && value <= ok_to, "x={}, y={}", x, y);
        }
    }
    let mut rnd = Random::new(123);
    for _it in 0.. {
        dbg!(_it);
        let cnt = rnd.gen_range(1..100000);
        let mut real_sum = 0;
        let mut value_sum = 0.0;
        for _ in 0..cnt {
            let x = rnd.gen_range(1..1_000_000);
            let b_id_1 = binary_search_first_true(0..buckets.len(), |i| buckets[i].to > x);
            value_sum += buckets[b_id_1].value;
            real_sum += x;
        }
        let ok_from = (real_sum as f64) * 0.999;
        let ok_to = (real_sum as f64) * 1.001;
        dbg!(real_sum, value_sum);
        assert!(
            ok_from <= value_sum && value_sum <= ok_to,
            "real_sum={}, value_sum={}",
            real_sum,
            value_sum
        );
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
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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
