//{"name":"E. MEX против DIFF","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 1\n3 0 1 2\n4 1\n0 2 4 5\n7 2\n4 13 0 0 13 1337 1000000000\n6 2\n1 2 8 0 0 0\n","output":"0\n1\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMEXProtivDIFF"}}}

use algo_lib::collections::multiset::MultiSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct RemoveGroups {
    k: usize,
    groups: MultiSet<usize>,
    sum_group_sizes: usize,
}

impl RemoveGroups {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            groups: MultiSet::new(),
            sum_group_sizes: 0,
        }
    }

    fn add_group(&mut self, sz: usize) {
        self.groups.insert(sz);
        self.sum_group_sizes += sz;
        while self.sum_group_sizes > self.k {
            let last = *self.groups.last().unwrap();
            self.groups.remove(&last);
            self.sum_group_sizes -= last;
        }
    }

    fn calc_max_removed_groups(&self) -> usize {
        self.groups.len_total()
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<usize>(n);
    let mut cnt = MultiSet::new();
    for val in a.into_iter() {
        cnt.insert(val);
    }
    let total_groups = cnt.len_groups();
    let mut res = n;
    let mut cnt0s = n;
    let mut rg = RemoveGroups::new(k);

    for (key, cnt) in cnt.iter_counts() {
        if *key >= n {
            rg.add_group(cnt);
        } else {
            cnt0s -= 1;
        }
    }
    for max_elem in (0..n).rev() {
        if cnt0s <= k {
            let to_remove = rg.calc_max_removed_groups();
            let diff = total_groups + cnt0s - to_remove;
            res.update_min(diff - (max_elem + 1));
        }
        match cnt.get_count(&max_elem) {
            0 => cnt0s -= 1,
            sz => rg.add_group(sz),
        }
    }

    {
        let can_remove = rg.calc_max_removed_groups();
        if can_remove == total_groups {
            res.update_min(0);
        } else {
            res.update_min(total_groups - can_remove);
        }
    }

    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
    // tester::run_single_test("3");
}
//END MAIN
