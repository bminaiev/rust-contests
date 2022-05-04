//{"name":"hash-code-test","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hash-code-test"}}}

use std::cmp::min;
use std::collections::HashMap;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use marathon_utils::{
    distribution_stat::DistributionStat,
    hashcode_solver::{hashcode_solver, OneTest},
};

mod visualizer;

#[derive(Clone, Copy, Debug)]
pub struct AccRange {
    max_w: i64,
    max_ac: i64,
}

type Point = PointT<i64>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Accelerate(Point),
    Float(i64),
    LoadCarrots(i64),
    LoadGift(usize),
    DeliverGift(usize),
}

impl Operation {
    pub fn print(&self, gifts: &[Gift]) {
        match self {
            Operation::Accelerate(ac) => {
                if ac.x > 0 {
                    out_line!("AccRight", ac.x.abs())
                } else if ac.x < 0 {
                    out_line!("AccLeft", ac.x.abs());
                } else if ac.y > 0 {
                    out_line!("AccUp", ac.y.abs());
                } else if ac.y < 0 {
                    out_line!("AccDown", ac.y.abs());
                } else {
                    unreachable!();
                }
            }
            Operation::Float(len) => out_line!("Float", *len),
            Operation::LoadCarrots(cnt) => out_line!("LoadCarrots", *cnt),
            Operation::LoadGift(id) => out_line!("LoadGift", vec2str(&gifts[*id].child_name)),
            Operation::DeliverGift(id) => out_line!("DeliverGift", vec2str(&gifts[*id].child_name)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Path {
    ops: Vec<Operation>,
    final_v: Point,
    final_pos: Point,
    time: i64,
    use_carrots: i64,
}

fn simulate(mut pos: Point, mut v: Point, ops: &[Operation]) -> (Point, Point) {
    for op in ops.iter() {
        if let Operation::Float(len) = op {
            pos.x += v.x * *len;
            pos.y += v.y * *len;
        }
        if let Operation::Accelerate(delta) = op {
            v += *delta;
        }
    }
    (pos, v)
}

fn find_path_fixed_time(
    start_pos: Point,
    end_pos: Point,
    range2: i64,
    start_v: Point,
    max_ac: i64,
    sum_time: i64,
) -> Option<Path> {
    if start_pos.dist2(&end_pos) <= range2 {
        return Some(Path {
            ops: vec![],
            final_v: start_v,
            time: 0,
            use_carrots: 0,
            final_pos: start_pos,
        });
    }
    let mut cur_dx = start_pos.x + sum_time * start_v.x - end_pos.x;
    let mut cur_dy = start_pos.y + sum_time * start_v.y - end_pos.y;
    let mut ops = vec![];
    let mut use_carrots = 0;
    for time in 0..sum_time {
        let mul = sum_time - time;
        // let mul = mul * (mul + 1) / 2;
        if cur_dx.abs() > cur_dy.abs() {
            let mut use_ac = cur_dx / mul;
            if use_ac > max_ac {
                use_ac = max_ac;
            } else if use_ac < -max_ac {
                use_ac = -max_ac;
            }
            if use_ac != 0 {
                cur_dx -= use_ac * mul;
                ops.push(Operation::Accelerate(Point::new(-use_ac, 0)));
                use_carrots += 1;
            }
        } else {
            let mut use_ac = cur_dy / mul;
            if use_ac > max_ac {
                use_ac = max_ac;
            } else if use_ac < -max_ac {
                use_ac = -max_ac;
            }
            if use_ac != 0 {
                cur_dy -= use_ac * mul;
                ops.push(Operation::Accelerate(Point::new(0, -use_ac)));
                use_carrots += 1;
            }
        }
        ops.push(Operation::Float(1));
    }
    if cur_dx * cur_dx + cur_dy * cur_dy <= range2 {
        let mut final_v = start_v;
        for op in ops.iter() {
            if let Operation::Accelerate(ac) = op {
                final_v += *ac;
            }
        }
        let final_pos = Point::new(end_pos.x + cur_dx, end_pos.y + cur_dy);
        let (exp_pos, exp_v) = simulate(start_pos, start_v, &ops);

        assert_eq!(exp_pos, final_pos);
        assert_eq!(exp_v, final_v);
        return Some(Path {
            ops,
            final_v,
            time: sum_time,
            use_carrots,
            final_pos,
        });
    }
    None
}

fn find_path_fastest(
    start_pos: Point,
    end_pos: Point,
    range2: i64,
    start_v: Point,
    max_ac: i64,
    max_time: i64,
) -> Option<Path> {
    for sum_time in 0..max_time {
        if let Some(path) =
            find_path_fixed_time(start_pos, end_pos, range2, start_v, max_ac, sum_time)
        {
            return Some(path);
        }
    }
    None
}

fn find_path_slow_end(
    start_pos: Point,
    end_pos: Point,
    range2: i64,
    start_v: Point,
    max_ac: i64,
    max_time: i64,
) -> Option<Path> {
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Way {
        v2: i64,
        path: Path,
    }

    let mut ways = vec![];

    for sum_time in 0..max_time {
        if let Some(path) =
            find_path_fixed_time(start_pos, end_pos, range2, start_v, max_ac, sum_time)
        {
            let v2 = path.final_v.x * path.final_v.x + path.final_v.y * path.final_v.y;
            ways.push(Way { path, v2 });
        }
    }
    if ways.is_empty() {
        return None;
    }
    let way = ways.iter().min().unwrap();
    Some(way.path.clone())
}

fn find_path(
    start_pos: Point,
    end_pos: Point,
    range2: i64,
    start_v: Point,
    max_ac: i64,
    rnd: &mut Random,
    max_time: i64,
) -> Option<Path> {
    if start_pos.dist2(&end_pos) <= range2 {
        return Some(Path {
            ops: vec![],
            final_v: start_v,
            time: 0,
            use_carrots: 0,
            final_pos: start_pos,
        });
    }

    let mut best: Option<Path> = None;

    for _ in 0..5 {
        let delta = (end_pos.x - start_pos.x).abs() / 5 + (end_pos.y - start_pos.y).abs() / 5;
        let mid = Point::new(
            (start_pos.x + end_pos.x) / 2 + rnd.gen_in_range(-delta..delta),
            (start_pos.y + end_pos.y) / 2 + rnd.gen_in_range(-delta..delta),
        );
        if let Some(mut p1) = find_path_fastest(start_pos, mid, range2, start_v, max_ac, max_time) {
            let used_time = p1.time;
            let more = max_time - p1.time;
            if let Some(mut p2) = find_path_slow_end(
                p1.final_pos,
                end_pos,
                range2,
                p1.final_v,
                max_ac,
                min(more, used_time + rnd.gen_in_range(5..25)),
            ) {
                let mut ops = vec![];
                ops.append(&mut p1.ops);
                ops.append(&mut p2.ops);
                let zz = Path {
                    ops,
                    final_v: p2.final_v,
                    time: p1.time + p2.time,
                    use_carrots: p1.use_carrots + p2.use_carrots,
                    final_pos: p2.final_pos,
                };
                if best.is_none() || best.clone().unwrap().time > zz.time {
                    best = Some(zz);
                }
            }
        }
    }

    best
}

pub struct Gift {
    id: usize,
    child_name: Vec<u8>,
    score: i64,
    weight: i64,
    p: Point,
}

pub struct TestInfo {
    pub gifts: Vec<Gift>,
    pub acc_ranges: Vec<AccRange>,
    pub range2: i64,
    pub range: i64,
    pub max_time: i64,
}

fn solve(input: &mut Input, test: &mut OneTest) {
    let max_time = input.i64();
    let range = input.i64();
    let range2 = range * range;
    let w = input.usize();
    let n_gifts = input.usize();

    let acc_ranges = gen_vec(w, |_| AccRange {
        max_w: input.read(),
        max_ac: input.read(),
    });

    for r in acc_ranges.iter() {
        dbg!(r);
    }

    let gifts = gen_vec(n_gifts, |id| Gift {
        id,
        child_name: input.string(),
        score: input.read(),
        weight: input.read(),
        p: Point::new(input.read(), input.read()),
    });

    let t = TestInfo {
        gifts,
        acc_ranges,
        range2,
        range,
        max_time,
    };

    test.report.add_value("max_time", &max_time);
    test.report.add_value("range", &range);
    test.report.add_value("n gifts", &t.gifts.len());
    test.report
        .add_value("last range w", &t.acc_ranges.last_exn().max_w);
    test.report.add_value("max ac", &t.acc_ranges[0].max_ac);

    let max_score = t.gifts.iter().map(|g| g.score).sum::<i64>();
    test.report.add_value("max score", &max_score);

    {
        let mut ds = DistributionStat::new("Score by gift");
        for g in t.gifts.iter() {
            ds.add(g.score as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("Weight by gift");
        for g in t.gifts.iter() {
            ds.add(g.weight as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("x");
        for g in t.gifts.iter() {
            ds.add(g.p.x as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("y");
        for g in t.gifts.iter() {
            ds.add(g.p.y as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    let mut rnd = Random::new_time_seed();
    let mut sol = None;
    test.load_existing_result(|mut input: Input| {
        let n_ops = input.usize();
        let mut child_id_by_name = HashMap::new();
        for g in t.gifts.iter() {
            child_id_by_name.insert(g.child_name.clone(), g.id);
        }
        let mut score = 0;
        let ops = gen_vec(n_ops, |_| {
            let op_type = input.string_as_string();
            if op_type == "AccUp" {
                Operation::Accelerate(Point::new(0, input.i64()))
            } else if op_type == "AccDown" {
                Operation::Accelerate(Point::new(0, input.i64() * -1))
            } else if op_type == "AccLeft" {
                Operation::Accelerate(Point::new(input.i64() * -1, 0))
            } else if op_type == "AccRight" {
                Operation::Accelerate(Point::new(input.i64(), 0))
            } else if op_type == "Float" {
                Operation::Float(input.read())
            } else if op_type == "LoadCarrots" {
                Operation::LoadCarrots(input.read())
            } else if op_type == "LoadGift" {
                let child_name = input.string();
                Operation::LoadGift(*child_id_by_name.get(&child_name).unwrap())
            } else if op_type == "DeliverGift" {
                let child_name = input.string();
                let id = *child_id_by_name.get(&child_name).unwrap();
                score += t.gifts[id].score;
                Operation::DeliverGift(id)
            } else {
                unreachable!();
            }
        });
        sol = Some(Solution { score, ops });
    });
    let sol = sol.unwrap();

    let save = |sol: &Solution| {
        test.save_result(&mut || {
            out_line!(sol.ops.len());
            for op in sol.ops.iter() {
                op.print(&t.gifts);
            }
        });
    };

    save(&sol);

    // let mut best_score = sol.score;

    // for it in 0.. {
    //     let sol2 = gen_sol(&mut rnd, &gifts, &acc_ranges, range2, max_time);
    //     if sol2.score > sol.score {
    //         dbg!(it, sol2.score);
    //         sol = sol2;
    //         save(&sol);
    //         best_score.update_max(sol.score);
    //     }
    //     dbg!(best_score);
    // }

    test.report.add_value("expected score", &sol.score);

    dbg!("best score!", sol.score);

    visualizer::visualize(sol, &test.name, t);
    // test.load_existing_result(|mut input: Input| {});
}

pub struct Solution {
    score: i64,
    pub ops: Vec<Operation>,
}

fn gen_sol(rnd: &mut Random, t: &TestInfo) -> Solution {
    let max_ac_by_w = |w: i64| -> i64 {
        let id =
            binary_search_first_true(0..t.acc_ranges.len(), |idx| t.acc_ranges[idx].max_w >= w);
        if id == t.acc_ranges.len() {
            return 0;
        }
        return t.acc_ranges[id].max_ac;
    };

    let mut prefix_ops = vec![];
    let mut suffix_ops = vec![];

    let mut delivered = vec![false; t.gifts.len()];
    let mut cur_time = 0;
    let mut cur_pos = Point::ZERO;
    let mut cur_v = Point::ZERO;
    // TODO: not the best option
    let mut cur_sum_w = t.acc_ranges[0].max_w;
    let mut total_carrots = 0;

    let max_v = rnd.gen_in_range(20..200);
    let max_v2 = max_v * max_v;

    let mut first = true;
    let mut mult = 1;

    while mult < 4 {
        let max_ac = max_ac_by_w(cur_sum_w);

        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
        struct Way {
            v2: i64,
            time: i64,
            gift_id: usize,
            path: Path,
            type_: i32,
        }

        let mut ways = vec![];
        let one_max_time = if first { 200 } else { 20 * (1 << mult) };
        for gift_id in 0..t.gifts.len() {
            if delivered[gift_id] {
                continue;
            }

            let gift = &t.gifts[gift_id];

            let d2 = gift.p.dist2(&cur_pos);
            if !first && d2 > 10000000 {
                continue;
            }
            if let Some(path) =
                find_path(cur_pos, gift.p, t.range2, cur_v, max_ac, rnd, one_max_time)
            {
                if path.use_carrots + gift.weight <= cur_sum_w && cur_time + path.time <= t.max_time
                {
                    ways.push(Way {
                        v2: path.final_v.x * path.final_v.x + path.final_v.y * path.final_v.y,
                        gift_id,
                        time: path.time,
                        path,
                        type_: 0,
                    });
                }
            }
            if let Some(path) =
                find_path_fastest(cur_pos, gift.p, t.range2, cur_v, max_ac, one_max_time)
            {
                if path.use_carrots + gift.weight <= cur_sum_w && cur_time + path.time <= t.max_time
                {
                    ways.push(Way {
                        v2: path.final_v.x * path.final_v.x + path.final_v.y * path.final_v.y,
                        gift_id,
                        time: path.time,
                        path,
                        type_: 1,
                    });
                }
            }
            if let Some(path) = find_path_slow_end(cur_pos, gift.p, t.range2, cur_v, max_ac, 7) {
                if path.use_carrots + gift.weight <= cur_sum_w && cur_time + path.time <= t.max_time
                {
                    ways.push(Way {
                        v2: path.final_v.x * path.final_v.x + path.final_v.y * path.final_v.y,
                        gift_id,
                        time: path.time,
                        path,
                        type_: 2,
                    });
                }
            }
        }
        if ways.is_empty() {
            if cur_time + (1 << mult) * 20 > t.max_time {
                break;
            }
            mult += 1;

            continue;
        }
        let w = ways
            .iter()
            .min_by_key(|w| {
                if w.v2 > max_v2 {
                    return std::i64::MAX;
                }
                w.time * (1e15 as i64) / t.gifts[w.gift_id].score
            })
            .unwrap();
        let gift = &t.gifts[w.gift_id];
        let path = w.path.clone();
        total_carrots += path.use_carrots;
        prefix_ops.push(Operation::LoadGift(gift.id));
        delivered[gift.id] = true;
        for op in path.ops.iter() {
            suffix_ops.push(op.clone());
        }
        suffix_ops.push(Operation::DeliverGift(gift.id));
        cur_time += path.time;
        cur_pos = path.final_pos;
        cur_v = path.final_v;
        cur_sum_w -= gift.weight;
        cur_sum_w -= path.use_carrots;
        first = false;
        dbg!(
            "delivered gift!",
            gift.score,
            cur_time,
            cur_sum_w,
            cur_v,
            cur_pos,
            w.type_
        );
    }

    let mut res = vec![];

    if total_carrots != 0 {
        res.push(Operation::LoadCarrots(total_carrots));
    }
    res.append(&mut prefix_ops);
    res.append(&mut suffix_ops);

    let mut expected_score = 0;
    for op in res.iter() {
        if let Operation::DeliverGift(gift_id) = op {
            expected_score += t.gifts[*gift_id].score;
        }
    }

    dbg!(expected_score);

    Solution {
        score: expected_score,
        ops: res,
    }
}

pub(crate) fn run(mut _input: Input) -> bool {
    let task = b'f';
    hashcode_solver(
        &"hash-code-2022-finals",
        &"inputs",
        &"outputs",
        task..=task,
        &mut solve,
    );
    true
}

#[allow(unused)]
pub fn submit() {
    let sin = std::io::stdin();
    let input = Input::new(Box::new(sin));
    set_global_output_to_stdout();
    run(input);
}

//START MAIN
mod tester;

fn main() {
    tester::run_locally();
}
//END MAIN
