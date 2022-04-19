//{"name":"heroes","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"heroes"}}}

use std::cmp::{max, min, Ordering};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::lcm;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable6, RecursiveFunction6};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let damage = input.i64();
    let health = input.i64();
    let a = input.vec::<i64>(n);
    let res = solve_fast(&a, damage, health);
    out_line!(res);
}

///
/// Works in O(a.len * log^3(damage * health))
///
/// There are O(log) segments for each a_i (hopefully).
/// To find each segment we do a binary search over "angle".
/// Inside each binary serach we calculate number of lattice points inside
/// a triangle, which also works in O(log).
///
fn solve_fast(a: &[i64], damage: i64, health: i64) -> i64 {
    let mut segments = vec![];
    for &cnt in a.iter() {
        segments.append(&mut gen_segments_fast(cnt, health, damage));
    }
    segments.sort();
    let mut combined_seg = Segment::EMPTY;
    for seg in segments.iter() {
        combined_seg = combined_seg.join(seg);
    }
    let start_alive = a.iter().sum::<i64>();
    1 + start_alive * combined_seg.turns - combined_seg.pref_sum
}

fn gen_segments_fast(cnt: i64, health: i64, damage: i64) -> Vec<Segment> {
    let mut segs = vec![];
    let mut from_x = 0;
    let mut a = damage;
    let mut b = health;
    let need_hits = (health * cnt + damage - 1) / damage;
    // f(x) = a/b * x;
    while from_x + 1 < need_hits {
        let p = find_lattice_point_in_triangle(need_hits - 1 - from_x, a, b);

        assert!(p.x > 0);

        let next_x = from_x + p.x;

        let pref_sum = {
            let offset = (from_x * damage) % health + damage;
            div_sum(p.x, offset, damage, health)
        };

        segs.push(Segment {
            total_killed: (next_x * damage) / health - (from_x * damage) / health,
            turns: p.x,
            pref_sum,
        });
        from_x += p.x;
        a = (from_x * damage) / health;
        b = from_x;
    }
    {
        let done = ((need_hits - 1) * damage) / health;
        assert!(done < cnt);
        let left = min(cnt, (need_hits * damage) / health) - done;
        segs.push(Segment {
            total_killed: left,
            turns: 1,
            pref_sum: left,
        });
    }
    while segs.len() >= 2 {
        let n = segs.len();
        if segs[n - 2].cmp(&segs[n - 1]) != Ordering::Less {
            let p1 = segs.pop().unwrap();
            let p2 = segs.pop().unwrap();
            segs.push(p2.join(&p1));
        } else {
            break;
        }
    }
    for w in segs.windows(2) {
        assert!(w[0].cmp(&w[1]) == Ordering::Less);
    }
    assert!(segs.len() < 40);

    segs
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    total_killed: i64,
    turns: i64,
    pref_sum: i64,
}

impl Segment {
    pub fn join(&self, other: &Self) -> Self {
        Self {
            total_killed: self.total_killed + other.total_killed,
            turns: self.turns + other.turns,
            pref_sum: self.pref_sum + other.pref_sum + self.total_killed * other.turns,
        }
    }

    const EMPTY: Self = Self {
        total_killed: 0,
        turns: 0,
        pref_sum: 0,
    };
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.total_killed == other.total_killed
            && self.turns == other.turns
            && self.pref_sum == other.pref_sum
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((other.total_killed * self.turns).cmp(&(self.total_killed * other.turns)))
    }
}

impl Eq for Segment {}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

type Point = PointT<i64>;

///
/// Let's look at all lattice points (x, y) such that:
/// - x > 0
/// - x <= max_x
/// - y <= (a/b) * x
/// - (y/x) -> maximum possible. Over all such points, maximize x.
///
///
fn find_lattice_point_in_triangle(max_x: i64, a: i64, b: i64) -> Point {
    const MAX: i64 = 1.2e9 as i64;
    assert!(max_x < MAX);
    assert!(max_x > 0);
    assert!(a * max_x / b < MAX);
    assert!(a < MAX);
    assert!(b < MAX);
    assert!(b > 0);

    let denum = (max_x * max_x / b + 1) * b;
    assert!(denum >= max_x * max_x);
    assert_eq!(denum % b, 0);

    let max_num = (denum / b) * a;
    assert!(max_num <= MAX * MAX);
    let total_points = num_points_in_triangle(max_x, a, b);
    let total_points_via_denum = num_points_in_triangle(max_x, max_num, denum);
    assert_eq!(total_points, total_points_via_denum);
    let biggest_num = binary_search_first_true(0..max_num + 1, |num| {
        num_points_in_triangle(max_x, num, denum) == total_points
    });
    if biggest_num == 0 {
        return Point { x: max_x, y: 0 };
    }
    let on_line = total_points - num_points_in_triangle(max_x, biggest_num - 1, denum);
    let x = binary_search_first_true(0..max_x + 1, |cur_max_x| {
        num_points_in_triangle(cur_max_x, biggest_num, denum)
            - num_points_in_triangle(cur_max_x, biggest_num - 1, denum)
            == on_line
    });
    Point { x, y: (x * a) / b }
}

