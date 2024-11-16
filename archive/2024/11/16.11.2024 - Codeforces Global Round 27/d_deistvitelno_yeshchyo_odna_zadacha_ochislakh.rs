//{"name":"D. Действительно, ещё одна задача о числах","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10\n1 2 3 4 5 6 7 8 9 10\n11\n1 6 9 4 7 4 4 10 3 2 3\n4\n527792568 502211460 850237282 374773208\n","output":"1 3 8 13 46 59 126 149 1174 1311\n1 7 22 26 70 74 150 1303 1306 1308 1568\n527792568 83665723 399119771 773892979\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDeistvitelnoYeshchyoOdnaZadachaOChislakh"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod7;

type Mod = Mod7;

#[derive(Clone, Copy)]
struct Elem {
    value: i64,
    pw2_cnt: usize,
    pw2: Mod,
    sum: Mod,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut sum = Mod::ZERO;
    let mut res = vec![];
    let mut elems: Vec<Elem> = vec![];
    for x in a.iter() {
        let mut x = *x;
        let mut pw2 = Mod::ONE;
        let mut cur_x = x;
        let mut pw2_cnt = 0;
        while x % 2 == 0 {
            pw2 *= Mod::new(2);
            x /= 2;
            pw2_cnt += 1;
        }
        let mut new_elem = Elem {
            value: x,
            pw2,
            pw2_cnt,
            sum: pw2 * Mod::new(x as i32),
        };
        while !elems.is_empty() && elems.last().unwrap().value <= cur_x {
            let elem = elems.pop().unwrap();
            sum -= elem.sum;
            sum += Mod::new(elem.value as i32);
            new_elem.sum *= elem.pw2;
            new_elem.pw2 *= elem.pw2;
            new_elem.pw2_cnt += elem.pw2_cnt;
            for _ in 0..elem.pw2_cnt {
                if cur_x > 1e10 as i64 {
                    break;
                }
                cur_x *= 2;
            }
        }
        elems.push(new_elem);
        sum += new_elem.sum;
        res.push(sum);
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_deistvitelno_yeshchyo_odna_zadacha_ochislakh";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
