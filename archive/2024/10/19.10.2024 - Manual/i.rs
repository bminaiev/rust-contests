//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

fn inside_triangle(p: Point, tr: &[Point; 3]) -> bool {
    let mut s = 0;
    for i in 0..3 {
        let j = (i + 1) % 3;
        let a = tr[i];
        let b = tr[j];
        if Point::vect_mul(&a, &b, &p) < 0 {
            s -= 1;
        } else {
            s += 1;
        }
    }
    s == 3 || s == -3
}

#[derive(Clone, Copy)]
struct Edge {
    w: i32,
    mask: i32,
    left_edges: i32,
}

fn solve_case(a_base: &[Point], d_base: &Array2D<i32>) -> DP {
    let n = a_base.len();
    let mut rnd = Random::new(123123);
    let c1 = rnd.gen_double();
    let c2 = rnd.gen_double();
    let mut perm = gen_vec(n, |i| i);
    let f = |i: usize| c1 * a_base[i].x as f64 + c2 * a_base[i].y as f64;
    perm.sort_by(|&i, &j| f(i).partial_cmp(&f(j)).unwrap());
    let mut a = a_base.to_vec();
    for i in 0..n {
        a[i] = a_base[perm[i]];
    }
    // for i in 0..n {
    //     dbg!(a[i]);
    // }
    // dbg!("!!");
    let d = Array2D::new_f(n, n, |i, j| d_base[perm[i]][perm[j]]);
    let is_good_triangle = gen_vec(n, |i| {
        let mut res = Array2D::new(false, n, n);
        for j in i + 1..n {
            for k in j + 1..n {
                res[j][k] = true;
                for l in 0..n {
                    if l != i && l != j && l != k {
                        if inside_triangle(a[l], &[a[i], a[j], a[k]]) {
                            res[j][k] = false;
                            break;
                        }
                    }
                }
            }
        }
        res
    });
    let vvv = gen_vec(n, |i| {
        let mut res = Array2D::new(false, n, n);
        for j in 0..n {
            for k in 0..n {
                res[j][k] = Point::vect_mul(&a[i], &a[j], &a[k]) > 0;
            }
        }
        res
    });
    // for i in 0..n {
    //     for j in i + 1..n {
    //         for k in j + 1..n {
    //             dbg!(i, j, k, is_good_triangle[i][j][k]);
    //         }
    //     }
    // }

    let start_mask = {
        let mut res = 1 + (1 << (n - 1));
        let mut first = 0;
        loop {
            let mut next = n - 1;
            for i in first + 1..n {
                if Point::vect_mul(&a[first], &a[next], &a[i]) < 0 {
                    next = i;
                }
            }
            res |= 1 << first;
            if next == n - 1 {
                break;
            }
            first = next;
        }
        res
    };
    let end_mask = {
        let mut res = 1 + (1 << (n - 1));
        let mut first = n - 1;
        loop {
            let mut next = 0;
            for i in 0..first {
                if Point::vect_mul(&a[first], &a[next], &a[i]) < 0 {
                    next = i;
                }
            }
            res |= 1 << first;
            if next == 0 {
                break;
            }
            first = next;
        }
        res
    };
    let mut g = Array2D::new(vec![], 1 << n, n);
    // {mask, cnt_edges_not_change}
    for mask in 0i32..1 << n {
        if (mask & 1) == 0 {
            continue;
        }
        if (mask & (1 << (n - 1))) == 0 {
            continue;
        }
        let mut vert = vec![];
        for i in 0..n {
            if (1 << i) & mask != 0 {
                vert.push(i);
            }
        }
        let cnt_v = mask.count_ones() as i32;
        for left_edges in 0..cnt_v {
            if left_edges != cnt_v - 1 {
                g[mask as usize][left_edges as usize].push(Edge {
                    w: 0,
                    mask,
                    left_edges: left_edges + 1,
                });
                if left_edges != 0 {
                    // remove vertex vert[left_edges]
                    let mut new_mask = mask;
                    let v = vert[left_edges as usize];
                    new_mask ^= 1 << v;
                    if is_good_triangle[vert[left_edges as usize - 1]][vert[left_edges as usize]]
                        [vert[left_edges as usize + 1]]
                    {
                        if !vvv[vert[left_edges as usize - 1]][vert[left_edges as usize + 1]][v] {
                            g[mask as usize][left_edges as usize].push(Edge {
                                w: d[vert[left_edges as usize - 1]][vert[left_edges as usize + 1]],
                                mask: new_mask,
                                left_edges: left_edges - 1,
                            });
                        }
                    }
                }
                for k in vert[left_edges as usize] + 1..vert[left_edges as usize + 1] {
                    if is_good_triangle[vert[left_edges as usize]][k][vert[left_edges as usize + 1]]
                        && Point::vect_mul(
                            &a[vert[left_edges as usize]],
                            &a[vert[left_edges as usize + 1]],
                            &a[k],
                        ) > 0
                    {
                        g[mask as usize][left_edges as usize].push(Edge {
                            w: d[vert[left_edges as usize]][k]
                                + d[k][vert[left_edges as usize + 1]],
                            mask: mask ^ (1 << k),
                            left_edges,
                        });
                    }
                }
            }
        }
    }
    // topological sort
    let mut cnt_inner = Array2D::new(0, 1 << n, n);
    let mut queue = vec![];
    for mask in 0..1 << n {
        if (mask & 1) == 0 {
            continue;
        }
        if (mask & (1 << (n - 1))) == 0 {
            continue;
        }
        for cnt in 0..n {
            for e in g[mask][cnt].iter() {
                // dbg!(mask, cnt, e.mask, e.left_edges);
                cnt_inner[e.mask as usize][e.left_edges as usize] += 1;
            }
        }
    }
    for mask in 0..1 << n {
        if (mask & 1) == 0 {
            continue;
        }
        if (mask & (1 << (n - 1))) == 0 {
            continue;
        }
        for cnt in 0..n {
            if cnt_inner[mask][cnt] == 0 {
                queue.push((mask as i32, cnt as i32));
            }
        }
    }
    let mut it = 0;
    while it < queue.len() {
        let (mask, cnt) = queue[it];
        it += 1;
        for e in g[mask as usize][cnt as usize].iter() {
            cnt_inner[e.mask as usize][e.left_edges as usize] -= 1;
            if cnt_inner[e.mask as usize][e.left_edges as usize] == 0 {
                queue.push((e.mask, e.left_edges));
            }
        }
    }
    // dbg!(queue.len());
    let mut dp = Array2D::new(
        DP {
            ways: 0,
            min_cost: i32::MAX,
        },
        1 << n,
        n,
    );
    let mut start_cost = 0;
    {
        let mut vertices = vec![];
        for i in 0..n {
            if (1 << i) & start_mask != 0 {
                vertices.push(i);
            }
        }
        for i in 1..vertices.len() {
            start_cost += d[vertices[i - 1]][vertices[i]];
        }
    }
    dp[start_mask][0] = DP {
        ways: 1,
        min_cost: start_cost,
    };
    // dbg!(queue.len());
    // dbg!(start_mask);
    for it in 0..queue.len() {
        let (mask, cnt) = queue[it];
        let cur_dp = dp[mask as usize][cnt as usize];
        // dbg!(it, cur_dp, mask, cnt);
        if cur_dp.ways == 0 {
            continue;
        }
        // dbg!(mask, cnt, cur_dp.ways, cur_dp.min_cost);
        for e in g[mask as usize][cnt as usize].iter() {
            let new_dp = DP {
                ways: cur_dp.ways,
                min_cost: cur_dp.min_cost + e.w,
            };
            if new_dp.min_cost < dp[e.mask as usize][e.left_edges as usize].min_cost {
                dp[e.mask as usize][e.left_edges as usize] = new_dp;
            } else if new_dp.min_cost == dp[e.mask as usize][e.left_edges as usize].min_cost {
                dp[e.mask as usize][e.left_edges as usize].ways += new_dp.ways;
            }
        }
    }
    // dbg!(end_mask);
    let res = dp[end_mask][end_mask.count_ones() as usize - 1];
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a_base = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
        let d_base = Array2D::new_f(n, n, |_, _| input.i32());
        let res = solve_case(&a_base, &d_base);
        out.println(vec![res.min_cost as i64, res.ways]);
        // if 2 + 2 == 4 {
        //     break;
        // }
    }
}

#[derive(Clone, Copy, Debug)]
struct DP {
    ways: i64,
    min_cost: i32,
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

fn stress() {
    for it in 1.. {
        dbg!(it);

        let n = 18;
        let mut rnd = Random::new(123123);
        let a_base = gen_vec(n, |_| Point::new(rnd.gen(-100..100), rnd.gen(-100..100)));
        let d_base = Array2D::new_f(n, n, |_, _| rnd.gen(1..100));
        let start = Instant::now();
        let res = solve_case(&a_base, &d_base);
        dbg!(start.elapsed());
    }
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