///
/// Number of lattice points (x, y)
///
/// 0 <= x <= max_x
/// 0 <= y <= (a/b) * x
///
///
fn num_points_in_triangle(max_x: i64, a: i64, b: i64) -> i64 {
    const MAX: i64 = 1.2e9 as i64;
    assert!(max_x < MAX);
    assert!(a < MAX * MAX);
    assert!(b < MAX * MAX);
    assert!((a as i128) < (MAX * MAX / (max_x + 1)) as i128 * (b as i128));

    div_sum(max_x + 1, 0, a, b)
}

///
/// Copied from: https://github.com/bicsi/kactl/blob/master/content/number-theory/ModSum.h
///
/// Calculates $\sum{i=0}^{to - 1}{(ki+c) / m}$
///
/// TODO: think about i128 here...
fn div_sum(to: i64, c: i64, k: i64, m: i64) -> i64 {
    let to = to as i128;
    let mut c = c as i128;
    let mut k = k as i128;
    let m = m as i128;

    const MAX: i128 = 1.2e9 as i128;
    const MAX2: i128 = MAX * MAX;
    assert!(to < MAX);
    assert!(k / m <= MAX2 / (to + 1) / (to + 1));
    assert!(c / m <= MAX2 / (to + 1));

    fn sum_sq(to: i64) -> i64 {
        to / 2 * ((to - 1) | 1)
    }

    let mut res = k / m * sum_sq(to as i64) as i128 + c / m * to;
    k %= m;
    c %= m;
    if k != 0 {
        // TODO: this overflows (?) i64, so we use i128 :(
        let to2 = (to * k + c) / m;
        assert!(to < MAX2 / (to2 + 1));
        res += to * to2;
        res -= div_sum(to2 as i64, (m - 1 - c) as i64, m as i64, k as i64) as i128 + to2;
    }
    assert!(res < MAX2);
    assert!(res >= 0);
    return res as i64;
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

/*


End of the actual solution. Some slow solutions / stress is below.


*/

fn gen_killed(cnt: i64, damage: i64, health: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut cur_health = cnt * health;
    while cur_health != 0 {
        let cur_cnt = (cur_health + health - 1) / health;
        let next_health = max(0, cur_health - damage);
        let next_cnt = (next_health + health - 1) / health;
        res.push(cur_cnt - next_cnt);
        cur_health = next_health;
    }
    res
}

fn combine_kills(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let m = b.len();

    let a_pref = a.pref_sum();
    let b_pref = b.pref_sum();
    let start_alive = *a_pref.last_exn() + *b_pref.last_exn();

    // dp[i][j] - used `i` items from `a`, `j` items from `b`
    // (what is the smallest amount of damage recieved, prev i, prev j)

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Value {
        damage_recieved: i64,
        prev_i: usize,
        prev_j: usize,
    }

    let mut dp = Array2D::new(
        Value {
            damage_recieved: i64::MAX,
            prev_i: 0,
            prev_j: 0,
        },
        n + 1,
        m + 1,
    );
    dp[0][0] = Value {
        damage_recieved: 0,
        prev_i: 0,
        prev_j: 0,
    };

    for i in 0..=n {
        for j in 0..=m {
            assert!(dp[i][j].damage_recieved != i64::MAX);
            let still_alive = start_alive - a_pref[i] - b_pref[j];
            let expected_damage = dp[i][j].damage_recieved + still_alive;
            if i < n {
                dp[i + 1][j].update_min(Value {
                    damage_recieved: expected_damage - a[i],
                    prev_i: i,
                    prev_j: j,
                });
            }
            if j < m {
                dp[i][j + 1].update_min(Value {
                    damage_recieved: expected_damage - b[j],
                    prev_i: i,
                    prev_j: j,
                });
            }
        }
    }
    let mut res = vec![];
    {
        let mut i = n;
        let mut j = m;
        while i != 0 || j != 0 {
            let val = dp[i][j];
            if val.prev_i != i {
                assert_eq!(val.prev_j, j);
                i = val.prev_i;
                res.push(a[i]);
            } else {
                assert!(val.prev_j != j);
                j = val.prev_j;
                res.push(b[j]);
            }
        }
        res.reverse();
    }
    res
}

#[allow(unused)]
fn solve_slow(a: &[i64], damage: i64, health: i64) -> i64 {
    let kills: Vec<_> = a
        .iter()
        .map(|&cnt| gen_killed(cnt, damage, health))
        .collect();

    let mut combined_kills = vec![];
    for another in kills.into_iter() {
        combined_kills = combine_kills(&combined_kills, &another);
    }

    let mut alive_cnt: i64 = a.iter().sum();
    let mut res = 0;
    for &k in combined_kills.iter() {
        alive_cnt -= k;
        res += alive_cnt
    }
    res + 1
}

fn calc_kills(order: &[&[i64]]) -> i64 {
    let mut cnt = 0;
    for o in order.iter() {
        cnt += o.iter().sum::<i64>();
    }
    let mut res = 0;
    for a in order.iter() {
        for &x in a.iter() {
            cnt -= x;
            res += cnt;
        }
    }
    res
}

fn combine_kills3_slow(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut best_ans = (i64::MAX, vec![]);

    let mut dfs = RecursiveFunction6::new(
        |f,
         a: &[i64],
         b: &[i64],
         cur_kills: i64,
         max_splits_a: usize,
         max_splits_b: usize,
         how: Vec<i64>| {
            if a.len() == 0 && b.len() == 0 {
                best_ans.update_min((cur_kills, how));
                return;
            }
            if max_splits_a == 0 && a.len() != 0 {
                return;
            }
            if max_splits_b == 0 && b.len() != 0 {
                return;
            }
            // if max_splits_a == 1 && a.len() > 1 {
            //     return;
            // }
            // if max_splits_b == 1 && b.len() > 1 {
            //     return;
            // }
            if a.len() != 0 && max_splits_a > 0 {
                for pos in 1..=a.len() {
                    let mut new_how = how.clone();
                    new_how.append(&mut a[0..pos].to_vec());
                    let mut new_kills = cur_kills;
                    let mut cur_alive = a.iter().sum::<i64>() + b.iter().sum::<i64>();
                    for x in a[0..pos].iter() {
                        cur_alive -= x;
                        new_kills += cur_alive;
                    }
                    f.call(
                        &a[pos..],
                        b,
                        new_kills,
                        max_splits_a - 1,
                        max_splits_b,
                        new_how,
                    );
                }
            }
            if b.len() != 0 && max_splits_b > 0 {
                for pos in 1..=b.len() {
                    let mut new_how = how.clone();
                    new_how.append(&mut b[0..pos].to_vec());
                    let mut new_kills = cur_kills;
                    let mut cur_alive = a.iter().sum::<i64>() + b.iter().sum::<i64>();
                    for x in b[0..pos].iter() {
                        cur_alive -= x;
                        new_kills += cur_alive;
                    }
                    f.call(
                        a,
                        &b[pos..],
                        new_kills,
                        max_splits_a,
                        max_splits_b - 1,
                        new_how,
                    );
                }
            }
        },
    );
    dfs.call(a, b, 0, 2, 2, vec![]);
    assert_ne!(best_ans.0, i64::MAX);
    best_ans.1
}

#[allow(unused)]
fn stress3() {
    const MAX: i64 = 600;

    for health in 1..MAX {
        dbg!(health);
        for damage in 1..MAX {
            let a = gen_killed(100, damage, health);
            let a = a[..a.len() - 1].to_vec();
            if a.len() == 0 {
                continue;
            }
            let min_val = *a.iter().min().unwrap();
            let mut exist_two_mins = false;
            let mut exist_two_maxs = false;
            for w in a.windows(2) {
                if w[0] == min_val && w[1] == min_val {
                    exist_two_mins = true;
                }
                if w[0] == min_val + 1 && w[1] == min_val + 1 {
                    exist_two_maxs = false;
                }
            }
            assert!(!exist_two_maxs || !exist_two_mins);
        }
    }
}

#[derive(Debug)]
struct Group {
    value: i64,
    cnt: usize,
}

fn split_groups(a: &[i64]) -> Vec<Group> {
    let mut res = vec![];
    let mut it = 0;
    while it != a.len() {
        let mut it2 = it;
        while it2 != a.len() && a[it2] == a[it] {
            it2 += 1;
        }
        res.push(Group {
            value: a[it],
            cnt: it2 - it,
        });
        it = it2;
    }
    res
}

fn sum_prefix(a: &[i64]) -> i64 {
    let pref_sum = a.pref_sum();
    pref_sum.iter().sum()
}

fn gen_segments(cnt: i64, health: i64, damage: i64) -> Vec<Segment> {
    let mut kills = &gen_killed(cnt, damage, health)[..];
    let mut segs = vec![];
    while kills.len() != 0 {
        let mut last = Segment {
            total_killed: kills[0],
            turns: 1,
            pref_sum: kills[0],
        };
        let mut best = last;
        for &val in &kills[1..] {
            let next = Segment {
                total_killed: last.total_killed + val,
                turns: last.turns + 1,
                pref_sum: last.pref_sum + val + last.total_killed,
            };
            last = next;
            if next.cmp(&best) != Ordering::Greater {
                best = next
            }
        }
        segs.push(best);
        kills = &kills[best.turns as usize..];
    }
    // dbg!(segs);
    assert!(segs.len() < 20);
    segs
}

fn segments_solve(cnt1: i64, cnt2: i64, health: i64, damage: i64) -> i64 {
    let mut segs = vec![];
    segs.append(&mut gen_segments(cnt1, health, damage));
    segs.append(&mut gen_segments(cnt2, health, damage));
    // dbg!(segs);
    segs.sort();
    let mut res = 0;
    let mut cnt_ones = 0;
    for s in segs.iter() {
        res += cnt_ones * s.turns;
        res += s.pref_sum;
        cnt_ones += s.total_killed;
    }
    res
}

#[allow(unused)]
fn stress331() {
    const MAX: i64 = 5000;
    for it in 1600.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let health = rnd.gen_in_range(1..MAX);
        let damage = rnd.gen_in_range(1000..MAX);
        let cnt1 = rnd.gen_in_range(1..MAX / 5);
        let cnt2 = rnd.gen_in_range(1..MAX / 5);
        dbg!(health, damage, cnt1, cnt2);
        let need1 = ((cnt1 * health + damage - 1) / damage) as usize;
        let need2 = ((cnt2 * health + damage - 1) / damage) as usize;
        // dp[where last hit?][segs1][segs2][hit1][hit2] -> sum prefix killed, try to maximize
        const MAX_SEGS: usize = 5;
        let mut dp =
            vec![vec![vec![vec![vec![i64::MIN; need2 + 1]; need1 + 1]; MAX_SEGS]; MAX_SEGS]; 2];
        dp[0][1][0][0][0] = 0;
        dp[1][0][1][0][0] = 0;
        for hit1 in 0..=need1 {
            for hit2 in 0..=need2 {
                for seg1 in 0..MAX_SEGS {
                    for seg2 in 0..MAX_SEGS {
                        for last in 0..2 {
                            let cur = dp[last][seg1][seg2][hit1][hit2];
                            if cur == i64::MIN {
                                continue;
                            }
                            for next in 0..2 {
                                if hit1 == need1 && next == 0 {
                                    continue;
                                }
                                if hit2 == need2 && next == 1 {
                                    continue;
                                }
                                let nhit1 = hit1 + (if next == 0 { 1 } else { 0 });
                                let nhit2 = hit2 + (if next == 1 { 1 } else { 0 });
                                let nseg1 = seg1 + (if last == 1 && next == 0 { 1 } else { 0 });
                                let nseg2 = seg2 + (if last == 0 && next == 1 { 1 } else { 0 });
                                if nseg1 >= MAX_SEGS || nseg2 >= MAX_SEGS {
                                    continue;
                                }
                                let killed1 = min(cnt1, (nhit1 as i64) * damage / health);
                                let killed2 = min(cnt2, (nhit2 as i64) * damage / health);

                                dp[next][nseg1][nseg2][nhit1][nhit2]
                                    .update_max(cur + killed1 + killed2);
                            }
                        }
                    }
                }
            }
        }
        // store [maximum prefix sums; -max(seg1, seg2)]
        let mut best_res = (0, 0);
        for cur in 0..2 {
            for seg1 in 0..MAX_SEGS {
                for seg2 in 0..MAX_SEGS {
                    let dp = dp[cur][seg1][seg2][need1][need2];
                    if dp == i64::MIN {
                        continue;
                    }
                    let max_segs = (max(seg1, seg2) as i32) * -1;
                    best_res.update_max((dp, max_segs));
                }
            }
        }
        let segs_used = best_res.1 * -1;
        if segs_used > 3 {
            dbg!(best_res);
        }
        assert!(segs_used <= 3);
        // dbg!(best_res);
        // let correct = {
        //     let a = gen_killed(cnt1, damage, health);
        //     let b = gen_killed(cnt2, damage, health);
        //     let optimal = combine_kills(&a, &b);
        //     sum_prefix(&optimal)
        // };
        // if correct != best_res {
        //     assert_eq!(correct, best_res);
        // }
    }
}

