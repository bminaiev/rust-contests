//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::bit_set::BitSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn gauss(a: &mut [BitSet]) -> bool {
    let n = a.len();
    let mut row_from = 0;
    for i in 0..n {
        let mut use_row = n;
        for j in row_from..n {
            if a[j].get(i) {
                use_row = j;
                break;
            }
        }
        if use_row == n {
            continue;
        }
        a.swap(row_from, use_row);
        for j in 0..n {
            if j != row_from && a[j].get(i) {
                a[j] ^= &a[row_from].clone();
            }
        }
        row_from += 1;
    }
    true
}

fn solve_gauss(s: &[Vec<u8>]) -> Option<Vec<Vec<bool>>> {
    let n = s.len();
    let m = s[0].len();
    let mut click = Array2D::new(BitSet::new(m + 1), n, m);
    for j in 0..m {
        click[0][j].set(j, true);
    }
    for row in 0..n - 1 {
        for col in 0..m {
            let mut my_value = BitSet::new(m + 1);
            if s[row][col] == b'1' {
                my_value.set(m, true);
            }
            if row > 0 {
                my_value ^= &click[row - 1][col];
            }
            if col > 0 {
                my_value ^= &click[row][col - 1];
            }
            if col + 1 < m {
                my_value ^= &click[row][col + 1];
            }
            my_value ^= &click[row][col];
            click[row + 1][col] = my_value;
        }
    }
    // m equations
    let mut a = vec![BitSet::new(m + 1); m];
    for col in 0..m {
        let mut my_value = BitSet::new(m + 1);
        if s[n - 1][col] == b'1' {
            my_value.set(m, true);
        }
        if n > 1 {
            my_value ^= &click[n - 2][col];
        }
        if col > 0 {
            my_value ^= &click[n - 1][col - 1];
        }
        if col + 1 < m {
            my_value ^= &click[n - 1][col + 1];
        }
        my_value ^= &click[n - 1][col];
        a[col] = my_value;
    }
    if !gauss(&mut a) {
        return None;
    }
    let mut vars_values = vec![false; m];
    let mut var_id = 0;
    for i in 0..m {
        while var_id < m && !a[var_id].get(i) {
            var_id += 1;
        }
        if var_id == m {
            break;
        }
        vars_values[var_id] = a[i].get(m);
        // dbg!(i, var_id, vars_values[var_id]);
        var_id += 1;
    }
    let mut ans = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut res = click[i][j].get(m);
            for k in 0..m {
                if click[i][j].get(k) && vars_values[k] {
                    res = !res;
                }
            }
            ans[i][j] = res;
        }
    }
    let mut final_board = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if ans[i][j] {
                if i > 0 {
                    final_board[i - 1][j] = !final_board[i - 1][j];
                }
                if j > 0 {
                    final_board[i][j - 1] = !final_board[i][j - 1];
                }
                if i + 1 < n {
                    final_board[i + 1][j] = !final_board[i + 1][j];
                }
                if j + 1 < m {
                    final_board[i][j + 1] = !final_board[i][j + 1];
                }
                final_board[i][j] = !final_board[i][j];
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            let expected = s[i][j] == b'1';
            if final_board[i][j] != expected {
                return None;
            }
            assert_eq!(final_board[i][j], expected);
        }
    }
    Some(ans)
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut s = vec![];
    for _ in 0..n {
        let ss = input.string();
        s.push(ss);
    }
    if let Some(ans) = solve_gauss(&s) {
        out.println("YES");
        for i in 0..n {
            for j in 0..m {
                if ans[i][j] {
                    out.print("1");
                } else {
                    out.print("0");
                }
            }
            out.println("");
        }
    } else {
        out.println("NO");
    }
}

fn solve_slow(s: &[Vec<u8>]) -> Option<Vec<Vec<bool>>> {
    let n = s.len();
    let m = s[0].len();
    let total = n * m;
    let mut ids = Array2D::new(0, n, m);
    let mut need_mask = 0;
    for i in 0..n {
        for j in 0..m {
            ids[i][j] = i * m + j;
            if s[i][j] == b'1' {
                need_mask |= 1 << ids[i][j];
            }
        }
    }
    let mut apply = Array2D::new(0, n, m);
    for i in 0..n {
        for j in 0..m {
            for i2 in 0..n {
                for j2 in 0..m {
                    let d = i.abs_diff(i2) + j.abs_diff(j2);
                    if d <= 1 {
                        apply[i][j] |= 1 << ids[i2][j2];
                    }
                }
            }
        }
    }
    for mask in 0..1 << total {
        let mut xor_mask = 0;
        for i in 0..n {
            for j in 0..m {
                if (mask >> ids[i][j]) & 1 == 1 {
                    xor_mask ^= apply[i][j];
                }
            }
        }
        if xor_mask == need_mask {
            let mut ans = vec![vec![false; m]; n];
            for i in 0..n {
                for j in 0..m {
                    if (mask >> ids[i][j]) & 1 == 1 {
                        ans[i][j] = true;
                    }
                }
            }
            return Some(ans);
        }
    }
    None
}

fn stress() {
    const N: usize = 5;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..N);
        let m = rnd.gen_range(1..N);
        let mut s = vec![vec![b'0'; m]; n];
        for i in 0..n {
            for j in 0..m {
                if rnd.gen_bool() {
                    s[i][j] = b'1';
                }
            }
        }
        let ans_slow = solve_slow(&s);
        let ans_fast = solve_gauss(&s);
        if ans_fast != ans_slow {
            if ans_fast.is_some() && ans_slow.is_some() {
                continue;
            }
            let s_str = s
                .iter()
                .map(|row| String::from_utf8_lossy(row))
                .collect::<Vec<_>>()
                .join("\n");
            dbg!(s_str);
            dbg!(ans_slow);
            dbg!(ans_fast);
        }
        assert_eq!(ans_slow, ans_fast);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
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
