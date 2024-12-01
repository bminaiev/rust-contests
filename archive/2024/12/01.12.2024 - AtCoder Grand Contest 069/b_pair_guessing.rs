//{"name":"B - Pair Guessing","group":"AtCoder - AtCoder Grand Contest 069","url":"https://atcoder.jp/contests/agc069/tasks/agc069_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n01\n11\n2\n11\n11\n10\n0101011110\n1100100001\n1101100000\n0111101010\n1000011001\n1110101010\n1110110100\n1110000110\n0000001011\n1001111100\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPairGuessing"}}}

use std::collections::{HashMap, HashSet};
use std::i64;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::flows::dinic::FlowDinic;
use algo_lib::flows::hungarian_algorithm::hungarian_algorithm;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

fn can_win(n: usize) -> Vec<bool> {
    let nn = n * n;
    // dp[move_moves][who_can_be] -> can_win
    let mut dp = Array2D::new(false, n + 1, 1 << nn);
    for mask in 0usize..1 << nn {
        if mask.count_ones() <= 1 {
            dp[0][mask] = true;
        }
    }
    for more_moves in 1..=n {
        for mask in 0..1 << nn {
            for i in 0..nn {
                let mut mask_yes = 0;
                let mut mask_no = 0;
                for j in 0..nn {
                    let bit = (1 << j) & mask;
                    if i / n == j / n || (i % n == j % n) {
                        mask_yes |= bit;
                    } else {
                        mask_no |= bit;
                    }
                }
                if dp[more_moves - 1][mask_yes] && dp[more_moves - 1][mask_no] {
                    if more_moves == n && mask == (1 << nn) - 2 {
                        dbg!(i, mask_yes, mask_no);
                    }
                    if more_moves == 2 && mask == 432 {
                        dbg!(i, mask_yes, mask_no);
                    }
                    dp[more_moves][mask] = true;
                    break;
                }
            }
        }
    }
    dp[n].to_vec()
}