#[allow(unused)]
fn stress84846723() {
    let mut max_segs = 0;
    const MAX: i64 = 50000000;
    for it in 1600.. {
        let mut rnd = Random::new(it);
        let health = rnd.gen_in_range(1..MAX);
        let damage = rnd.gen_in_range(10000..MAX);
        let cnt = rnd.gen_in_range(1..MAX / 10);
        let need1 = ((cnt * health + damage - 1) / damage) as usize;

        if need1 > 10_000_000 {
            dbg!("skip");
            continue;
        }
        dbg!(it);
        dbg!(health, damage, cnt);

        let segs = gen_segments(cnt, health, damage);
        max_segs.update_max(segs.len());
        dbg!(segs.len(), max_segs);
    }
}

fn find_lattice_point_in_triangle_slow(max_x: i64, a: i64, b: i64) -> Point {
    let mut best = Point { x: 1, y: 0 };
    for x in 1..=max_x {
        let y = a * x / b;
        let p = Point { x, y };
        if Point::vect_mul2(&best, &p) >= 0 {
            best = p;
        }
    }
    best
}

#[allow(unused)]
fn stress42255() {
    for it in 21607.. {
        let mut rnd = Random::new(it);
        const MAX: i64 = 1_000_000_000;
        let max_x = rnd.gen_in_range(1..MAX);
        let a = rnd.gen_in_range(1..MAX);
        let b = rnd.gen_in_range(1..MAX);
        if a * max_x / b > 1_000_000_000 {
            continue;
        }
        dbg!(it);
        let my = find_lattice_point_in_triangle(max_x, a, b);
        let correct = find_lattice_point_in_triangle_slow(max_x, a, b);
        assert_eq!(my, correct);
    }
}

