use std::collections::BTreeSet;
use std::{todo, unreachable};

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

struct Answer {
    res: usize,
    answer: Vec<u8>,
}

fn gen_random_str(s: &[u8], rnd: &mut Random, len: usize) -> Vec<u8> {
    let mut seen_chars: BTreeSet<u8> = BTreeSet::new();
    for i in 0..s.len() {
        seen_chars.insert(s[i]);
    }
    let seen_chars: Vec<u8> = seen_chars.into_iter().collect();
    let mut answer = vec![b'0'; len];
    for i in 0..len {
        answer[i] = seen_chars[rnd.gen_range(0..seen_chars.len())];
    }
    answer
}

fn solve_case(s: &[u8]) -> Answer {
    let n = s.len();
    let mut dsu = Dsu::new(n);
    let mut s2 = s.to_vec();
    s2.extend(s.to_vec());
    for i in 0..n {
        let mut j = i + n - 1;
        while s2[j] != s2[i] {
            j -= 1;
        }
        for k in j + 1..i + n {
            let k2 = k % n;
            let j2 = j % n;
            dsu.unite(k2, j2);
        }
    }
    let res = dsu.num_components();

    let mut rnd = Random::new(234234);

    const MX: usize = 1_000;
    let answer = gen_random_str(s, &mut rnd, MX);
    Answer { res, answer }
}

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    c: usize,
}

fn solve_case_real(s: &[u8]) -> Answer {
    let n = s.len();
    let mut next = Array2D::new(usize::MAX, 26, n);
    for i in 0..26 {
        for j in 0..n {
            let need_char = (b'a' + i as u8) as u8;
            let mut k = j + 1;
            while s[k % n] != need_char && k < j + n + 5 {
                k += 1;
            }
            if k > j + n {
                next[i][j] = usize::MAX;
            } else {
                next[i][j] = k % n;
            }
        }
    }
    let node_id = |i: usize, j: usize| -> usize { i * n + j };
    let mut g_rev = vec![vec![]; n * n];
    for i in 0..n {
        for j in 0..n {
            for c in 0..26 {
                let next_pos_i = next[c][i];
                let next_pos_j = next[c][j];
                if next_pos_i != usize::MAX && next_pos_j != usize::MAX {
                    let next_node = node_id(next_pos_i, next_pos_j);
                    g_rev[next_node].push(Edge {
                        from: node_id(i, j),
                        to: next_node,
                        c,
                    });
                }
            }
        }
    }
    let mut first_move = vec![(usize::MAX, usize::MAX); n * n];
    let mut seen = vec![false; n * n];
    let mut queue = vec![];
    for i in 0..n {
        queue.push(node_id(i, i));
        seen[node_id(i, i)] = true;
    }
    while let Some(cur) = queue.pop() {
        for e in g_rev[cur].iter() {
            if !seen[e.from] {
                seen[e.from] = true;
                first_move[e.from] = (e.c, cur);
                queue.push(e.from);
            }
        }
    }
    let mut positions = vec![0; n];
    for i in 0..n {
        positions[i] = i;
    }
    let mut answer = vec![];
    // dbg!(n);
    loop {
        let mut ch = false;
        let mut new_moves = vec![];
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let mut cur_node = node_id(positions[i], positions[j]);
                if first_move[cur_node].0 != usize::MAX {
                    while first_move[cur_node].1 != usize::MAX {
                        let (c, next_node) = first_move[cur_node];
                        new_moves.push(c);
                        cur_node = next_node;
                    }

                    ch = true;
                    break;
                }
            }
            if ch {
                break;
            }
        }

        if !ch {
            break;
        }
        let prev_len = positions.len();
        // dbg!(positions);
        for c in new_moves {
            // dbg!(c);
            for i in 0..positions.len() {
                positions[i] = next[c as usize][positions[i]];
            }
            // dbg!(positions);
            answer.push(b'a' + c as u8);
        }
        positions.sort();
        positions.dedup();
        assert!(positions.len() < prev_len);
    }
    Answer {
        res: positions.len(),
        answer,
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let s = input.string();
    let Answer { res, answer } = solve_case_real(&s);
    assert!(answer.len() <= 1_000_000);
    out.println(res);
    out.println(answer.len());
    out.println(String::from_utf8(answer).unwrap());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn stress2() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(123 + it);
        let n = rnd.gen_range(1..10);
        let max_char = b'a' + rnd.gen_range(1..5);
        let s = rnd.gen_vec(n, b'a'..max_char);
        let Answer { res, answer } = solve_case(&s);
        dbg!(n, res, answer.len());
        let check_res = check(&s, &answer);
        if res != check_res {
            let s_str = String::from_utf8(s.clone()).unwrap();
            dbg!(s_str, res, check_res);
            unreachable!();
        }
    }
}

fn stress123() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(123 + it);
        let n = rnd.gen_range(1..100);
        let max_char = b'a' + rnd.gen_range(1..5);
        let s = rnd.gen_vec(n, b'a'..max_char);
        const LEN: usize = 1_000;
        let ans1 = gen_random_str(&s, &mut rnd, LEN);
        let ans2 = gen_random_str(&s, &mut rnd, LEN);
        let res1 = check(&s, &ans1);
        let Answer {
            res: res2,
            answer: ans2,
        } = solve_case_real(&s);
        let res3 = check(&s, &ans2);
        if res1 < res2 || res2 != res3 {
            dbg!(res1, res2);
            unreachable!();
        }
    }
}

fn check(s: &[u8], answer: &[u8]) -> usize {
    let n = s.len();
    let mut next = Array2D::new(0, 26, n);
    for i in 0..26 {
        for j in 0..n {
            let need_char = (b'a' + i as u8) as u8;
            let mut k = j + 1;
            while s[k % n] != need_char && k < j + n {
                k += 1;
            }
            next[i][j] = k % n;
        }
    }
    let mut positions = vec![0; n];
    for i in 0..n {
        positions[i] = i;
    }
    for c in answer {
        let c = *c - b'a';
        for i in 0..positions.len() {
            positions[i] = next[c as usize][positions[i]];
        }
        positions.sort();
        positions.dedup();
    }
    positions.len()
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_follow_the_letters";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
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
