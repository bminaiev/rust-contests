//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone)]
struct Expr {
    s: String,
    values: Vec<f64>,
}

impl Expr {
    pub fn sin(&self) -> Self {
        let values: Vec<_> = self.values.iter().map(|x| x.sin()).collect();
        Self {
            s: format!("sin({})", self.s),
            values,
        }
    }

    pub fn cos(&self) -> Self {
        let values: Vec<_> = self.values.iter().map(|x| x.cos()).collect();
        Self {
            s: format!("cos({})", self.s),
            values,
        }
    }

    pub fn sum(&self, other: &Self) -> Self {
        let values: Vec<_> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x + y)
            .collect();
        Self {
            s: format!("({})+({})", self.s, other.s),
            values,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        let values: Vec<_> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x - y)
            .collect();
        Self {
            s: format!("({})-({})", self.s, other.s),
            values,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let values: Vec<_> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x * y)
            .collect();
        Self {
            s: format!("({})*({})", self.s, other.s),
            values,
        }
    }

    pub fn div(&self, other: &Self) -> Option<Self> {
        for x in other.values.iter() {
            if x.abs() < 0.015 {
                return None;
            }
        }
        let values: Vec<_> = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(x, y)| x / y)
            .collect();
        Some(Self {
            s: format!("({})/({})", self.s, other.s),
            values,
        })
    }
}

fn diff(x: f64, y: f64) -> f64 {
    let mut div = y.abs();
    if div < 1.0 {
        div = 1.0;
    }
    (x - y).abs() / div
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        x.push(input.f64());
        y.push(input.f64());
    }
    const MAX_COMPLEXITY: usize = 9;
    let mut exprs = vec![vec![]; MAX_COMPLEXITY + 1];
    exprs[0].push(Expr {
        s: "x".to_string(),
        values: x.clone(),
    });
    for complexity in 1..=MAX_COMPLEXITY {
        let mut res = vec![];
        for e in exprs[complexity - 1].iter() {
            res.push(e.sin());
            res.push(e.cos());
        }
        for c1 in 0..complexity {
            for c2 in 0..complexity {
                if c1 + c2 + 2 == complexity {
                    for e1 in exprs[c1].iter() {
                        for e2 in exprs[c2].iter() {
                            res.push(e1.sum(e2));
                            res.push(e1.sub(e2));
                            res.push(e1.mul(e2));
                            if let Some(e) = e1.div(e2) {
                                res.push(e);
                            }
                        }
                    }
                }
            }
        }
        exprs[complexity] = res;
        for e in exprs[complexity].iter() {
            let mut ok = true;
            for i in 0..n {
                if diff(e.values[i], y[i]) > 0.5 * 1e-3 {
                    ok = false;
                    break;
                }
            }
            if ok {
                out_line!(e.s);
                return;
            }
        }
    }
    assert!(false);
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
