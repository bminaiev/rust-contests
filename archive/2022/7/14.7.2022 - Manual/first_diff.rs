//{"name":"first_diff","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"first_diff"}}}

use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let t = input.string();
    let mut sum_pos = 0;
    const ITERS: usize = 100_000;
    let start = Instant::now();
    for _ in 0..ITERS {
        sum_pos += mismatch_fast(&s, &t);
    }
    let diff_mcs = start.elapsed().as_secs_f64() / (ITERS as f64) * 1000.0 * 1000.0;
    dbg!(diff_mcs);
    dbg!(sum_pos);
}

fn solve_gen(input: &mut Input, _test_case: usize) {
    let n = 200_001;
    let diff_pos = 100500;
    let mut rnd = Random::new(88999);
    let a = gen_slice(n, &mut rnd);
    let mut b = a.clone();
    while a[diff_pos] == b[diff_pos] {
        b[diff_pos] = rnd.gen(b'a'..b'z');
    }
    out_line!(vec2str(&a));
    out_line!(vec2str(&b));
}

fn gen_slice(n: usize, rnd: &mut Random) -> Vec<u8> {
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = rnd.gen(b'a'..b'z');
    }
    res
}



// #[inline(never)]
fn mismatch(s: &[u8], t: &[u8]) -> usize {
    let bound = std::cmp::min(s.len(), t.len());
    let res = s
        .iter()
        .zip(t.iter())
        .enumerate()
        .find(|(_, (x, y))| x != y);
    if let Some((i, _)) = res {
        i
    } else {
        bound
    }
}

#[inline(never)]
fn mismatch_fast(s: &[u8], t: &[u8]) -> usize {
    let bound = std::cmp::min(s.len(), t.len());
    let s = &s[..bound];
    let t = &t[..bound];

    const CHUNK_SIZE: usize = 64;
    let offset = s
        .chunks_exact(CHUNK_SIZE)
        .zip(t.chunks_exact(CHUNK_SIZE))
        .position(|(c1, c2)| c1 != c2)
        .unwrap_or(bound / CHUNK_SIZE)
        * CHUNK_SIZE;

    s[offset..]
        .iter()
        .zip(t[offset..].iter())
        .position(|(c1, c2)| c1 != c2)
        .unwrap_or(bound - offset)
        + offset
}

#[inline(never)]
pub fn mismatch_simd(xs: &[u8], ys: &[u8]) -> usize {
    let l = xs.len().min(ys.len());
    let mut xs = &xs[..l];
    let mut ys = &ys[..l];
    let mut off = 0;

    unsafe {
        use std::arch::x86_64::*;

        let zero = _mm256_setzero_si256();
        while xs.len() >= 32 {
            let x = _mm256_loadu_si256(xs.as_ptr() as _);
            let y = _mm256_loadu_si256(ys.as_ptr() as _);

            let r = _mm256_xor_si256(x, y);
            let r = _mm256_cmpeq_epi8(r, zero);
            let r = _mm256_movemask_epi8(r);
            if r.trailing_ones() < 32 {
                return off + r.trailing_ones() as usize;
            }

            xs = &xs[32..];
            ys = &ys[32..];
            off += 32;
        }
    }
    off + mismatch(xs, ys)
}

pub fn stress_correct() {
    let mut rnd = Random::new(88999);

    let mut sum_elapsed = 0.0;
    for cnt in 1.. {
        let n = rnd.gen(30..100);
        let mut diff_pos;
        loop {
            diff_pos = rnd.gen(0..n);
            if diff_pos > n / 10 * 9 {
                break;
            }
        }
        let a = gen_slice(n, &mut rnd);
        let mut b = a.clone();
        while a[diff_pos] == b[diff_pos] {
            b[diff_pos] = rnd.gen(0..std::u8::MAX);
        }

        let start = Instant::now();
        let found_pos = mismatch_fast(&a, &b);
        assert_eq!(found_pos, diff_pos);

        sum_elapsed += start.elapsed().as_secs_f64();
        let av_elapsed_ms = sum_elapsed / (cnt as f64) * 1000.0;
        dbg!(cnt, start.elapsed(), av_elapsed_ms);
    }
}

pub fn stress() {
    let mut rnd = Random::new(88999);
    let n = 105001235;
    let mut diff_pos;
    loop {
        diff_pos = rnd.gen(0..n);
        if diff_pos > n / 10 * 9 {
            break;
        }
    }
    let a = gen_slice(n, &mut rnd);
    let mut b = a.clone();
    b[diff_pos] = rnd.gen(0..std::u8::MAX);
    let mut sum_elapsed = 0.0;
    for cnt in 1.. {
        let start = Instant::now();
        let found_pos = mismatch_fast(&a, &b);
        assert_eq!(found_pos, diff_pos);

        sum_elapsed += start.elapsed().as_secs_f64();
        let av_elapsed_ms = sum_elapsed / (cnt as f64) * 1000.0;
        dbg!(start.elapsed(), av_elapsed_ms);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
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
    // tester::run_tests();
    // tester::run_stress(stress);
    tester::run_single_test("1");
    // submit();
    // tester::run_stress(stress);
}
//END MAIN
