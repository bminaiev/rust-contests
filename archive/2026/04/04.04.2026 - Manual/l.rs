//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve_case(a: &Array2D<usize>, k: usize) -> usize {
    let n = a.len();
    let max_val = (n - k) * (n - k);
    let mut rnd = Random::new(7687788);
    let mut r = vec![0; n * n];
    for i in 0..n * n {
        r[i] = rnd.gen_u64();
    }
    let mut cnt = vec![0; max_val];
    // let mut a = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            // a[i][j] = input.usize() - 1;
            cnt[a[i][j]] += 1;
        }
    }
    for i in 0..max_val {
        if cnt[i] == 0 {
            return 0;
        }
    }
    let mut xor_rows = vec![0; n];
    let mut xor_cols = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            xor_rows[i] ^= r[a[i][j]];
            xor_cols[j] ^= r[a[i][j]];
        }
    }
    let mut expected_xor = 0;
    for i in 0..max_val {
        expected_xor ^= r[i];
    }
    let mut full_xor = 0;
    for i in 0..n {
        full_xor ^= xor_rows[i];
    }
    let mut allowed_rows = vec![];
    for i in 0..n {
        let mut ok = true;
        for j in 0..n {
            if cnt[a[i][j]] == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            allowed_rows.push(i);
        }
    }
    // dbg!(allowed_rows);
    assert!(allowed_rows.len() <= 20);
    let mut allowed_cols = vec![];
    for j in 0..n {
        let mut ok = true;
        for i in 0..n {
            if cnt[a[i][j]] == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            allowed_cols.push(j);
        }
    }
    assert!(allowed_cols.len() <= 20);
    let mut res = 0;
    let mut allowed_rows_masks = vec![];
    for mask in 0i32..(1 << allowed_rows.len()) {
        if mask.count_ones() as usize == k {
            let mut tmp = vec![];
            for i in 0..allowed_rows.len() {
                if (mask >> i) & 1 == 1 {
                    tmp.push(allowed_rows[i]);
                }
            }
            allowed_rows_masks.push((mask, tmp));
        }
    }
    let mut allowed_cols_masks = vec![];
    for mask in 0i32..(1 << allowed_cols.len()) {
        if mask.count_ones() as usize == k {
            let mut tmp = vec![];
            for j in 0..allowed_cols.len() {
                if (mask >> j) & 1 == 1 {
                    tmp.push(allowed_cols[j]);
                }
            }
            allowed_cols_masks.push((mask, tmp));
        }
    }
    for &(mask1, ref rows) in &allowed_rows_masks {
        for &(mask2, ref cols) in &allowed_cols_masks {
            let mut cur_xor = full_xor;
            for r in 0..rows.len() {
                cur_xor ^= xor_rows[rows[r]];
            }
            for c in 0..cols.len() {
                cur_xor ^= xor_cols[cols[c]];
            }
            for i in rows.iter() {
                for j in cols.iter() {
                    cur_xor ^= r[a[*i][*j]];
                }
            }
            if cur_xor == expected_xor {
                res += 1;
            }
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let k = input.usize();
    let mut a = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            a[i][j] = input.usize() - 1;
        }
    }
    let res = solve_case(&a, k);
    out.println(res);
}

fn stress() {
    for it in 17513.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..100);
        let k = rnd.gen_range(1..6);
        if k >= n {
            continue;
        }
        dbg!(n, k);

        let max_val = (n - k) * (n - k);
        let mut a = Array2D::new(0, n, n);
        for i in 0..n {
            for j in 0..n {
                a[i][j] = rnd.gen_range(0..max_val);
            }
        }
        let mut ok_rows = vec![];
        let mut ok_cols = vec![];
        for i in 0..n {
            ok_rows.push(i);
            ok_cols.push(i);
        }
        rnd.shuffle(&mut ok_rows);
        rnd.shuffle(&mut ok_cols);
        ok_rows.truncate(ok_rows.len() - k);
        ok_cols.truncate(ok_cols.len() - k);
        let mut it = 0;
        for i in 0..ok_rows.len() {
            for j in 0..ok_cols.len() {
                a[ok_rows[i]][ok_cols[j]] = it;
                it += 1;
            }
        }

        // for i in 0..n {
        //     dbg!(a[i]);
        // }

        let res = solve_case(&a, k);
        dbg!(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
