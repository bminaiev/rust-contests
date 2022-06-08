//{"name":"H. Hoshi Sudoku","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/H/","interactive":false,"timeLimit":6000,"tests":[{"input":"        4\n  5....3..\n  ......7..\n ..1.   5...6\n9....   ....\n  .7..9...1\n   .4....23\n    .\n","output":"Single solution\n        4\n  52679318\n  413896725\n 8714   52396\n93652   1847\n  278396451\n   54697823\n    1\n"},{"input":"        4\n  5....3..\n  ......7..\n ..1.   5...6\n9....   ....\n  .7..9...1\n   .4..8.23\n    .\n","output":"No solutions\n"},{"input":"        4\n  .....3..\n  ......7..\n ..1.   ....6\n9....   ....\n  .7..9....\n   .4.....3\n    .\n","output":"456 solutions\n"},{"input":"        .\n  ........\n  .........\n ....   .....\n.....   ....\n  .........\n   ........\n    .\n","output":"801655626240 solutions\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHoshiSudoku"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{
    Callable0, Callable2, RecursiveFunction, RecursiveFunction0, RecursiveFunction2,
};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut cur = vec![];
    const UNKNOWN: usize = 9;
    while input.has_more_elements() {
        let token = input.string();
        for &c in &token {
            let val = if c == b'.' {
                UNKNOWN
            } else {
                (c - b'1') as usize
            };
            cur.push(val);
        }
    }
    assert_eq!(cur.len(), 54);
    solve_case(cur);
}

const UNKNOWN: usize = 9;

