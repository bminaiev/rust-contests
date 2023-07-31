//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r {
            z[i] = min(r + 1 - i, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

fn output_tandems(left: bool, l: usize, k1: usize, k2: usize) -> u64 {
    let mut res = 0;
    let from = max(1, l.saturating_sub(k2));
    let mut to = min(l, k1);
    if left {
        to = min(to, l - 1);
    }
    if from <= to {
        res += to - from + 1;
    }
    res as u64
}

fn get_z(z: &[usize], pos: usize) -> usize {
    if pos >= z.len() {
        return 0;
    }
    z[pos]
}

fn find_tandems(s: &[u8], shift: usize) -> u64 {
    let n = s.len();
    if n == 1 {
        return 0;
    }

    let nu = n / 2;
    let nv = n - nu;
    let u = &s[0..nu];
    let v = &s[nu..];
    let mut ru = u.to_vec();
    ru.reverse();
    let mut rv = v.to_vec();
    rv.reverse();

    let mut res = find_tandems(u, shift);
    res += find_tandems(v, shift + nu);

    let z1 = z_function(&ru);
    let z2 = {
        let mut s = v.to_vec();
        s.push(b'#');
        s.extend(u);
        z_function(&s)
    };
    let z3 = {
        let mut s = ru.to_vec();
        s.push(b'#');
        s.extend(rv);
        z_function(&s)
    };
    let z4 = z_function(v);
    for cntr in 0..n {
        let l;
        let k1;
        let k2;
        if cntr < nu {
            l = nu - cntr;
            k1 = get_z(&z1, nu - cntr);
            k2 = get_z(&z2, nv + 1 + cntr);
        } else {
            l = cntr - nu + 1;
            k1 = get_z(&z3, nu + 1 + nv - 1 - (cntr - nu));
            k2 = get_z(&z4, (cntr - nu) + 1);
        }
        if k1 + k2 >= l {
            res += output_tandems(cntr < nu, l, k1, k2);
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let res = find_tandems(&s, 0);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
