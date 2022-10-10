//{"name":"A. Эла распределяет книги","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n12 3\ncabccadabaac\n12 6\ncabccadabaac\n12 12\ncabccadabaac\n25 1\nabcdefghijklmnopqrstuvwxy\n10 5\nbcdxedbcfg\n","output":"edb\nccbbba\nbbbbbaaaaaaa\nz\naaaaa\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AElaRaspredelyaetKnigi"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let s = input.string();
    let mut res = vec![b'a'; k];
    let mut cnt = vec![0; 26];
    for &c in s.iter() {
        let pos = (c - b'a') as usize;
        cnt[pos] += 1;
    }
    for i in 0..k {
        let mut r = 0;
        while r < n / k && cnt[r] != 0 {
            cnt[r] -= 1;
            r += 1;
        }
        res[i] = b'a' + (r as u8);
    }
    let ans = vec2str(&res);
    out_line!(ans);
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