#[allow(unused)]
fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX: i64 = 1000000;
        let cnt = rnd.gen_in_range(1..MAX);
        let health = rnd.gen_in_range(1..MAX);
        let damage = rnd.gen_in_range(1..MAX);
        let correct_segs = gen_segments(cnt, health, damage);
        let my_segs = gen_segments_fast(cnt, health, damage);
        assert_eq!(my_segs, correct_segs);
    }
}

#[allow(unused)]
fn stress883() {
    const MAX: i64 = 50000;
    for it in 1600.. {
        let mut rnd = Random::new(it);
        let health = rnd.gen_in_range(1..MAX);
        let damage = rnd.gen_in_range(10000..MAX);
        let cnt1 = rnd.gen_in_range(1..MAX / 10);
        let cnt2 = rnd.gen_in_range(1..MAX / 10);
        let need1 = ((cnt1 * health + damage - 1) / damage) as usize;
        let need2 = ((cnt2 * health + damage - 1) / damage) as usize;

        if need1 * need2 > 1_000_000 {
            dbg!("skip");
            continue;
        }
        dbg!(it);
        dbg!(health, damage, cnt1, cnt2);

        // dbg!(best_res);
        let correct = {
            let a = gen_killed(cnt1, damage, health);
            let b = gen_killed(cnt2, damage, health);
            let optimal = combine_kills(&a, &b);
            sum_prefix(&optimal)
        };
        let my = segments_solve(cnt1, cnt2, health, damage);
        assert_eq!(correct, my);
        // if correct != best_res {
        //     assert_eq!(correct, best_res);
        // }
    }
}

