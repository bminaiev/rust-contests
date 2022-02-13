//{"name":"E. Лучшая пара","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 1\n6 3 6 7 3 3\n3 6\n2 0\n3 4\n7 4\n1 2 2 3 1 5 1\n1 5\n3 5\n1 3\n2 5\n","output":"40\n14\n15\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ELuchshayaPara"}}}

use std::collections::{BTreeSet, HashMap, HashSet};

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::peek_random::PeekRandom;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::ordered_pair::OrderedPair;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Iterator {
    result: i64,
    cnt1: usize,
    pos1: usize,
    cnt2: usize,
    pos2: usize,
}

impl Iterator {
    fn new(result: i64, cnt1: usize, pos1: usize, cnt2: usize, pos2: usize) -> Self {
        Self {
            result,
            cnt1,
            pos1,
            cnt2,
            pos2,
        }
    }
}

fn fast(a: &[i64], bad: &HashSet<OrderedPair<i64>>) -> i64 {
    let n = a.len();
    let mut cnt: HashMap<_, i64> = HashMap::new();
    for &x in a.iter() {
        *cnt.entry(x).or_default() += 1;
    }
    let mut by_cnt: Vec<Vec<i64>> = vec![vec![]; n + 1];
    for (key, val) in cnt.iter() {
        by_cnt[*val as usize].push(*key);
    }
    let mut exist_cnt = vec![];
    for cnt in 0..by_cnt.len() {
        if !by_cnt[cnt].is_empty() {
            exist_cnt.push(cnt);
            by_cnt[cnt].sort();
        }
    }

    let mut seen = BTreeSet::new();
    let mut add_iter = |by_cnt: &Vec<Vec<i64>>,
                        pq: &mut BTreeSet<Iterator>,
                        cnt1: usize,
                        pos1: usize,
                        cnt2: usize,
                        pos2: usize| {
        if cnt1 == cnt2 && pos1 == pos2 {
            return;
        }
        let result = (cnt1 as i64 + cnt2 as i64) * (by_cnt[cnt1][pos1] + by_cnt[cnt2][pos2]);
        let iter = Iterator::new(result, cnt1, pos1, cnt2, pos2);
        if !seen.contains(&iter) {
            seen.insert(iter);
            pq.insert(iter);
        }
    };

    let mut pq = BTreeSet::<Iterator>::new();
    for &cnt1 in exist_cnt.iter() {
        for &cnt2 in exist_cnt.iter() {
            if cnt1 > cnt2 {
                continue;
            }
            if cnt1 == cnt2 {
                let len = by_cnt[cnt1].len();
                if len > 1 {
                    add_iter(&by_cnt, &mut pq, cnt1, len - 2, cnt2, len - 1);
                }
            } else {
                let pos1 = by_cnt[cnt1].len() - 1;
                let pos2 = by_cnt[cnt2].len() - 1;
                add_iter(&by_cnt, &mut pq, cnt1, pos1, cnt2, pos2);
            }
        }
    }
    while !pq.is_empty() {
        let iter = pq.last_exn().clone();
        pq.remove(&iter);
        let pair = OrderedPair::new(by_cnt[iter.cnt1][iter.pos1], by_cnt[iter.cnt2][iter.pos2]);
        if !bad.contains(&pair) {
            return iter.result;
        }
        if iter.pos1 != 0 {
            add_iter(
                &by_cnt,
                &mut pq,
                iter.cnt1,
                iter.pos1 - 1,
                iter.cnt2,
                iter.pos2,
            );
        }
        if iter.pos2 != 0 {
            add_iter(
                &by_cnt,
                &mut pq,
                iter.cnt1,
                iter.pos1,
                iter.cnt2,
                iter.pos2 - 1,
            );
        }
    }
    return 0;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.read_vec::<i64>(n);
    let mut bad = HashSet::new();
    for _ in 0..m {
        let x = input.i64();
        let y = input.i64();
        bad.insert(OrderedPair::new(x, y));
    }
    let result = fast(&a, &bad);
    out_line!(result);
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

fn stupid(a: &[i64], fobidden_pairs: &HashSet<OrderedPair<i64>>) -> i64 {
    let mut res = 0;
    for &x in a.iter() {
        for &y in a.iter() {
            if x == y {
                continue;
            }
            if fobidden_pairs.contains(&OrderedPair::new(x, y)) {
                continue;
            }
            let cnt_x = a.iter().filter(|val| *val == &x).count() as i64;
            let cnt_y = a.iter().filter(|val| *val == &y).count() as i64;
            res.update_max((cnt_x + cnt_y) * (x + y));
        }
    }
    res
}

pub fn stress() {
    for test in 7.. {
        dbg!(test);
        let mut rnd = Random::new(787788 + test);
        let n = 1 + rnd.gen_in_range(1..10);
        let a = rnd.gen_vec(n, 1..10);
        let mut forbidden_pairs = HashSet::new();
        let m: usize = rnd.gen_in_range(0..10);
        for _ in 0..m {
            let x = *a.peek_random(&mut rnd).unwrap();
            let y = *a.peek_random(&mut rnd).unwrap();
            forbidden_pairs.insert(OrderedPair::new(x, y));
        }
        let slow = stupid(&a, &forbidden_pairs);
        let fast = fast(&a, &forbidden_pairs);
        if slow != fast {
            dbg!(a);
            dbg!(&forbidden_pairs);
            dbg!(fast);
        }
        assert_eq!(slow, fast);
    }
}

fn main() {
    // stress();

    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
