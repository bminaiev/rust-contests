//{"name":"D2: Work-Life Balance - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 3\n1 2\n2 1 1\n1 2 1\n1 2 1\n4 3\n1 1 1 2\n1 2 2\n2 2 2\n4 1 2\n8 5\n1 1 1 1 2 2 2 2\n5 2 4\n7 2 3\n6 2 5\n1 2 4\n3 2 4\n","output":"Case #1: -2\nCase #2: 0\nCase #3: 16\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2WorkLifeBalanceChapter2"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree_set_sum::SegTreeSetSum;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type SegTree = SegTreeSetSum<i64>;

use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::binary_search::binary_search_last_true;

use algo_lib::seg_trees::lazy_seg_tree_set_sum::Node;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        res: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            let n = input.usize();
            let m = input.usize();
            let mut a = input.vec::<usize>(n);
            let mut cnt_st = vec![SegTree::new(&Node { sum: 0, len: 1 }, n, ()); 2];
            let mut pos_sum_st = vec![SegTree::new(&Node { sum: 0, len: 1 }, n, ()); 2];
            let change_value = |pos: usize,
                                value: usize,
                                delta: i64,
                                cnt_st: &mut [SegTree],
                                pos_sum_st: &mut [SegTree]| {
                cnt_st[value - 1].update(pos..pos + 1, delta);
                pos_sum_st[value - 1].update(pos..pos + 1, delta * (pos as i64));
            };

            let get_cost = |fr: usize,
                            to: usize,
                            value: usize,
                            cnt_st: &mut [SegTree],
                            pos_sum_st: &mut [SegTree],
                            from_left: bool|
             -> i64 {
                let cnt_need = (to - fr) as i64;
                let exp_sum = cnt_need * (to + fr) as i64 / 2;

                if from_left {
                    let first = binary_search_last_true(0..fr + 1, |mid| {
                        cnt_st[value - 1].get(mid..to).sum >= cnt_need
                    })
                    .unwrap();
                    let sum_pos = pos_sum_st[value - 1].get(first..to).sum;
                    exp_sum - sum_pos
                } else {
                    let last = binary_search_first_true(to..n, |mid| {
                        cnt_st[value - 1].get(fr..mid).sum >= cnt_need
                    });
                    let sum_pos = pos_sum_st[value - 1].get(fr..last).sum;
                    sum_pos - exp_sum
                }
            };
            for i in 0..n {
                change_value(i, a[i], 1, &mut cnt_st, &mut pos_sum_st);
            }
            dbg!("new test");
            for it in 0..m {
                let pos = input.usize() - 1;
                let value = input.usize();
                change_value(pos, a[pos], 0, &mut cnt_st, &mut pos_sum_st);
                a[pos] = value;
                dbg!(a);
                change_value(pos, a[pos], 1, &mut cnt_st, &mut pos_sum_st);
                let mid = input.usize();
                dbg!(mid);
                let mut res = || -> i64 {
                    let cnt1_left = cnt_st[0].get(0..mid).sum;
                    let cnt1_right = cnt_st[0].get(mid..n).sum;
                    let cnt2_left = (mid as i64) - cnt1_left;
                    let cnt2_right = n as i64 - cnt1_left - cnt1_right - cnt2_left;
                    let sum_left = cnt1_left + cnt2_left * 2;
                    let sum_right = cnt1_right + cnt2_right * 2;
                    let delta = sum_right - sum_left;
                    dbg!(it, delta, cnt1_left, cnt1_right, sum_left, sum_right);
                    if delta == 0 {
                        return 0;
                    }
                    if delta > 0 {
                        if cnt1_left < delta || cnt2_right < delta {
                            return -1;
                        }
                        let left_cost = get_cost(
                            mid - delta as usize,
                            mid,
                            1,
                            &mut cnt_st,
                            &mut pos_sum_st,
                            true,
                        );
                        let right_cost = get_cost(
                            mid,
                            mid + delta as usize,
                            2,
                            &mut cnt_st,
                            &mut pos_sum_st,
                            false,
                        );
                        return delta * delta + left_cost + right_cost;
                    } else {
                        let delta = -delta;
                        if cnt1_right < delta || cnt2_left < delta {
                            return -1;
                        }
                        let left_cost = get_cost(
                            mid - delta as usize,
                            mid,
                            2,
                            &mut cnt_st,
                            &mut pos_sum_st,
                            true,
                        );
                        let right_cost = get_cost(
                            mid,
                            mid + delta as usize,
                            1,
                            &mut cnt_st,
                            &mut pos_sum_st,
                            false,
                        );
                        return delta * delta + left_cost + right_cost;
                    }
                    unreachable!();
                };
                let r = res();
                dbg!(r);
                self.res += r;
            }
        }

        fn solve(&mut self) {}

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.res));
        }
    }

    run_parallel::<Job>(input, Some(1));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("worklife_balance_chapter__output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
