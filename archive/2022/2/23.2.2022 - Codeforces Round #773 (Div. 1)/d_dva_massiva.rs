//{"name":"D. Два массива","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4 2\n1 2 5\n4 3 1\n2 3 2\n4 5 3\n","output":"5\n"},{"input":"4 3\n1 2 3 5\n2 3 4 2\n3 4 5 3\n1 3 10 10\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDvaMassiva"}}}

use std::time::Instant;

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::id_map::IdMap;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Line {
    w: i32,
    a: Vec<i32>,
}

fn hash(a: &[i32]) -> i128 {
    let mult = 1234567893421i128;
    let mut res = 0i128;
    for &x in a.iter() {
        res = res.wrapping_mul(mult).wrapping_add(x as i128);
    }
    res
}

#[allow(unused)]
fn solve_test(mut lines: Vec<Line>, m: usize) -> i32 {
    lines.sort();

    let mut ids = IdMap::new();
    let mut positions = vec![];
    let mut lens: Vec<i32> = vec![];

    let mut all_check_ids = Vec::with_capacity(lines.len());

    for (line_no, line) in lines.iter().enumerate() {
        let mut check_ids = vec![];

        for mask in 1i32..(1 << m) {
            let mut cur = vec![];
            for i in 0..m {
                if ((1 << i) & mask) != 0 {
                    cur.push(line.a[i]);
                }
            }
            let cur = hash(&cur);
            let id = ids.get_or_add(&cur);
            while id >= positions.len() {
                positions.push(vec![]);
                lens.push(mask.count_ones() as i32 % 2i32);
            }
            positions[id].push(line_no as i32);

            check_ids.push(id as i32);
        }

        all_check_ids.push(check_ids);
    }
    let mut res = i32::MAX;

    // dbg!(ids.len());

    let mut counter = 0;
    let mut counter_logs = 0;
    for (it, line) in lines.iter().enumerate() {
        if line.w * 2 >= res {
            // break;
        }
        let check_ids = &all_check_ids[it];

        let first_ok_pair = binary_search_first_true(it + 1..lines.len(), |pos| -> bool {
            let mut cnt_bad = 0;

            for &id in check_ids.iter() {
                let id = id as usize;
                let sign = if lens[id] == 1 { 1 } else { -1 };
                counter += 1;
                counter_logs += positions[id].len().next_power_of_two().trailing_zeros() as i64 + 1;
                let count_bad = number_of_le_elements(&positions[id], pos as i32) as i32;
                cnt_bad += sign * count_bad;
            }

            cnt_bad < pos as i32 + 1
        });
        if first_ok_pair < lines.len() {
            res.update_min(lines[first_ok_pair].w + line.w);
        }
    }
    dbg!(counter);
    dbg!(counter_logs);
    res
}

#[derive(Debug)]
enum Positions {
    Vec(Vec<i32>),
    BitSet(BitSet),
}

fn solve_bitsets(mut lines: Vec<Line>, _m: usize) -> i32 {
    lines.sort();

    let mut ids = IdMap::new();

    let mut positions = vec![];

    for (idx, line) in lines.iter().enumerate() {
        for &value in line.a.iter() {
            let value_id = ids.get_or_add(&value);
            if value_id == positions.len() {
                positions.push(Positions::Vec(vec![]));
            }
            match &mut positions[value_id] {
                Positions::Vec(vec) => vec.push(idx as i32),
                Positions::BitSet(_) => panic!(),
            }
        }
    }

    // can use 400Mb of memory
    // 8bitsets = 1mb -> can store ~400 * 8 = 3K bitsets
    // 500K values in total, 3K-th bitset will have > 500K / 3K values = 200 elements

    const BIG_SIZE: usize = 200;

    for entry in positions.iter_mut() {
        if let Positions::Vec(vec) = entry {
            if vec.len() > BIG_SIZE {
                let mut bs = BitSet::new(lines.len());
                for &pos in vec.iter() {
                    bs.set(pos as usize, true);
                }
                *entry = Positions::BitSet(bs);
            }
        }
    }

    let mut res = i32::MAX;

    let mut bad = BitSet::new(lines.len());
    for (line_no, line) in lines.iter().enumerate() {
        bad.clear();
        for value_id in line.a.iter().map(|val| ids.get_exn(val)) {
            match &positions[value_id] {
                Positions::BitSet(bs) => {
                    bad |= bs;
                }
                Positions::Vec(vec) => {
                    for &pos in vec.iter() {
                        bad.set(pos as usize, true);
                    }
                }
            }
        }
        let first = bad.first_not_set(line_no + 1);
        if first != lines.len() {
            assert!(!bad.get(first));
            assert_ne!(first, line_no);
            res.update_min(line.w + lines[first].w);
        }
    }

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut lines = vec![];
    for _ in 0..n {
        let mut a = input.vec::<i32>(m);
        let w = input.i32();
        a.sort();
        let mut ok = true;
        for w in a.windows(2) {
            if w[0] == w[1] {
                ok = false;
            }
        }
        if ok {
            lines.push(Line { w, a });
        }
    }

    let res = solve_bitsets(lines, m);

    if res == i32::MAX {
        out_line!(-1);
    } else {
        out_line!(res);
    }
}

fn number_of_le_elements(a: &[i32], val: i32) -> usize {
    binary_search_first_true(0..a.len(), |pos| a[pos] > val)
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

fn stress() {
    for it in 1..2 {
        dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = 100_000; //rnd.gen_in_range(6..200);
        let m = 5;
        let mut lines = vec![];
        let max_a = 20; //rnd.gen_in_range(6..10000);
        for _ in 0..n {
            loop {
                let mut a = rnd.gen_vec(m, 1..max_a);
                a.sort();
                let mut ok = true;
                for w in a.windows(2) {
                    if w[0] == w[1] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                lines.push(Line {
                    w: rnd.gen_in_range(1..1_000_000_000),
                    a,
                });
                break;
            }
        }
        let res_bs = solve_bitsets(lines.clone(), m);
        // let res_old = solve_test(lines, m);
        // assert_eq!(res_bs, res_old);
        dbg!(res_bs);
    }
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    let start = Instant::now();
    stress();
    dbg!(start.elapsed().as_millis());
}
//END MAIN
