//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::convex_hull::convex_hull;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

fn solve_slow(first_bad: &[usize]) -> i64 {
    let n = first_bad.len();
    let mut res = 0;

    for first in 0..n {
        for offset in 1..first_bad[first] {
            let second = (first + offset) % n;
            for offset2 in 1..first_bad[second] {
                let third = (second + offset2) % n;
                let delta = (first + n - third) % n;
                if delta < first_bad[third] {
                    res += 1;
                }
            }
        }
    }
    assert!(res % 3 == 0);

    res / 3
}

fn c(n: i64, k: usize) -> i64 {
    if n <= 0 {
        return 0;
    }
    if k == 2 {
        return n * (n - 1) / 2;
    }
    if k == 3 {
        return n * (n - 1) * (n - 2) / 6;
    }
    unreachable!()
}

fn solve_fast(first_bad: &[usize], first_bad_rev: &[usize]) -> i64 {
    let n = first_bad.len();

    // dbg!(first_bad);
    // dbg!(first_bad_rev);

    for i in 0..n {
        assert!(first_bad[i] + first_bad_rev[i] <= n + 1);
    }

    let mut good_vertices = vec![false; n];
    for first in 0..n {
        let l1 = first_bad[first] - 1;
        let second = (first + l1) % n;
        let l2 = first_bad[second] - 1;
        let third = (second + l2) % n;
        let l3 = first_bad[third] - 1;
        if l1 + l2 + l3 >= n {
            good_vertices[first] = true;
        } else {
            // dbg!("BAD VERTEX!");
        }
    }
    let mut pref_good = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        pref_good[i + 1] = pref_good[i];
        if good_vertices[i % n] {
            pref_good[i + 1] += 1;
        }
    }
    let cnt_good = |start: usize, len: usize| -> i64 {
        let end = start + len;
        (pref_good[end] - pref_good[start]) as i64
    };
    let tot_good = cnt_good(0, n);
    // dbg!(tot_good);
    let mut res = c(tot_good, 3);
    let mut res2 = 0;
    for first in 0..n {
        if !good_vertices[first] {
            continue;
        }
        let next_good = cnt_good(first + 1, first_bad[first] - 1);
        res -= c(next_good, 2);

        let last_bad = n + 1 - first_bad_rev[first];

        let cnt_good_inside = if last_bad >= first_bad[first] {
            cnt_good((first + first_bad[first]) % n, last_bad - first_bad[first])
        } else {
            0
        };
        // dbg!(
        //     first,
        //     first_bad[first],
        //     last_bad,
        //     cnt_good_inside,
        //     next_good
        // );

        res2 -= cnt_good_inside * (tot_good - 2);

        res += c(cnt_good_inside, 2);
    }

    res + res2 / 2
}

fn calc_first_bad(a: &[Point], b: &[Point]) -> Vec<usize> {
    let n = a.len();
    let mut first_bad = vec![0; n];
    let mut closest_id = 0;
    for start in 0..n {
        if start == 0 {
            first_bad[start] = 1;
            let p = a[start];
            let q = a[(start + first_bad[start]) % n];
            let mut min_vmul = i64::MAX;
            for j in 0..b.len() {
                let vmul = Point::vect_mul(&p, &q, &b[j]);
                if vmul < min_vmul {
                    min_vmul = vmul;
                    closest_id = j;
                }
            }
        } else {
            first_bad[start] = first_bad[start - 1] - 1;
        }
        let p = a[start];
        loop {
            let q = a[(start + first_bad[start]) % n];
            loop {
                let vmul1 = Point::vect_mul(&p, &q, &b[closest_id]);
                let vmul2 = Point::vect_mul(&p, &q, &b[(closest_id + 1) % b.len()]);
                if vmul2 <= vmul1 {
                    closest_id = (closest_id + 1) % b.len();
                } else {
                    break;
                }
            }
            let vmul = Point::vect_mul(&p, &q, &b[closest_id]);
            if vmul >= 0 {
                first_bad[start] += 1;
            } else {
                break;
            }
        }
    }
    first_bad
}

fn solve_case(a: &[Point], b: &[Point], slow: bool) -> i64 {
    let n = a.len();
    let first_bad = calc_first_bad(a, b);
    let mut a_rev = a.to_vec();
    for i in 0..n {
        a_rev[i].x = -a_rev[i].x;
    }
    a_rev.reverse();
    let mut b_rev = b.to_vec();
    for i in 0..b.len() {
        b_rev[i].x = -b_rev[i].x;
    }
    b_rev.reverse();
    let mut first_bad_rev = calc_first_bad(&a_rev, &b_rev);
    first_bad_rev.reverse();
    if slow {
        solve_slow(&first_bad)
    } else {
        solve_fast(&first_bad, &first_bad_rev)
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for i in 0..n {
            let x = input.i64();
            let y = input.i64();
            a.push(Point::new(x, y));
        }
        let m = input.usize();
        let mut b = vec![];
        for i in 0..m {
            let x = input.i64();
            let y = input.i64();
            b.push(Point::new(x, y));
        }
        let r = solve_case(&a, &b, false);
        out.println(r);
    }
}

fn stress() {
    for it in 5937.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..100);
        const MAX_C: i64 = 100;
        let mut all = vec![];
        for _i in 0..n {
            let x = rnd.gen_range(-MAX_C..MAX_C);
            let y = rnd.gen_range(-MAX_C..MAX_C);
            all.push(Point::new(x, y));
        }
        let a = convex_hull(&all);
        let mut rest = vec![];
        for p in all.iter() {
            if !a.contains(p) {
                rest.push(*p);
            }
        }
        let b = convex_hull(&rest);
        if a.len() >= 3 && b.len() >= 3 {
            let slow = solve_case(&a, &b, true);
            let fast = solve_case(&a, &b, false);
            if slow != fast {
                dbg!(a.len());
                for p in a.iter() {
                    dbg!(p);
                }
                dbg!(b.len());
                for p in b.iter() {
                    dbg!(p);
                }
                panic!("Mismatch: slow = {}, fast = {}", slow, fast);
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