fn stress2() {
    for n in 3..=3 {
        println!("n = {}", n);
        let w = can_win(n);
        for mask in 0..(1 << (n * n)) {
            if !w[mask] {
                for i in 0..n {
                    let mut s = vec![];
                    for j in 0..n {
                        s.push(if (mask & (1 << (n * i + j))) != 0 {
                            b'1'
                        } else {
                            b'0'
                        });
                    }
                    println!("{}", String::from_utf8(s).unwrap());
                }
                println!();
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    a: Array2D<bool>,
    more_moves: usize,
}

impl State {
    fn new(a: Array2D<bool>, more_moves: usize) -> Self {
        let n = a.len();
        let mut rows_perm: Vec<_> = (0..n).collect();
        rows_perm.sort_by_key(|&i| a[i].iter().map(|&x| x as i32).sum::<i32>());
        let mut cols_perm: Vec<_> = (0..n).collect();
        cols_perm.sort_by_key(|&j| (0..n).map(|i| a[i][j] as i32).sum::<i32>());
        let mut new_a = Array2D::new(false, n, n);
        for i in 0..n {
            for j in 0..n {
                new_a[i][j] = a[rows_perm[i]][cols_perm[j]];
            }
        }
        Self {
            a: new_a,
            more_moves,
        }
    }
}

fn stress() {
    let mut hm = HashMap::<State, bool>::new();
    let mut can_win = RecursiveFunction::new(|f, s: State| -> bool {
        if let Some(&res) = hm.get(&s) {
            return res;
        }
        let n = s.a.len();
        let mut res = false;
        if s.more_moves == 0 {
            let mut cnt_ones = 0;
            for i in 0..n {
                for j in 0..n {
                    if s.a[i][j] {
                        cnt_ones += 1;
                    }
                }
            }
            res = cnt_ones <= 1;
        } else {
            for i in 0..n {
                for j in 0..n {
                    let mut can_win_here = true;
                    for &ok in [false, true].iter() {
                        let mut new_a = Array2D::new(false, n, n);
                        for ii in 0..n {
                            for jj in 0..n {
                                let expected_ok = i == ii || j == jj;
                                if ok == expected_ok {
                                    new_a[ii][jj] = s.a[ii][jj];
                                }
                            }
                        }
                        let new_s = State::new(new_a, s.more_moves - 1);
                        if !f.call(new_s) {
                            can_win_here = false;
                            break;
                        }
                    }
                    if can_win_here {
                        res = true;
                        break;
                    }
                }
                if res {
                    break;
                }
            }
        }
        hm.insert(s, res);
        res
    });
    let n = 6;
    let sub_n = 4;
    // let mut seen = HashSet::new();
    for mask in 0usize..(1 << (sub_n * sub_n)) {
        let cnt_zeros = sub_n * sub_n - mask.count_ones() as usize;
        // if cnt_zeros != 7 {
        //     continue;
        // }
        let mut field = Array2D::new(true, n, n);
        for i in 0..sub_n {
            for j in 0..sub_n {
                field[i][j] = (mask & (1 << (sub_n * i + j))) != 0;
            }
        }
        let num_rows_with_zeros = (0..n).filter(|i| field[*i].iter().any(|&x| !x)).count();
        let num_cols_with_zeros = (0..n).filter(|j| (0..n).any(|i| !field[i][*j])).count();
        let num_max = num_rows_with_zeros.max(num_cols_with_zeros);
        let num_min = num_rows_with_zeros.min(num_cols_with_zeros);
        let s = State::new(field.clone(), n);
        let res = can_win.call(s);
        let fast_can_win = fast_solve_can_win(&field);
        if res != fast_can_win {
            dbg!(res, fast_can_win);
            for i in 0..n {
                for j in 0..n {
                    print!("{}", if field[i][j] { '1' } else { '0' });
                }
                println!();
            }
            println!();
        }
        assert!(res == fast_can_win);
        // if !res {
        //     if !seen.insert(field.clone()) {
        //         continue;
        //     }
        //     if num_min <= 3 {
        //         continue;
        //     }
        //     for i in 0..n {
        //         for j in 0..n {
        //             print!("{}", if field[i][j] { '1' } else { '0' });
        //         }
        //         println!();
        //     }
        //     println!();
        // } else {
        //     if (num_min <= 3) {
        //         for i in 0..n {
        //             for j in 0..n {
        //                 print!("{}", if field[i][j] { '1' } else { '0' });
        //             }
        //             println!();
        //         }
        //         println!();
        //         dbg!("WIN");
        //     }
        //     // unreachable!()
        // }
    }
}

fn fast_solve_can_win(a: &Array2D<bool>) -> bool {
    let mut cnt_zeros = 0;
    for i in 0..a.rows() {
        for j in 0..a.cols() {
            if !a[i][j] {
                cnt_zeros += 1;
            }
        }
    }
    let n = a.len();
    // const INF: i64 = i64::MAX / 1000;
    // let mut b = Array2D::new(INF, n, n);
    // for i in 0..n {
    //     for j in 0..n {
    //         if !a[i][j] {
    //             b[i][j] = 0;
    //         }
    //     }
    // }
    // let hg = hungarian_algorithm(&b).unwrap();
    if n == 1 {
        true
    } else if n == 2 || n == 3 {
        cnt_zeros > 0
    } else {
        let mut f = FlowDinic::new(1 + n + n + n * n + 1);
        for i in 0..n {
            f.add_edge(0, 1 + i, 1);
            f.add_edge(0, 1 + n + i, 1);
        }
        for i in 0..n {
            for j in 0..n {
                if !a[i][j] {
                    let v = 1 + n + n + n * i + j;
                    f.add_edge(1 + i, v, 1);
                    f.add_edge(1 + n + j, v, 1);
                    f.add_edge(v, f.n - 1, 1);
                }
            }
        }
        f.find_flow() >= n as i64 - 2
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = Array2D::new(false, n, n);
        for i in 0..n {
            let s = input.string();
            for j in 0..n {
                a[i][j] = s[j] == b'1';
            }
        }
        out.println(if fast_solve_can_win(&a) { "Yes" } else { "No" });
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_pair_guessing";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
