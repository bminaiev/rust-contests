//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use std::mem;
use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn add_dp_old(dp: &mut [i64], right_sides: &[i64], left_sides: &[i64]) {
    let n = dp.len();
    let dp = &mut dp[..n];
    let right_sided = &right_sides[..n];
    let left_sides = &left_sides[..n];
    for i in 0..n {
        dp[i] += right_sided[i] - left_sides[i];
    }
}

#[target_feature(enable = "avx2")]
unsafe fn add_dp(dp: &mut [i64], right_sides: &[i64], left_sides: &[i64]) {
    let n = dp.len();

    // Ensure slices are at least as long as dp
    assert!(right_sides.len() >= n && left_sides.len() >= n);

    let mut i = 0;
    let simd_width = 4; // Number of i64 values per __m256i vector

    // Process in chunks of 4 elements
    while i + simd_width <= n {
        // Load 4 i64 elements from dp, right_sides, and left_sides
        let dp_vec = _mm256_loadu_si256(dp.as_ptr().add(i) as *const __m256i);
        let right_vec = _mm256_loadu_si256(right_sides.as_ptr().add(i) as *const __m256i);
        let left_vec = _mm256_loadu_si256(left_sides.as_ptr().add(i) as *const __m256i);

        // Compute right_sides[i..i+4] - left_sides[i..i+4]
        let diff_vec = _mm256_sub_epi64(right_vec, left_vec);

        // Add the difference to dp[i..i+4]
        let result_vec = _mm256_add_epi64(dp_vec, diff_vec);

        // Store the result back into dp[i..i+4]
        _mm256_storeu_si256(dp.as_mut_ptr().add(i) as *mut __m256i, result_vec);

        i += simd_width;
    }

    // Process any remaining elements individually
    while i < n {
        dp[i] += right_sides[i] - left_sides[i];
        i += 1;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn mx_old(dp: &[i64], ndp: &mut [i64]) {
    let n = dp.len();
    let ndp = &mut ndp[..n];
    for i in 1..n {
        ndp[i] = dp[i - 1].max(dp[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn mx2(dp: &mut [i64]) {
    let n = dp.len();
    for i in 1..n {
        dp[i] = dp[i - 1].max(dp[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn mx22(dp: &mut [i64]) {
    let n = dp.len();
    if n == 0 {
        return;
    }

    let mut i = 1;
    let simd_width = 4; // Number of i64 values per __m256i vector

    // Initialize previous maximum with dp[0]
    let mut prev_max = _mm256_set1_epi64x(dp[0]);

    // Process in chunks of 4 elements
    while i + simd_width - 1 < n {
        // Load dp[i..i+4]
        let current = _mm256_loadu_si256(dp.as_ptr().add(i) as *const __m256i);

        // Create a vector with previous maximums
        let mut max_vec = prev_max;

        // Compute the maximum between current and max_vec
        // Compare current > max_vec
        let cmp_mask = _mm256_cmpgt_epi64(current, max_vec);
        // Blend values based on the comparison mask
        max_vec = _mm256_blendv_epi8(max_vec, current, cmp_mask);

        // Shift max_vec and repeat to propagate the maximum within the vector
        // Shift by one element (8 bytes)
        let shifted1 = _mm256_permute4x64_epi64(max_vec, 0b10010011); // Rotate elements
        let cmp_mask1 = _mm256_cmpgt_epi64(shifted1, max_vec);
        max_vec = _mm256_blendv_epi8(max_vec, shifted1, cmp_mask1);

        // Shift again and compare
        let shifted2 = _mm256_permute4x64_epi64(max_vec, 0b01001110); // Rotate elements
        let cmp_mask2 = _mm256_cmpgt_epi64(shifted2, max_vec);
        max_vec = _mm256_blendv_epi8(max_vec, shifted2, cmp_mask2);

        // Store the result back into dp[i..i+4]
        _mm256_storeu_si256(dp.as_mut_ptr().add(i) as *mut __m256i, max_vec);

        // Update prev_max with the last element of max_vec
        let last_value = _mm256_extract_epi64(max_vec, 3);
        prev_max = _mm256_set1_epi64x(last_value);

        i += simd_width;
    }

    // Handle remaining elements
    while i < n {
        dp[i] = dp[i - 1].max(dp[i]);
        i += 1;
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn mx(dp: &[i64], ndp: &mut [i64]) {
    if dp.len() < 4 {
        return mx_old(dp, ndp);
    }

    let n = dp.len();
    let ndp = &mut ndp[..n];

    let mut i = 1;
    let limit = n - 4;

    while i <= limit {
        // Load dp[i - 1 .. i + 3]
        let a = _mm256_loadu_si256(dp[i - 1..].as_ptr() as *const __m256i);
        // Load dp[i .. i + 4]
        let b = _mm256_loadu_si256(dp[i..].as_ptr() as *const __m256i);

        // Compare a and b
        let mask = _mm256_cmpgt_epi64(a, b);
        // Use blend to select max values
        let max = _mm256_blendv_epi8(b, a, mask);

        // Store max into ndp[i .. i + 3]
        _mm256_storeu_si256(ndp[i..].as_mut_ptr() as *mut __m256i, max);

        i += 4;
    }

    // Handle remaining elements
    for j in i..n {
        ndp[j] = dp[j - 1].max(dp[j]);
    }
}

fn solve_case(a: &mut Vec<i64>, max_size: i64) -> i64 {
    a.sort();
    let n = a.len();
    let mut dp = vec![0; n + 1];
    let mut left_sides = vec![0; n + 1];
    for left_used in 0..=n {
        left_sides[left_used] = if left_used == 0 { 0 } else { a[left_used - 1] };
    }
    let mut right_sides = a.clone();
    right_sides.push(max_size + 1);
    // right_sides.reverse();

    // let mut ndp = vec![0; n + 1];
    for removed in 0..n {
        // for left_used in 0..=removed {
        //     let len = right_sides[n - removed + left_used] - left_sides[left_used];
        //     dp[left_used] += len;
        // }
        unsafe {
            add_dp(
                &mut dp[..=removed],
                &right_sides[n - removed..],
                &left_sides,
            );
        }
        // ndp[0] = dp[0];
        // for left_used in 0..=removed {
        //     ndp[left_used + 1] = dp[left_used + 1].max(dp[left_used]);
        // }
        unsafe {
            mx2(&mut dp[..=removed + 1]);
        }
        // mem::swap(&mut dp, &mut ndp);
    }
    let res = *dp.iter().max().unwrap();
    res
}

fn solve_case2(a: &mut Vec<i64>, max_size: i64) -> i64 {
    a.push(0);
    a.push(max_size + 1);
    a.sort();
    let n = a.len();
    let mut res = 0;
    for split in 1..a.len() - 1 {
        let mut cur_res = 0;
        let mut left_index = split - 1;
        let mut right_index = split;
        while left_index > 0 || right_index < a.len() - 1 {
            let mut use_left = true;
            if right_index == a.len() - 1 {
                use_left = true;
            } else if left_index == 0 {
                use_left = false;
            } else {
                let left_score = a[left_index] - a[left_index - 1];
                let right_score = a[right_index + 1] - a[right_index];
                use_left = left_score > right_score;
            }
            if use_left {
                cur_res += a[right_index] - a[left_index - 1];
                left_index -= 1;
            } else {
                cur_res += a[right_index + 1] - a[left_index];
                right_index += 1;
            }
        }
        res = res.max(cur_res);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let max_size = input.i64();
    let mut a = input.vec::<i64>(n);
    let res = solve_case2(&mut a, max_size);
    out.println(res);
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let n = 3;
        const MAX: i64 = 10; //_000_000_000;
        let mut rnd = Random::new(it);
        let a = rnd.gen_vec(n, 1..MAX);
        let max_size = MAX;
        let start = Instant::now();
        let res = solve_case2(&mut a.clone(), max_size);
        let res2 = solve_case(&mut a.clone(), max_size);
        dbg!(res, start.elapsed());
        dbg!(a, res, res2, MAX);
        assert_eq!(res, res2);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