#[allow(unused)]
fn stress33() {
    const MAX: i64 = 200;

    for health in 1..MAX {
        dbg!(health);
        for damage in 1..health {
            let cycle = lcm(health, damage);
            let cnt = cycle / health;
            if cnt == 1 {
                continue;
            }
            let a = gen_killed(cnt, damage, health);
            let groups = split_groups(&a);

            // dbg!(damage, cycle, cnt);
            // dbg!(groups);

            for i in (0..groups.len()).step_by(2) {
                assert_eq!(groups[i].value, groups[0].value);
            }
            for i in (1..groups.len()).step_by(2) {
                assert_eq!(groups[i].value, groups[1].value);
            }
            assert_eq!(groups[0].value + 1, groups[1].value);
            for start_pos in 1..a.len() {
                let mut sum_start = 0;
                let mut sum_suf = 0;
                let max_len = a.len() - start_pos;
                for i in 0..max_len {
                    sum_start += a[i];
                    sum_suf += a[i + start_pos];
                    assert!(sum_suf >= sum_start);
                }
            }
            let mut lens = vec![];
            let mut sum_len = 0;
            for i in 0..groups.len() / 2 {
                sum_len += groups[i * 2].cnt;
                sum_len += groups[i * 2 + 1].cnt;
                lens.push(sum_len);
            }
            for &l1 in lens.iter() {
                for &l2 in lens.iter() {
                    let first = &a[0..l1];
                    let second = &a[0..l2];
                    let best = combine_kills(first, second);
                    let best_ans = sum_prefix(&best);
                    let mut my = 0;
                    for (o1, o2) in [(first, second), (second, first)].iter() {
                        let mut joined = vec![];
                        joined.append(&mut o1.to_vec());
                        joined.append(&mut o2.to_vec());
                        let cur = sum_prefix(&joined);
                        my.update_max(cur);
                    }
                    if best_ans != my {
                        dbg!(first);
                        dbg!(second);
                        dbg!(damage);
                        dbg!(a);
                        assert_eq!(best_ans, my);
                    }
                }
            }
        }
    }
}

