//{"name":"D - Range XOR","group":"AtCoder - AtCoder Regular Contest 133","url":"https://atcoder.jp/contests/arc133/tasks/arc133_d","interactive":false,"timeLimit":2000,"tests":[{"input":"1 3 3\n","output":"2\n"},{"input":"10 20 0\n","output":"6\n"},{"input":"1 1 1\n","output":"1\n"},{"input":"12345 56789 34567\n","output":"16950\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRangeXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::range_intersect::range_intersect;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::{dbg, out, out_line};
use std::cmp::max;
use std::collections::HashMap;
use std::ops::{Not, Range};

type Mod = Mod_998_244_353;
const MOD_VAL: i64 = 998_244_353;

// WA????

fn solve(input: &mut Input) {
    let l = input.i64();
    let r = input.i64();
    let need_xor = input.i64();
    let mut res = Mod::ZERO;
    let last_two_bits = [0, 1, 3, 0];
    let use_for_xor = [true, false, true, false];

    let is_empty = |range: &Range<i64>| range.start >= range.end;

    for first_end in 0..4 {
        for second_end in 0..4 {
            let mut cache: HashMap<(usize, i64, i64), Mod> = HashMap::new();

            let expected_last_two_bits =
                last_two_bits[first_end as usize] ^ last_two_bits[second_end as usize];
            if expected_last_two_bits != need_xor & 3 {
                continue;
            }
            // more_bits, pref_first, pref_second
            let cur_res = RecursiveFunction3::new(
                |f, more_bits: usize, mut pref_first: i64, mut pref_second: i64| -> Mod {
                    if pref_first > pref_second {
                        return Mod::ZERO;
                    }
                    let size = 1i64 << more_bits;
                    let first_range = range_intersect(l - 1..r, pref_first..pref_first + size);
                    if is_empty(&first_range) {
                        return Mod::ZERO;
                    }
                    let second_range = range_intersect(l..r + 1, pref_second..pref_second + size);
                    if is_empty(&second_range) {
                        return Mod::ZERO;
                    }
                    if more_bits <= 1 {
                        if (pref_first & 2) != first_end & 2 {
                            return Mod::ZERO;
                        }
                        if (pref_second & 2) != second_end & 2 {
                            return Mod::ZERO;
                        }
                    }
                    if more_bits == 0 {
                        if (pref_first & 1) != first_end & 1 {
                            return Mod::ZERO;
                        }
                        if (pref_second & 1) != second_end & 1 {
                            return Mod::ZERO;
                        }
                        return if pref_first < pref_second {
                            Mod::ONE
                        } else {
                            Mod::ZERO
                        };
                    }
                    if more_bits > 2 {
                        if first_range == (pref_first..pref_first + size) {
                            if second_range == (pref_second..pref_second + size) {
                                let use_first = use_for_xor[first_end as usize];
                                let use_second = use_for_xor[second_end as usize];

                                let total_options = Mod::new(
                                    ((1i64 << max(0, (more_bits as i32) - 2)) % MOD_VAL) as i32,
                                );

                                let need_xor_part = (need_xor & ((1i64 << more_bits) - 1)) >> 2;
                                let need_xor_part_mod =
                                    Mod::new(((need_xor_part) % MOD_VAL) as i32);
                                let additional = if first_end < second_end {
                                    Mod::ONE
                                } else {
                                    Mod::ZERO
                                };

                                let res = if !use_first && !use_second {
                                    if (need_xor & (3i64.not())) != 0 {
                                        Mod::ZERO
                                    } else {
                                        if pref_first == pref_second {
                                            if first_end < second_end {
                                                total_options * (total_options + Mod::ONE)
                                                    / Mod::TWO
                                            } else {
                                                total_options * (total_options - Mod::ONE)
                                                    / Mod::TWO
                                            }
                                        } else {
                                            total_options * total_options
                                        }
                                    }
                                } else if use_first && use_second {
                                    if pref_first == pref_second {
                                        if need_xor_part == 0 {
                                            if first_end < second_end {
                                                total_options
                                            } else {
                                                Mod::ZERO
                                            }
                                        } else {
                                            total_options
                                        }
                                    } else {
                                        total_options
                                    }
                                } else if use_first && !use_second {
                                    if pref_first == pref_second {
                                        (total_options - need_xor_part_mod - Mod::ONE) + additional
                                    } else {
                                        total_options
                                    }
                                } else if !use_first && use_second {
                                    if pref_first == pref_second {
                                        need_xor_part_mod + additional
                                    } else {
                                        total_options
                                    }
                                } else {
                                    unreachable!();
                                };

                                return res;
                            }
                        }
                    }

                    if more_bits > 2 {
                        if !use_for_xor[first_end as usize] {
                            pref_first &= (1i64 << more_bits).not();
                        }

                        if !use_for_xor[second_end as usize] {
                            pref_second &= (1i64 << more_bits).not();
                        }
                    }

                    let cache_id = (more_bits, pref_first, pref_second);
                    if let Some(val) = cache.get(&cache_id) {
                        if more_bits > 2 {
                            return *val;
                        }
                    }

                    let multiplier = 1i64 << (more_bits - 1);
                    let mut res = Mod::ZERO;
                    for bit_first in 0..2 {
                        for bit_second in 0..2 {
                            let next_pref_first = pref_first + multiplier * bit_first;
                            let next_pref_second = pref_second + multiplier * bit_second;
                            let mut final_bit = 0;
                            if use_for_xor[first_end as usize] && bit_first == 1 {
                                final_bit ^= 1;
                            }
                            if use_for_xor[second_end as usize] && bit_second == 1 {
                                final_bit ^= 1;
                            }
                            if more_bits > 2 {
                                let expected_final_bit = (need_xor >> (more_bits - 1)) & 1;
                                if expected_final_bit != final_bit {
                                    continue;
                                }
                            }

                            // TODO: optimize for similar arguments
                            res += f.call(more_bits - 1, next_pref_first, next_pref_second);
                        }
                    }

                    if more_bits > 2 {
                        cache.insert(cache_id, res);
                    }

                    return res;
                },
            )
            .call(61, 0, 0);
            res += cur_res;
        }
    }
    out_line!(res);
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
