//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Value {
    val: usize,
    loc: usize,
}

impl Value {
    fn new(val: usize, loc: usize) -> Self {
        Self { val, loc }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Op {
    PopPush(Value, Value),
    Halt(Value),
}

type Mod = Mod_998_244_353;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Res {
    NotSeen,
    Seen,
    Value(Mod, usize),
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut ops = vec![];
    for _ in 0..n {
        let s = input.string_as_string();
        if s == "HALT;" {
            input.string(); // PUSH
            let b = input.usize();
            input.string(); // GOTO
            let loc = input.usize() - 1;
            ops.push(Op::Halt(Value::new(b, loc)));
        } else {
            assert_eq!(s, "POP");
            let a = input.usize();
            // TODO: ;
            input.string(); // GOTO
            let mut loc1_bytes = input.string();
            loc1_bytes.pop();
            let mut loc1: usize = String::from_utf8(loc1_bytes).unwrap().parse().unwrap();
            loc1 -= 1;
            input.string(); // PUSH
            let b = input.usize();
            input.string(); // GOTO
            let loc2 = input.usize() - 1;
            ops.push(Op::PopPush(Value::new(a, loc1), Value::new(b, loc2)));
        }
    }
    let mut max_value = 0;
    for op in ops.iter() {
        match op {
            Op::PopPush(a, b) => {
                max_value = max_value.max(a.val).max(b.val);
            }
            Op::Halt(b) => {
                max_value = max_value.max(b.val);
            }
        }
    }
    max_value += 1;

    let mut dp = Array2D::new(Res::NotSeen, ops.len() + 1, max_value + 1);
    for value in 0..=max_value {
        dp[ops.len()][value] = Res::Value(Mod::ZERO, ops.len());
    }

    let res = RecursiveFunction2::new(|f, pos, top_val: usize| -> Res {
        let cur = dp[pos][top_val];
        if cur == Res::Seen {
            return Res::Seen;
        }
        if cur != Res::NotSeen {
            return cur;
        }
        dp[pos][top_val] = Res::Seen;

        match ops[pos] {
            Op::PopPush(v1, v2) => {
                if top_val == v1.val {
                    dp[pos][top_val] = Res::Value(Mod::ONE, v1.loc);
                } else {
                    let go = f.call(v2.loc, v2.val);
                    if go == Res::Seen {
                        dp[pos][top_val] = Res::Seen;
                    } else {
                        match go {
                            Res::NotSeen => todo!(),
                            Res::Seen => todo!(),
                            Res::Value(len1, loc2) => {
                                let go2 = f.call(loc2, top_val);
                                match go2 {
                                    Res::NotSeen => todo!(),
                                    Res::Seen => dp[pos][top_val] = Res::Seen,
                                    Res::Value(len2, loc3) => {
                                        dp[pos][top_val] = Res::Value(len1 + len2 + Mod::ONE, loc3);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Op::Halt(v2) => {
                if top_val == max_value {
                    dp[pos][top_val] = Res::Value(Mod::ONE, ops.len());
                } else {
                    let go = f.call(v2.loc, v2.val);
                    if go == Res::Seen {
                        dp[pos][top_val] = Res::Seen;
                    } else {
                        match go {
                            Res::NotSeen => todo!(),
                            Res::Seen => todo!(),
                            Res::Value(len1, loc2) => {
                                let go2 = f.call(loc2, top_val);
                                match go2 {
                                    Res::NotSeen => todo!(),
                                    Res::Seen => dp[pos][top_val] = Res::Seen,
                                    Res::Value(len2, loc3) => {
                                        dp[pos][top_val] = Res::Value(len1 + len2 + Mod::ONE, loc3);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        dp[pos][top_val]
    })
    .call(0, max_value);

    match res {
        Res::NotSeen => unreachable!(),
        Res::Seen => out.println(-1),
        Res::Value(res, _) => out.println(res),
    };
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
