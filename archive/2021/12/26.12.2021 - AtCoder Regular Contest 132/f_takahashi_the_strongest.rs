//{"name":"F - Takahashi The Strongest","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_f","interactive":false,"timeLimit":5000,"tests":[{"input":"2 1 3\nRS\nRP\nRR\nRS\n","output":"3\n3\n3\n0\n1\n0\n0\n1\n0\n"},{"input":"3 5 4\nRRP\nSSS\nRSR\nPPP\nRSS\nPPS\nSRP\nSSP\nRRS\n","output":"4\n7\n7\n6\n9\n10\n4\n7\n8\n4\n8\n7\n4\n8\n8\n3\n7\n7\n3\n7\n6\n4\n8\n8\n1\n5\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTakahashiTheStrongest"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};
use std::ops::Not;

fn solve(input: &mut Input) {
    let k = input.usize();
    let n = input.usize();
    let m = input.usize();
    // how we need to move, to win
    let conv_char = |c: u8| -> u32 {
        if c == b'P' {
            3
        } else if c == b'R' {
            1
        } else if c == b'S' {
            2
        } else {
            unreachable!();
        }
    };
    let mut read_strategy = || -> u32 {
        let mut res = 0;
        let s = input.string_as_vec();
        for (idx, &c) in s.iter().rev().enumerate() {
            let val = conv_char(c);
            res |= val << (idx * 2);
        }
        res
    };
    let mut read_strategies = |n: usize| -> Vec<u32> { (0..n).map(|_| read_strategy()).collect() };
    let first = read_strategies(n);
    let second = read_strategies(m);
    // TODO: different type?
    let count_conv = |strategies: &[u32]| -> Vec<i64> {
        let mut res = vec![0; 1 << (k * 2)];
        for &strategy in strategies.iter() {
            res[strategy as usize] += 1;
        }
        for remove_bit in 0..k {
            let and_mask = (3 << (remove_bit * 2)).not() as usize;
            for mask in 0..(1 << (2 * k)) {
                let bit_val = (mask >> (remove_bit * 2)) & 3;
                if bit_val != 0 {
                    res[mask & and_mask] += res[mask];
                }
            }
        }
        res
    };
    let conv_first = count_conv(&first);
    let conv_second = count_conv(&second);
    let mut conv_mult = vec![0; 1 << (2 * k)];

    let bit_count = |mut m: usize| -> u32 {
        let mut res = 0;
        while m != 0 {
            if (m & 3) != 0 {
                res += 1;
            }
            m >>= 2;
        }
        res
    };

    for mask in 0..(1 << (2 * k)) {
        conv_mult[mask] = conv_first[mask] * conv_second[mask];
        if bit_count(mask) % 2 == 0 {
            conv_mult[mask] *= -1;
        }
    }

    for add_bit in 0..k {
        let mask_to_check = 3 << (2 * add_bit);
        for mask in 1..(1 << (2 * k)) {
            if mask & mask_to_check != 0 {
                continue;
            }
            for add in 1..4 {
                let nmask = mask | (add << (2 * add_bit));
                conv_mult[nmask] += conv_mult[mask];
            }
        }
    }

    for mask in 0..(1 << 2 * k) {
        if bit_count(mask) != k as u32 {
            continue;
        }
        let res = conv_mult[mask];
        assert!(res >= 0);
        assert!(res <= (n * m) as i64);
        out_line!(res);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
