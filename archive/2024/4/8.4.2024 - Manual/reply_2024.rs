//{"name":"reply_2024","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"reply_2024"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Write;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::{geometry::point::PointT, io::input::Input};
use marathon_utils::distribution_stat::DistributionStat;
use marathon_utils::hashcode_solver::{hashcode_solver, OneTest};

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let x = input.i32();
    let y = input.i32();
    out.println(x + y);
}

type Point = PointT<usize>;

#[derive(Clone, Copy)]
struct Silver {
    pos: Point,
    score: i64,
}

#[derive(Clone)]
struct Tile {
    mask: usize,
    cost: i64,
    num_tiles: usize,
}

#[derive(Clone, Copy, Default, Debug)]
struct TileMask {
    cost: i64,
    num: usize,
}

struct Test {
    w: usize,
    h: usize,
    silver: Vec<Silver>,
    gold: Vec<Point>,
    tiles: [TileMask; 16],
}

fn solve2(input: &mut Input, test: &mut OneTest) {
    dbg!("start");
    // let ww = input.string_as_string();
    // dbg!(ww);
    let w = input.usize();
    dbg!(w);
    let h = input.usize();
    let num_gold = input.usize();
    let num_silver = input.usize();
    let num_tiles = input.usize();

    let golden = gen_vec(num_gold, |_| Point::new(input.read(), input.read()));
    let silver = gen_vec(num_silver, |_| Silver {
        pos: Point::new(input.read(), input.read()),
        score: input.i64(),
    });
    let tiles = gen_vec(num_tiles, |_| {
        let tile_name = input.string()[0];
        let mask = if tile_name <= b'9' {
            tile_name - b'0'
        } else {
            10 + (tile_name - b'A')
        };

        Tile {
            mask: mask as usize,
            cost: input.i64(),
            num_tiles: input.usize(),
        }
    });

    test.report.add_value("w", &w);
    test.report.add_value("h", &h);
    test.report.add_value("num_gold", &num_gold);
    test.report.add_value("num_silver", &num_silver);
    test.report.add_value("num_tiles", &num_tiles);

    let total_tiles_cost = tiles
        .iter()
        .map(|t| t.cost * t.num_tiles as i64)
        .sum::<i64>();
    test.report.add_value("total_tiles_cost", &total_tiles_cost);

    let sum_silver_scores = silver.iter().map(|s| s.score).sum::<i64>();
    let gold_div2 = num_gold / 2;
    let coef = gold_div2 * (num_gold - gold_div2);
    let total_max_score = coef as i64 * sum_silver_scores;
    test.report.add_value("total_max_score", &total_max_score);

    for tile in tiles.iter() {
        test.report
            .add_value(&format!("tile_{}_cost", tile.mask), &tile.cost);
        test.report
            .add_value(&format!("tile_{}_num", tile.mask), &tile.num_tiles);
    }

    {
        let mut ds = DistributionStat::new("silver X");
        for &s in silver.iter() {
            ds.add(s.pos.x as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("silver Y");
        for &s in silver.iter() {
            ds.add(s.pos.y as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("silver score");
        for &s in silver.iter() {
            ds.add(s.score as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    let mut field = Array2D::new(b'.', h, w);
    for g in golden.iter() {
        field[g.y][g.x] = b'G';
    }
    for s in silver.iter() {
        field[s.pos.y][s.pos.x] = b'S';
    }

    let mut tiles_mask = [TileMask::default(); 16];
    for tile in tiles.iter() {
        tiles_mask[tile.mask].cost = tile.cost;
        tiles_mask[tile.mask].num = tile.num_tiles;
    }

    let mut cur_test = Test {
        w,
        h,
        silver,
        gold: golden,
        tiles: tiles_mask,
    };

    let res = solve_test(&cur_test);
    for tile in res.iter() {
        field[tile.pos.y][tile.pos.x] = conv_mask(tile.tile_mask);
    }

    // test.load_existing_result(|mut input: Input| {
    // });
    test.save_result(&mut |output| {
        {
            let mut output = Output::new_file("reply_2024/outputs/viz.txt");
            for y in 0..h {
                let line = &field[y];
                output.println(String::from_utf8_lossy(line).to_string());
            }
            output.flush();
        }

        for tile in res.iter() {
            let ss = String::from_utf8_lossy(&[conv_mask(tile.tile_mask)]).to_string();
            output.println(format!("{ss} {} {}", tile.pos.x, tile.pos.y));
        }
    });
}

fn conv_mask(mask: usize) -> u8 {
    if mask < 10 {
        (b'0' + mask as u8)
    } else {
        (b'A' + (mask - 10) as u8)
    }
}

#[derive(Clone, Copy)]
struct ResTile {
    tile_mask: usize,
    pos: Point,
}

fn nei(a: Point, b: Point) -> bool {
    (a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs() == 1
}

struct Path {
    used: Array2D<bool>,
    path: Vec<Point>,
    bfs_seen: Array2D<i32>,
    bfs_len: Array2D<i64>,
    bfs_color: i32,
    bfs_prev: Array2D<Point>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    len_cost: i64,
    pos: Point,
}

impl Path {
    fn add(&mut self, p: Point) {
        self.path.push(p);
        assert!(!self.used[p.y][p.x]);
        self.used[p.y][p.x] = true;
    }

    fn new(w: usize, h: usize) -> Self {
        Path {
            used: Array2D::new(false, h, w),
            path: vec![],
            bfs_seen: Array2D::new(0, h, w),
            bfs_color: 0,
            bfs_prev: Array2D::new(Point::new(0, 0), h, w),
            bfs_len: Array2D::new(0, h, w),
        }
    }

    fn bfs(
        &mut self,
        stop: &mut impl FnMut(Point) -> bool,
        ok_border: usize,
    ) -> Option<Vec<Point>> {
        self.bfs_color += 1;
        let color = self.bfs_color;
        let start = *self.path.last().unwrap();
        self.bfs_prev[start.y][start.x] = start;
        self.bfs_seen[start.y][start.x] = color;
        self.bfs_len[start.y][start.x] = 0;
        let mut pq = BinaryHeap::new();
        pq.push(Reverse(Vertex {
            len_cost: 0,
            pos: start,
        }));

        let should_skip = |p: Point| -> bool {
            p.x <= ok_border
                || p.x + ok_border >= self.used.cols()
                || p.y <= ok_border
                || p.y + ok_border >= self.used.rows()
        };

        while let Some(vertex) = pq.pop() {
            let p = vertex.0.pos;
            if self.bfs_len[p.y][p.x] != vertex.0.len_cost {
                continue;
            }
            if should_skip(p) {
                continue;
            }
            if p != start && stop(p) {
                let mut res = vec![];
                let mut cur = p;
                while cur != *self.path.last().unwrap() {
                    res.push(cur);
                    cur = self.bfs_prev[cur.y][cur.x];
                }
                res.reverse();
                return Some(res);
            }
            let prev = self.bfs_prev[p.y][p.x];
            let cur = p;
            const ON_LINE_COST: i64 = 5;
            // TODO: on test
            if p.x + 1 < self.used.cols() && !self.used[p.y][p.x + 1] {
                let np = Point::new(p.x + 1, p.y);
                let on_line = on_line(prev, cur, np);
                let len_cost = (if on_line { ON_LINE_COST } else { 1 }) + self.bfs_len[p.y][p.x];
                if self.bfs_seen[p.y][p.x + 1] != color || self.bfs_len[p.y][p.x + 1] > len_cost {
                    self.bfs_seen[p.y][p.x + 1] = color;
                    self.bfs_len[p.y][p.x + 1] = len_cost;
                    self.bfs_prev[p.y][p.x + 1] = p;
                    pq.push(Reverse(Vertex { pos: np, len_cost }));
                }
            }
            if p.x > 0 && !self.used[p.y][p.x - 1] {
                let np = Point::new(p.x - 1, p.y);
                let on_line = on_line(prev, cur, np);
                let len_cost = (if on_line { ON_LINE_COST } else { 1 }) + self.bfs_len[p.y][p.x];
                if self.bfs_seen[p.y][p.x - 1] != color || self.bfs_len[p.y][p.x - 1] > len_cost {
                    self.bfs_seen[p.y][p.x - 1] = color;
                    self.bfs_len[p.y][p.x - 1] = len_cost;
                    self.bfs_prev[p.y][p.x - 1] = p;
                    pq.push(Reverse(Vertex { pos: np, len_cost }));
                }
            }
            if p.y + 1 < self.used.rows() && !self.used[p.y + 1][p.x] {
                let np = Point::new(p.x, p.y + 1);
                let on_line = on_line(prev, cur, np);
                let len_cost = (if on_line { ON_LINE_COST } else { 1 }) + self.bfs_len[p.y][p.x];
                if self.bfs_seen[p.y + 1][p.x] != color || self.bfs_len[p.y + 1][p.x] > len_cost {
                    self.bfs_seen[p.y + 1][p.x] = color;
                    self.bfs_len[p.y + 1][p.x] = len_cost;
                    self.bfs_prev[p.y + 1][p.x] = p;
                    pq.push(Reverse(Vertex { pos: np, len_cost }));
                }
            }
            if p.y > 0 && !self.used[p.y - 1][p.x] {
                let np = Point::new(p.x, p.y - 1);
                let on_line = on_line(prev, cur, np);
                let len_cost = (if on_line { ON_LINE_COST } else { 1 }) + self.bfs_len[p.y][p.x];
                if self.bfs_seen[p.y - 1][p.x] != color || self.bfs_len[p.y - 1][p.x] > len_cost {
                    self.bfs_seen[p.y - 1][p.x] = color;
                    self.bfs_len[p.y - 1][p.x] = len_cost;
                    self.bfs_prev[p.y - 1][p.x] = p;
                    pq.push(Reverse(Vertex { pos: np, len_cost }));
                }
            }
        }
        None
    }

    fn extend_line(&mut self, to: Point) {
        let start = *self.path.last().unwrap();
        dbg!("extend", start, to);
        if start.y == to.y {
            if start.x < to.x {
                for x in start.x + 1..=to.x {
                    self.add(Point::new(x, start.y));
                }
            } else {
                for x in (to.x..start.x).rev() {
                    self.add(Point::new(x, start.y));
                }
            }
        } else {
            assert_eq!(start.x, to.x);
            if start.y < to.y {
                for y in start.y + 1..=to.y {
                    self.add(Point::new(start.x, y));
                }
            } else {
                for y in (to.y..start.y).rev() {
                    self.add(Point::new(start.x, y));
                }
            }
        }
    }

    fn to_res_tiles(&self, test: &Test) -> Vec<ResTile> {
        let h = self.used.rows();
        let w = self.used.cols();
        let mut need_mask = Array2D::new(0, h, w);
        let mut add = |from: Point, to: Point| {
            if from.x < to.x {
                need_mask[from.y][from.x] |= 1;
            }
            if from.x > to.x {
                need_mask[from.y][from.x] |= 2;
            }
            if from.y < to.y {
                need_mask[from.y][from.x] |= 4;
            }
            if from.y > to.y {
                need_mask[from.y][from.x] |= 8;
            }
        };
        for w in self.path.windows(2) {
            add(w[0], w[1]);
            add(w[1], w[0]);
        }

        let mut now_tiles = test.tiles.clone();
        let mut tiles_cost = 0;

        let mut res = vec![];

        for x in 0..w {
            for y in 0..h {
                if need_mask[y][x] == 0 {
                    continue;
                }
                let is_gold = test.gold.iter().any(|g| g == &Point::new(x, y));
                if is_gold {
                    continue;
                }
                let need = need_mask[y][x];
                let mut allowed_mask = 0;
                if x + 1 == w || !self.used[y][x + 1] {
                    allowed_mask |= 1;
                }
                if x == 0 || !self.used[y][x - 1] {
                    allowed_mask |= 2;
                }
                if y + 1 == h || !self.used[y + 1][x] {
                    allowed_mask |= 4;
                }
                if y == 0 || !self.used[y - 1][x] {
                    allowed_mask |= 8;
                }
                allowed_mask |= need;
                // dbg!(x, y, need, allowed_mask);
                let mut found = false;
                let mut best = (usize::MAX, i64::MAX);
                for mask in 0..now_tiles.len() {
                    if mask & allowed_mask != mask {
                        continue;
                    }
                    if (mask & need) != need {
                        continue;
                    }
                    if now_tiles[mask].num == 0 {
                        continue;
                    }
                    let cur_score = (mask, now_tiles[mask].cost);
                    if cur_score.1 > best.1 {
                        continue;
                    }
                    best = cur_score;
                    // break;
                }
                assert!(best.0 != usize::MAX);
                {
                    let mask = best.0;

                    now_tiles[mask].num -= 1;
                    tiles_cost += now_tiles[mask].cost;
                    res.push(ResTile {
                        tile_mask: mask,
                        pos: Point::new(x, y),
                    });
                }
            }
        }
        dbg!(tiles_cost);

        for mask in 0..16 {
            if test.tiles[mask].num == 0 {
                continue;
            }
            dbg!(now_tiles[mask].num, test.tiles[mask]);
        }
        res
    }
}

fn on_line(prev: Point, cur: Point, next: Point) -> bool {
    if prev.x == cur.x && cur.x == next.x {
        return true;
    }
    if prev.y == cur.y && cur.y == next.y {
        return true;
    }
    false
}

const START_V: usize = 2;
const CNT: usize = 755;

fn solve_test(test: &Test) -> Vec<ResTile> {
    let mut gold = test.gold.clone();
    // gold.sort_by_key(|g| g.x);
    // let mut first_half = gold[..gold.len() / 2].to_vec();
    // let mut second_half = gold[gold.len() / 2..].to_vec();
    // first_half.sort_by_key(|g| (usize::MAX - g.y, usize::MAX - g.x));
    // second_half.sort_by_key(|g| (g.y, usize::MAX - g.x));

    // let mut all_y: Vec<usize> = gold.iter().map(|g| g.y).collect();
    // all_y.sort();
    // dbg!(all_y);

    // dbg!(first_half);
    // dbg!(second_half);

    let mut path = Path::new(test.w, test.h);
    // path.add(first_half[0]);
    // path.extend_line(Point::new(0, first_half[0].y));
    // for i in 1..first_half.len() {
    //     if first_half[..i].iter().any(|p| p.y == first_half[i].y) {
    //         continue;
    //     }
    //     let p = first_half[i];
    //     path.extend_line(Point::new(0, p.y));
    //     path.extend_line(p);
    //     path.extend_line(Point::new(p.x, p.y - 1));
    //     path.extend_line(Point::new(0, p.y - 1));
    // }
    // path.extend_line(Point::new(0, 0));
    // path.extend_line(Point::new(test.w - 1, 0));
    // for i in 0..second_half.len() {
    //     let p = second_half[i];
    //     path.extend_line(Point::new(test.w - 1, p.y));
    //     path.extend_line(p);
    //     if i + 1 == second_half.len() {
    //         continue;
    //     }
    //     path.extend_line(Point::new(p.x, p.y + 1));
    //     path.extend_line(Point::new(test.w - 1, p.y + 1));
    // }
    let mut scores = Array2D::new(0, test.h, test.w);
    for g in gold.iter() {
        scores[g.y][g.x] = i64::MAX;
    }
    for s in test.silver.iter() {
        scores[s.pos.y][s.pos.x] = s.score;
    }
    path.add(gold[START_V]);
    let first_gold_size = gold.len() / 2 - 1;
    let mut first_half_used = 1;
    for _ in 0..first_gold_size {
        if let Some(extra_path) = path.bfs(&mut |p| scores[p.y][p.x] == i64::MAX, 0) {
            // dbg!(extra_path);
            first_half_used += 1;
            for p in extra_path {
                path.add(p);
            }
        } else {
            break;
        }
    }

    // let can_find_gold =
    //     |path: &mut Path| -> bool { path.bfs(&mut |p| scores[p.y][p.x] == i64::MAX).is_some() };

    let mut sum_used_silver_score = 0;
    let mut rnd = Random::new(7877332);
    for _cnt in 0..CNT {
        let mut found = false;
        let zz = rnd.gen(10..100);
        for ok_border in (zz..zz + 20).rev() {
            if let Some(extra_path) = path.bfs(
                &mut |p| scores[p.y][p.x] != i64::MAX && scores[p.y][p.x] != 0,
                ok_border,
            ) {
                dbg!(ok_border);
                let last = extra_path.last().unwrap();
                sum_used_silver_score += scores[last.y][last.x];
                for p in extra_path {
                    path.add(p);
                }
                found = true;
                break;
            }
        }
        if !found {
            dbg!("Stopped", _cnt);
            break;
        }
        // if !can_find_gold(&mut path) {
        //     dbg!("OOOPS...");
        //     break;
        // }
    }

    let mut second_half_used = 0;
    loop {
        if let Some(extra_path) = path.bfs(&mut |p| scores[p.y][p.x] == i64::MAX, 0) {
            second_half_used += 1;
            for p in extra_path {
                path.add(p);
            }
        } else {
            break;
        }
    }

    dbg!(first_half_used, second_half_used, sum_used_silver_score);
    let mut expected_good_score =
        (first_half_used * second_half_used) as i64 * sum_used_silver_score;

    dbg!(expected_good_score);

    path.to_res_tiles(test)
}

fn stress() {
    hashcode_solver("reply_2024", "inputs", "outputs", b'3'..=b'3', &mut solve2);
    // for test in 1..=1 {
    //     dbg!("TEST", test);
    //     let mut input = Input::new_file(format!("reply_2024/inputs/{test}.in"));
    //     let mut output = Output::new_file(format!("reply_2024/outputs/{test}.out"));

    //     solve(&mut input, &mut output, test);

    //     output.flush();
    // }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

#[test]
fn test() {
    let width = 10;
    let height = 5;
    let row = 4usize;
    let col = 9usize;
    for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let nrow = row.overflowing_add(dr).0;
        let ncol = col.overflowing_add(dc).0;
        if nrow < height && ncol < width {
            dbg!(nrow, ncol);
        }
    }
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "reply_2024";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