fn solve_case(mut cur: Vec<usize>) {
    assert_eq!(cur.len(), 54);

    let mut regions = vec![];

    let mut add_regions = |s: &str| {
        let mut pos = 0;

        let mut to_add = vec![vec![]; 26];

        for c in s.chars() {
            if c == '.' || c >= 'a' && c <= 'z' {
                if c != '.' {
                    let type_ = (c as i32 - 'a' as i32) as usize;
                    to_add[type_].push(pos);
                }
                pos += 1;
            }
        }
        for v in to_add.iter() {
            if v.len() == 0 {
                continue;
            }
            assert!(v.len() == 8 || v.len() == 9);
            regions.push(v.clone());
        }
    };

    {
        let a = r#" 
              .
        aaaaaaaa
        bbbbbbbbb
       cccc   ccccc
      ddddd   dddd
        eeeeeeeee
         ffffffff
          .
      "#;
        add_regions(a);
    }
    {
        let a = r#" 
              a
        ddccbbaa
        eddccbbaa
       feed   bbaa.
      .ffee   cbba
        ffeeddccb
         ffeeddcc
          f
      "#;
        add_regions(a);
    }
    {
        let a = r#" 
              a
        .ccbbaad
        ccbbaadde
       ccbb   deeff
      ccbba   eeff
        baaddeeff
         addeeff.
          d
      "#;
        add_regions(a);
    }
    {
        let a = r#" 
              a
        fffffaaa
        efffaaaaa
       eeef   bbbbb
      eeeee   cbbb
        dddddcccb
         dddccccc
          d
      "#;
        add_regions(a);
    }

    let mut inside_regions = vec![vec![]; cur.len()];
    for (reg_id, region) in regions.iter().enumerate() {
        for id in region.iter() {
            inside_regions[*id].push(reg_id);
        }
    }

    let mut used = vec![0; 9];
    for &c in cur.iter() {
        if c != UNKNOWN {
            used[c] += 1;
        }
    }

    let tot_regions = regions.len();
    let mut mask_by_region = vec![0; tot_regions];
    let mut ok = true;

    let full_mask = (1 << 9) - 1;

    for (pos, &c) in cur.iter().enumerate() {
        if c != UNKNOWN {
            for &r in inside_regions[pos].iter() {
                if (mask_by_region[r] & (1 << c)) != 0 {
                    ok = false;
                }
                mask_by_region[r] |= 1 << c;
            }
        }
    }
    let mut single_ans = None;
    let mut ways = 0;
    // let mut rnd = Random::new(898899);
    // perm.sort_by_key(|&id| (100 - inside_regions[id].len(), id));

    if ok {
        let mut not_used_mask: i32 = 0;
        for i in 0..used.len() {
            if used[i] == 0 {
                not_used_mask |= 1 << i;
            }
        }

        #[derive(Clone, Copy)]
        struct FullStackFrame {
            idx: usize,
            mult: i64,
            allowed: usize,
            first_not_used: usize,
        }

        #[derive(Clone, Copy)]
        struct Cleanup {
            idx: usize,
            next: usize,
        }

        #[derive(Clone, Copy)]
        enum StackFrame {
            FullStackFrame(FullStackFrame),
            Cleanup(Cleanup),
        }

        let mut stack = vec![];
        stack.push(StackFrame::FullStackFrame(FullStackFrame {
            idx: 0,
            mult: 1,
            allowed: std::usize::MAX,
            first_not_used: std::usize::MAX,
        }));

        let mut tot_change_mask_by_region = 0;

        while let Some(frame) = stack.last_mut() {
            if let StackFrame::Cleanup(cleanup) = frame {
                let next = cleanup.next;
                let idx = cleanup.idx;
                used[next] -= 1;
                if used[next] == 0 {
                    not_used_mask ^= 1 << next;
                }
                cur[idx] = UNKNOWN;
                unsafe {
                    for &r in inside_regions[idx].iter() {
                        *mask_by_region.get_unchecked_mut(r) ^= 1 << next;
                        tot_change_mask_by_region += 1;
                    }
                }
                stack.pop();
                continue;
            }
            let mut to_push = None;
            if let StackFrame::FullStackFrame(frame) = frame {
                if frame.idx == cur.len() {
                    ways += frame.mult;
                    if ways == 1 {
                        single_ans = Some(cur.clone());
                    }
                    stack.pop();
                    continue;
                }
                if cur[frame.idx] != UNKNOWN {
                    frame.idx += 1;
                    continue;
                }
                if frame.allowed == 0 {
                    stack.pop();
                    continue;
                }
                if frame.allowed == std::usize::MAX {
                    let first_not_used: usize = not_used_mask.trailing_zeros() as usize;

                    let mut allowed = full_mask;
                    unsafe {
                        for &r in inside_regions[frame.idx].iter() {
                            allowed &= !mask_by_region.get_unchecked(r);
                        }
                    }
                    allowed &= !not_used_mask;
                    if first_not_used < 9 {
                        allowed |= 1 << first_not_used;
                    }
                    let allowed = allowed as usize;
                    frame.allowed = allowed;
                    frame.first_not_used = first_not_used;
                    continue;
                }

                let idx = frame.idx;

                let next = frame.allowed.trailing_zeros() as usize;
                frame.allowed &= !(1 << next);
                let nmult = if next == frame.first_not_used {
                    not_used_mask.count_ones() as i64
                } else {
                    1
                };

                cur[idx] = next;
                unsafe {
                    for &r in inside_regions[idx].iter() {
                        *mask_by_region.get_unchecked_mut(r) |= 1 << next;
                    }
                }
                used[next] += 1;
                not_used_mask &= !(1 << next);
                let x1 = StackFrame::Cleanup(Cleanup { idx, next });
                let x2 = StackFrame::FullStackFrame(FullStackFrame {
                    idx: idx + 1,
                    mult: frame.mult * nmult,
                    allowed: std::usize::MAX,
                    first_not_used: std::usize::MAX,
                });
                to_push = Some((x1, x2));
            }
            if let Some((x1, x2)) = to_push {
                stack.push(x1);
                stack.push(x2);
            }
        }
        dbg!(tot_change_mask_by_region);
    }
    if ways == 0 {
        out_line!("No solutions");
    } else if ways == 1 {
        {
            out_line!("Single solution");
            let board = r#"
         a
   fffffaaa
  efffaaaaa
 eeef   bbbbb
eeeee   cbbb
  dddddcccb
   dddccccc
    d
          "#;
            let cur = single_ans.unwrap();
            let mut idx = 0;
            for c in board.chars().skip(1) {
                if c >= 'a' && c <= 'z' {
                    out!(cur[idx] + 1);
                    idx += 1;
                } else {
                    out!(c);
                }
            }
        }
    } else {
        out_line!(ways, "solutions");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    // solve(&mut input, 1);
    // output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
    stress();
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

fn stress() {
    let cur = vec![UNKNOWN; 54];
    solve_case(cur);
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    tester::run_single_test("4");
    // tester::run_stress(stress);
}
//END MAIN
