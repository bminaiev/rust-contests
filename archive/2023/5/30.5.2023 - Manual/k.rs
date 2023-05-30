//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::{Mod9, Mod_998_244_353};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod1 = Mod_998_244_353;
type Mod2 = Mod9;

fn solve_smart(field: &[Vec<u8>], need_cnt: usize) -> Option<usize> {
    let n = field.len();
    let m = field[0].len();
    let mut rnd = Random::new(77788);
    let magic1 = gen_vec(256, |_| Mod1::new(rnd.gen(1..1 << 30)));
    const B: usize = 13;
    let mut hashes1 = vec![];
    {
        let mut h0 = Array2D::new(Mod1::ZERO, n, m);
        for i in 0..n {
            for j in 0..m {
                h0[i][j] = magic1[field[i][j] as usize];
            }
        }
        hashes1.push(h0);
    }

    let magic2 = gen_vec(256, |_| Mod2::new(rnd.gen(1..1 << 30)));
    let mut hashes2 = vec![];
    {
        let mut h0 = Array2D::new(Mod2::ZERO, n, m);
        for i in 0..n {
            for j in 0..m {
                h0[i][j] = magic2[field[i][j] as usize];
            }
        }
        hashes2.push(h0);
    }

    let pow2 = Mod1::gen_powers(Mod1::new(239017), n * m + 1);
    let pow22 = Mod2::gen_powers(Mod2::new(239017), n * m + 1);

    for step in 1..B {
        let mut nh = Array2D::new(Mod1::ZERO, n, m);
        let last = *hashes1.last().as_ref().unwrap();

        let len = 1 << step;
        let mul = if len * len < pow2.len() {
            pow2[len * len]
        } else {
            Mod1::ZERO
        };

        for i in 0..n {
            for j in 0..m {
                if i + len <= n && j + len <= m {
                    let mut h = last[i][j];
                    h *= mul;
                    h += last[i + len / 2][j];
                    h *= mul;
                    h += last[i][j + len / 2];
                    h *= mul;
                    h += last[i + len / 2][j + len / 2];
                    nh[i][j] = h;
                }
            }
        }

        hashes1.push(nh);
    }

    for step in 1..B {
        let mut nh = Array2D::new(Mod2::ZERO, n, m);
        let last = *hashes2.last().as_ref().unwrap();

        let len = 1 << step;
        let mul = if len * len < pow22.len() {
            pow22[len * len]
        } else {
            Mod2::ZERO
        };

        for i in 0..n {
            for j in 0..m {
                if i + len <= n && j + len <= m {
                    let mut h = last[i][j];
                    h *= mul;
                    h += last[i + len / 2][j];
                    h *= mul;
                    h += last[i][j + len / 2];
                    h *= mul;
                    h += last[i + len / 2][j + len / 2];
                    nh[i][j] = h;
                }
            }
        }

        hashes2.push(nh);
    }

    let res = binary_search_last_true(1..n + m + 1, |check_len: usize| -> bool {
        let mut all_hashes = vec![];
        let mut ix = 0;
        while (1 << (ix + 1)) <= check_len {
            ix += 1;
        }
        let offset = check_len - (1 << ix);

        let mul = Mod1::new(1_000_123_71);
        let mul2 = Mod2::new(1_000_3_71);

        let last = &hashes1[ix];
        let last2 = &hashes2[ix];

        for i in 0..n {
            for j in 0..m {
                if i + check_len <= n && j + check_len <= m {
                    let mut h1 = last[i][j];
                    h1 *= mul;
                    h1 += last[i + offset][j];
                    h1 *= mul;
                    h1 += last[i][j + offset];
                    h1 *= mul;
                    h1 += last[i + offset][j + offset];

                    let mut h2 = last2[i][j];
                    h2 *= mul2;
                    h2 += last2[i + offset][j];
                    h2 *= mul2;
                    h2 += last2[i][j + offset];
                    h2 *= mul2;
                    h2 += last2[i + offset][j + offset];

                    all_hashes.push((h1, h2));
                }
            }
        }

        all_hashes.sort();
        for i in 0..all_hashes.len() {
            if i + need_cnt <= all_hashes.len() {
                if all_hashes[i] == all_hashes[i + need_cnt - 1] {
                    return true;
                }
            }
        }
        false
    });
    res
}

fn solve_slow(field: &[Vec<u8>], need_cnt: usize) -> Option<usize> {
    let n = field.len();
    let m = field[0].len();

    let check = |check_len: usize| -> bool {
        let mut all_hashes = vec![];
        for i in 0..n {
            for j in 0..m {
                if i + check_len <= n && j + check_len <= m {
                    let mut h = vec![];
                    for ii in i..i + check_len {
                        for jj in j..j + check_len {
                            h.push(field[ii][jj]);
                        }
                    }
                    all_hashes.push(h);
                }
            }
        }

        all_hashes.sort();
        for i in 0..all_hashes.len() {
            if i + need_cnt <= all_hashes.len() {
                if all_hashes[i] == all_hashes[i + need_cnt - 1] {
                    return true;
                }
            }
        }
        false
    };
    if !check(1) {
        return None;
    }
    let mut res = 1;
    while check(res + 1) {
        res += 1;
    }
    Some(res)
}

fn stress() {
    for it in 1882.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const N: usize = 50;
        let n = rnd.gen(1..N);
        let m = rnd.gen(1..N);
        let mut field = vec![];
        for i in 0..n {
            let mut row = vec![];
            for j in 0..m {
                let c = rnd.gen(b'a'..b'e');
                row.push(c);
            }
            field.push(row);
        }
        if n * m < 2 {
            continue;
        }
        let cnt_need = rnd.gen(2..10);
        let smart = solve_smart(&field, cnt_need);
        let slow = solve_slow(&field, cnt_need);
        dbg!(smart, slow, n, m, cnt_need);
        assert_eq!(smart, slow);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let need_cnt = input.usize();
    let mut field = vec![];
    for _ in 0..n {
        field.push(input.string());
    }
    let res = solve_smart(&field, need_cnt);
    if let Some(res) = res {
        out_line!(res);
    } else {
        out_line!(-1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