#[allow(unused)]
fn stress2() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it + 7782228);
        const MAX_CNT: i64 = 20;
        const MAX_HEALTH: i64 = 20;
        let cnt1 = rnd.gen_in_range(1..MAX_CNT);
        let cnt2 = rnd.gen_in_range(1..MAX_CNT);
        let health = rnd.gen_in_range(2..MAX_HEALTH);
        let damage = rnd.gen_in_range(1..MAX_HEALTH);
        let cycle = lcm(health, damage);
        let a = gen_killed(cnt1, damage, health);
        let b = gen_killed(cnt2, damage, health);
        if cnt1 * health >= cycle || cnt2 * health >= cycle {
            continue;
        }
        dbg!(a.len(), b.len());
        if a.len() > 25 || b.len() > 25 {
            continue;
        }
        // let a_splits = split(&a);
        // let b_splits = split(&b);
        // let mut first_parts = vec![];
        // let mut second_parts = vec![];
        // for zz in [a_splits, b_splits].iter() {
        //     first_parts.push(zz[0].to_vec());
        //     if zz.len() == 1 {
        //     } else {
        //         assert_eq!(zz.len(), 2);
        //         second_parts.push(zz[1].to_vec());
        //     }
        // // }
        // first_parts.push(a.clone());
        // first_parts.push(b.clone());
        // let cmp = |a: &Vec<i64>, b: &Vec<i64>| -> Ordering {
        //     let c1 = calc_kills(&[a, b]);
        //     let c2 = calc_kills(&[b, a]);
        //     c1.cmp(&c2)
        // };
        // first_parts.sort_by(cmp);
        // second_parts.sort_by(cmp);
        // let mut all = vec![];
        // for f in [&first_parts, &second_parts].iter() {
        //     for x in f.iter() {
        //         all.push(&x[..]);
        //     }
        // }
        // let my = calc_kills(&all);
        let my = combine_kills3_slow(&a, &b);
        let my_ans = calc_kills(&[&my]);
        let optimal = combine_kills(&a, &b);
        let optimal_ans = calc_kills(&[&optimal]);
        if my_ans != optimal_ans {
            dbg!(cnt1, cnt2, health, damage);
            dbg!(a);
            dbg!(b);
            dbg!(optimal);
            dbg!(my);
            dbg!(optimal_ans);
            assert_eq!(my_ans, optimal_ans);
        }
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
