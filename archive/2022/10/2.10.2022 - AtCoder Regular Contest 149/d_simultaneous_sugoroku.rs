//{"name":"D - Simultaneous Sugoroku","group":"AtCoder - AtCoder Regular Contest 149","url":"https://atcoder.jp/contests/arc149/tasks/arc149_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6 4\n2 4 6 8 10 12\n8 2 5 7\n","output":"No -6\nNo -4\nYes 2\nYes 1\nYes 2\nNo 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSimultaneousSugoroku"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
enum Res {
    Finished(usize),
    LastPos(i32),
}

struct Dsu {
    p: Vec<usize>,
    xor: Vec<i32>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            p: gen_vec(n, id),
            xor: vec![0; n],
        }
    }

    pub fn get(&self, mut v: usize) -> usize {
        while v != self.p[v] {
            v = self.p[v];
        }
        v
    }

    pub fn get_xor(&self, mut v: usize) -> i32 {
        let mut res = self.xor[v];
        while v != self.p[v] {
            v = self.p[v];
            res ^= self.xor[v];
        }
        res
    }

    pub fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        assert_ne!(x, y);
        self.p[x] = y;
        self.xor[x] ^= self.xor[y];
    }
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let xs = input.vec::<usize>(n);
    let max_c = xs.iter().max().unwrap() + 1;
    const MX: usize = std::usize::MAX;
    let mut who = vec![MX; max_c];
    for i in 0..n {
        who[xs[i]] = i;
    }
    let mut res = vec![Res::Finished(std::usize::MAX); n];
    let mut dsu = Dsu::new(n);

    let mut arr = &mut who[..];
    let mut offset = 0;
    for time in 0..m {
        let d = input.i32();
        if offset < 0 {
            offset += d;
        } else {
            offset -= d;
        }
        if offset <= 0 && offset + arr.len() as i32 > 0 {
            let left_sz = (-offset) as usize;
            let right_sz = arr.len() - left_sz - 1;
            let mid = arr[left_sz];
            if mid != MX {
                let idx = dsu.get(mid);
                res[idx] = Res::Finished(time + 1);
            }
            if left_sz < right_sz {
                for i in 0..left_sz {
                    let p1 = i;
                    let p2 = left_sz + (left_sz - i);
                    if arr[p1] != MX {
                        let idx = dsu.get(arr[p1]);
                        dsu.xor[idx] ^= 1;
                    }
                    if arr[p2] == MX {
                        arr[p2] = arr[p1];
                    } else if arr[p1] != MX {
                        dsu.unite(arr[p1], arr[p2]);
                    }
                }
                offset += left_sz as i32 + 1;
                arr = &mut arr[left_sz + 1..];
            } else {
                for i in 0..right_sz {
                    let p1 = left_sz + 1 + i;
                    let p2 = left_sz - i - 1;
                    if arr[p1] != MX {
                        let idx = dsu.get(arr[p1]);
                        dsu.xor[idx] ^= 1;
                    }
                    if arr[p2] == MX {
                        arr[p2] = arr[p1];
                    } else if arr[p1] != MX {
                        dsu.unite(arr[p1], arr[p2]);
                    }
                }
                arr = &mut arr[..left_sz];
            }
        }
    }
    for i in 0..arr.len() {
        if arr[i] != MX {
            res[dsu.get(arr[i])] = Res::LastPos(i as i32 + offset);
        }
    }
    for i in 0..n {
        match res[dsu.get(i)] {
            Res::Finished(time) => out_line!("Yes", time),
            Res::LastPos(pos) => {
                let xor = dsu.get_xor(i);
                let mul = if xor == 0 { 1 } else { -1 };
                out_line!("No", pos * mul)
            }
        }
    }
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
