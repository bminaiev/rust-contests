//{"name":"B. Обмен буквами","group":"Codeforces - VK 2022 Finals","url":"https://codeforces.com/gym/425375/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\nnwi\ninw\n3\ninn\nnww\nwii\n4\nwni\niww\nnni\nniw\n","output":"0\n2\n2 w 3 i\n3 w 1 n\n1\n2 w 3 n\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BObmenBukvami"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let chars = [b'w', b'i', b'n'];
    let mut cnt = vec![[0; 3]; n];
    for i in 0..n {
        let s = input.string();
        for &c in s.iter() {
            for j in 0..3 {
                if chars[j] == c {
                    cnt[i][j] += 1;
                }
            }
        }
    }
    let mut ops = vec![];
    let mut perm: Vec<_> = (0..n).collect();
    let mut rnd = Random::new(787788);
    let mut no_changes = 0;
    loop {
        let mut new_perm = vec![];
        for &x in perm.iter() {
            if cnt[x][0] != 1 || cnt[x][1] != 1 || cnt[x][2] != 1 {
                new_perm.push(x);
            }
        }
        perm = new_perm;
        if perm.is_empty() {
            break;
        }
        let mut any_changes = false;
        rnd.shuffle(&mut perm);
        for w in perm.windows(2) {
            let p1 = w[0];
            let p2 = w[1];
            for give in 0..3 {
                for take in 0..3 {
                    if cnt[p1][give] > 1 && (cnt[p2][give] == 0 || (no_changes > 50 && give == 0)) {
                        if cnt[p1][take] == 0 && cnt[p2][take] > 1 {
                            ops.push((p1, p2, give, take));
                            cnt[p1][give] -= 1;
                            cnt[p2][give] += 1;
                            cnt[p1][take] += 1;
                            cnt[p2][take] -= 1;
                            any_changes = true;
                        }
                    }
                }
            }
        }
        if any_changes {
            no_changes = 0;
        } else {
            no_changes += 1;
        }
    }
    out_line!(ops.len());
    for (p1, p2, give, take) in ops.into_iter() {
        let give = chars[give] as char;
        let take = chars[take] as char;
        out_line!(p1 + 1, give, p2 + 1, take);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
