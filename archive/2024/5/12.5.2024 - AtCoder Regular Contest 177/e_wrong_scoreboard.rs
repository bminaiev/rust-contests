//{"name":"E - Wrong Scoreboard","group":"AtCoder - AtCoder Regular Contest 177","url":"https://atcoder.jp/contests/arc177/tasks/arc177_e","interactive":false,"timeLimit":9000,"tests":[{"input":"6\n4\n0 1 1 0 0\n1 0 0 1 0\n1 1 0 1 0\n1 0 1 0 0\n1 2 3 4\n8\n0 1 0 0 0\n1 1 0 1 0\n0 1 1 0 1\n1 0 0 0 0\n1 1 0 1 0\n0 1 0 0 0\n0 0 0 1 0\n0 1 1 1 1\n7 4 2 8 3 6 5 1\n6\n1 1 0 0 0\n0 0 1 0 0\n1 1 1 0 0\n0 0 0 1 0\n1 1 1 1 0\n0 0 0 0 1\n1 2 3 4 5 6\n6\n1 1 0 0 0\n0 0 1 0 0\n1 1 1 0 0\n0 0 0 1 0\n1 1 1 1 0\n0 0 0 0 1\n6 5 4 3 2 1\n20\n0 0 0 0 1\n0 0 1 0 0\n1 1 0 0 1\n1 0 1 0 1\n0 0 0 1 1\n0 0 1 1 1\n1 1 1 1 0\n1 1 0 1 0\n0 0 1 1 0\n1 0 1 0 0\n0 1 0 0 1\n0 1 1 1 1\n1 1 1 1 1\n0 1 0 1 0\n1 0 0 0 1\n1 1 1 0 0\n0 1 1 1 0\n0 0 0 1 0\n1 1 1 0 1\n1 1 0 1 1\n7 18 3 5 19 11 13 2 4 10 14 15 17 6 16 9 8 12 1 20\n15\n0 0 1 1 0\n0 0 0 1 0\n0 0 0 0 1\n0 0 1 1 1\n1 1 0 0 1\n0 1 1 1 0\n1 1 1 1 1\n0 1 1 0 1\n1 1 0 1 0\n1 0 0 1 1\n1 0 1 0 0\n1 1 0 1 1\n0 1 0 1 0\n1 1 0 0 0\n0 1 0 0 1\n1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\n","output":"6\n0\n26\n0\n1054\n428\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWrongScoreboard"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Subset {
    score: i32,
    mask: usize,
}

fn first_is_better(p1: &[Vec<usize>], p2: &[Vec<usize>]) -> bool {
    let mut i = 0;
    for masks in p1.iter() {
        let mut here_masks = vec![];
        while here_masks.len() < masks.len() {
            here_masks.extend(p2[i].clone());
            i += 1;
        }
        here_masks.sort();
        if here_masks != *masks {
            return false;
        }
    }
    true
}

fn stress() -> Vec<Vec<Vec<usize>>> {
    let mut seen = std::collections::HashSet::new();
    let mut seen_vec = vec![];
    for it in 1.. {
        let mut rnd = Random::new(it);
        const MX: i32 = 50;
        let max = rnd.gen(1..MX);
        let n = 5;
        let mut costs = gen_vec(n, |_| rnd.gen(1..max + 1));
        costs.sort();
        let mut subsets = vec![];
        for mask in 1..(1 << n) {
            let mut score = 0;
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    score += costs[i];
                }
            }
            subsets.push(Subset { score, mask });
        }
        subsets.sort();
        let mut partitions = vec![];
        let mut i = 0;
        while i != subsets.len() {
            let mut j = i;
            while j != subsets.len() && subsets[j].score == subsets[i].score {
                j += 1;
            }
            partitions.push(subsets[i..j].iter().map(|s| s.mask).collect::<Vec<_>>());
            i = j;
        }
        if seen.insert(partitions.clone()) {
            partitions.reverse();
            seen_vec.push(partitions);
            if seen_vec.len() == 4672 {
                seen_vec.sort_by_key(|p| p.len());
                let mut res: Vec<Vec<Vec<usize>>> = vec![];
                for partition in seen_vec.into_iter() {
                    let mut exist_better = false;
                    for exist in res.iter() {
                        if first_is_better(exist, &partition) {
                            exist_better = true;
                            break;
                        }
                    }
                    if !exist_better {
                        res.push(partition);
                    }
                }
                // dbg!("Now size:", res.len());
                return res;
            }
            // dbg!(it, seen.len());
            // dbg!(&partitions);
        }
    }
    unreachable!()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let partitions = stress();
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut masks = vec![];
        const T: usize = 5;
        for i in 0..n {
            let mut mask = 0;
            for j in 0..T {
                if input.usize() == 1 {
                    mask |= 1 << j;
                }
            }
            masks.push(mask);
        }
        let positions = input.vec::<i64>(n).sub_from_all(1);
        let mut by_mask = vec![vec![]; 1 << T];
        for i in 0..n {
            by_mask[masks[i]].push(positions[i]);
        }
        for by_mask in by_mask.iter_mut() {
            by_mask.sort();
        }
        let mut res = i64::MAX;
        for partition in partitions.iter() {
            let mut start_from = 0;
            let mut cur_res = 0;
            for cur_masks in partition.iter() {
                let mut here_pos = Vec::<i64>::new();
                for &mask in cur_masks.iter() {
                    here_pos.extend(&by_mask[mask]);
                }
                here_pos.sort();
                for i in 0..here_pos.len() {
                    let real_pos = start_from + i as i64;
                    let delta = here_pos[i] - real_pos;
                    cur_res += delta * delta;
                }
                start_from += here_pos.len() as i64;
            }
            res = res.min(cur_res);
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e_wrong_scoreboard";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
