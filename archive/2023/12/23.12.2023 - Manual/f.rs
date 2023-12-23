//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::f;
use algo_lib::geometry::dcel::dcel;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Debug)]
enum Formula {
    Atomic(Line),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Xor(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
}

fn parse_line(s: &[u8]) -> (Line, &[u8]) {
    let mut to = 0usize;
    let mut numbers: Vec<i64> = vec![];
    let mut cur_number = vec![];
    loop {
        if s[to] == b',' || s[to] == b']' {
            let ss = String::from_utf8(cur_number).unwrap();
            let value: i64 = ss.parse().unwrap();
            numbers.push(value);
            cur_number = vec![];
        } else {
            cur_number.push(s[to]);
        }
        if s[to] == b']' {
            break;
        }
        to += 1;
    }
    assert_eq!(numbers.len(), 3);
    let line = Line::new3(numbers[0], numbers[1], numbers[2]);
    (line, &s[to + 1..])
}

fn parse(s: &[u8]) -> (Box<Formula>, &[u8]) {
    assert!(!s.is_empty());
    let c = s[0];
    if c == b'(' {
        let next_c = s[1];
        if next_c == b'!' {
            let (formula, rest) = parse(&s[2..]);
            assert!(rest[0] == b')');
            (Box::new(Formula::Not(formula)), &rest[1..])
        } else {
            let (left, rest) = parse(&s[1..]);
            let c = rest[0];
            let (right, rest) = parse(&rest[1..]);
            assert!(rest[0] == b')');
            let rest = &rest[1..];
            if c == b'&' {
                (Box::new(Formula::And(left, right)), rest)
            } else if c == b'|' {
                return (Box::new(Formula::Or(left, right)), rest);
            } else if c == b'^' {
                return (Box::new(Formula::Xor(left, right)), rest);
            } else {
                panic!("Unknown operation {}", c as char);
            }
        }
    } else {
        assert!(c == b'[');
        let (line, rest) = parse_line(&s[1..]);
        (Box::new(Formula::Atomic(line)), rest)
    }
}

fn calc_all_lines(f: &Formula, all_lines: &mut Vec<Line>) {
    match f {
        Formula::Atomic(line) => {
            all_lines.push(line.clone());
        }
        Formula::And(left, right) => {
            calc_all_lines(left, all_lines);
            calc_all_lines(right, all_lines);
        }
        Formula::Or(left, right) => {
            calc_all_lines(left, all_lines);
            calc_all_lines(right, all_lines);
        }
        Formula::Xor(left, right) => {
            calc_all_lines(left, all_lines);
            calc_all_lines(right, all_lines);
        }
        Formula::Not(formula) => {
            calc_all_lines(formula, all_lines);
        }
    }
}

type Point = PointT<OrdF64>;

fn calc_formula(f: &Formula, p: Point) -> i32 {
    match f {
        Formula::Atomic(line) => {
            let res = line.a * p.x + line.b * p.y + line.c;
            if res >= f!(0.0) {
                1
            } else {
                0
            }
        }
        Formula::And(left, right) => calc_formula(left, p) & calc_formula(right, p),
        Formula::Or(left, right) => calc_formula(left, p) | calc_formula(right, p),
        Formula::Xor(left, right) => calc_formula(left, p) ^ calc_formula(right, p),
        Formula::Not(formula) => 1 - calc_formula(formula, p),
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let xmin = input.i64();
    let xmax = input.i64();
    let ymin = input.i64();
    let ymax = input.i64();
    let s = input.string();
    let (formula, rest) = parse(&s);
    assert!(rest.is_empty());
    let mut all_lines = vec![];
    calc_all_lines(&formula, &mut all_lines);
    {
        let a = (xmin, ymin);
        let b = (xmin, ymax);
        let c = (xmax, ymax);
        let d = (xmax, ymin);

        all_lines.push(Line::new_gcd(a, b));
        all_lines.push(Line::new_gcd(b, c));
        all_lines.push(Line::new_gcd(c, d));
        all_lines.push(Line::new_gcd(d, a));
    }

    for line in all_lines.iter_mut() {
        line.norm();
    }

    all_lines.sort();
    all_lines.dedup();
    let polygons = dcel(&all_lines);
    let mut res = 0.0;
    for polygon in polygons.iter() {
        let center = polygon.center_of_gravity();
        if center.x >= xmin.into()
            && center.x <= xmax.into()
            && center.y >= ymin.into()
            && center.y <= ymax.into()
        {
            let formula_res = calc_formula(&formula, center);
            if formula_res == 1 {
                res += polygon.area().0;
            }
        }
    }
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
