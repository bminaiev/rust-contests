//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::point_location::PointLocation;
use algo_lib::geometry::polygon::PolygonT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;
type Mod = Mod_998_244_353;

fn slow(good: &Array2D<bool>) -> usize {
    let n = good.len();
    let mut res = 0;
    let mut by_first = vec![0; n];
    for mask in 0i32..(1 << n) {
        if mask.count_ones() >= 2 {
            let mut first = 0;
            while ((1 << first) & mask) == 0 {
                first += 1;
            }
            let mut mm = vec![];
            let mut ok = true;
            for i in 0..n {
                if ((1 << i) & mask) != 0 {
                    mm.push(1);
                    for j in i + 1..n {
                        if ((1 << j) & mask) != 0 {
                            if !good[i][j] {
                                ok = false;
                            }
                        }
                    }
                } else {
                    mm.push(0);
                }
            }
            if ok {
                // dbg!(mm);
                res += 1;
                by_first[first] += 1;
            }
        }
    }
    // dbg!(by_first);

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |_| Point::new(input.i64() * 2, input.i64() * 2));
    let poly = PolygonT::new(a.clone());
    let point_location = PointLocation::new(&[a.clone()]);
    a.sort();
    let mut good = Array2D::new(false, n, n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                good[i][j] = true;
                let p1 = a[i];
                let p2 = a[j];
                let mut all_pts = vec![];
                let bb = BoundingBox::new(&p1, &p2);
                for p in a.iter() {
                    if Point::vect_mul(&p1, &p2, p) == 0 {
                        if bb.contains(p) {
                            all_pts.push(p);
                        }
                    }
                }
                all_pts.sort();
                for w in all_pts.windows(2) {
                    let check = Point::new((w[0].x + w[1].x) / 2, (w[0].y + w[1].y) / 2);
                    if point_location.locate_point(check, true).is_none() {
                        good[i][j] = false;
                    }
                }
                for e in poly.edges() {
                    if Point::vect_mul(&p1, &p2, &e.from).signum()
                        * Point::vect_mul(&p1, &p2, &e.to).signum()
                        < 0
                    {
                        if Point::vect_mul(&e.from, &e.to, &p1).signum()
                            * Point::vect_mul(&e.from, &e.to, &p2).signum()
                            < 0
                        {
                            good[i][j] = false;
                        }
                    }
                }
            }
        }
    }
    let pow2 = Mod::gen_powers(Mod::TWO, n + 1);
    let mut res = Mod::ZERO;
    for first_id in 0..n {
        let mut other_ids = vec![];
        for i in first_id + 1..n {
            if good[first_id][i] {
                other_ids.push(i);
            }
        }
        other_ids.sort_by(|&i1, &i2| {
            let d1 = a[first_id].dist2(&a[i1]);
            let d2 = a[first_id].dist2(&a[i2]);
            Point::vect_mul(&a[first_id], &a[i1], &a[i2])
                .cmp(&0)
                .reverse()
                .then(d1.cmp(&d2))
        });
        other_ids.insert(0, first_id);
        let start = a[first_id];
        let sz = other_ids.len();
        let mut dp = Array2D::new(Mod::ZERO, sz, sz);
        for mid in 1..sz {
            let mut on_seg = 0;
            for i in 1..mid {
                let p1 = a[other_ids[0]];
                let p2 = a[other_ids[i]];
                let p3 = a[other_ids[mid]];
                if Point::vect_mul(&p1, &p2, &p3) == 0 {
                    on_seg += 1;
                }
            }
            let my_pow2 = pow2[on_seg];
            for prev in 0..mid {
                if !good[other_ids[prev]][other_ids[mid]] {
                    continue;
                }
                let p1 = a[other_ids[prev]];
                let p2 = a[other_ids[mid]];
                let cur_dp = if prev == 0 { Mod::ONE } else { dp[prev][mid] };
                let real_pow2 = if Point::vect_mul(&p1, &p2, &start) == 0 {
                    Mod::ONE
                } else {
                    my_pow2
                };
                res += cur_dp * real_pow2;
                for next in mid + 1..sz {
                    if good[other_ids[mid]][other_ids[next]] {
                        let p3 = a[other_ids[next]];
                        if Point::vect_mul(&p1, &p2, &p3) >= 0 {
                            dp[mid][next] += cur_dp;
                        }
                    }
                }
            }
        }
        // dbg!(first_id, res);
    }
    // dbg!(slow(&good));

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
