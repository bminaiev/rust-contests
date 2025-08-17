//{"name":"reply2025","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"reply2025"}}}

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

#[derive(Clone)]
struct Event {
    modulo: usize,
    from_mod: usize,
    cnt: usize,
    delta: i64,
}

struct MagicArray {
    existing_mods: Vec<usize>,
    per_mod: Vec<Vec<i64>>,
    events: Vec<Vec<Event>>,
    cur_time: usize,
}

impl MagicArray {
    pub fn new(max_time: usize, max_mod: usize) -> Self {
        Self {
            existing_mods: vec![],
            per_mod: vec![vec![]; max_mod],
            events: vec![vec![]; max_time + 1],
            cur_time: 0,
        }
    }

    fn apply_event(&mut self, ev: &Event) {
        if ev.from_mod + ev.cnt <= ev.modulo {
            for i in ev.from_mod..ev.from_mod + ev.cnt {
                self.per_mod[ev.modulo][i] += ev.delta;
            }
        } else {
            for i in ev.from_mod..ev.modulo {
                self.per_mod[ev.modulo][i] += ev.delta;
            }
            for i in 0..(ev.from_mod + ev.cnt) % ev.modulo {
                self.per_mod[ev.modulo][i] += ev.delta;
            }
        }
    }

    pub fn add(&mut self, from: usize, till: usize, modulo: usize, cnt: usize, delta: i64) {
        if !self.existing_mods.contains(&modulo) {
            self.existing_mods.push(modulo);
            self.per_mod[modulo] = vec![0; modulo];
        }
        let till = till.min(self.events.len() - 1);
        let mut event = Event {
            modulo,
            from_mod: from % modulo,
            cnt,
            delta,
        };
        self.apply_event(&event);
        event.delta = -event.delta;
        assert!(self.cur_time <= till);
        self.events[till].push(event);
    }

    pub fn get_value(&mut self, time: usize) -> i64 {
        // assert!(self.cur_time <= time);
        while self.cur_time <= time {
            let events = self.events[self.cur_time].clone();
            for ev in events {
                self.apply_event(&ev);
            }
            self.cur_time += 1;
        }
        let mut res = 0;
        for &mods in self.existing_mods.iter() {
            res += self.per_mod[mods][time % mods];
        }
        res
    }
}

#[derive(Clone, Copy, Debug)]
struct Resource {
    activation_cost: i64,
    turn_cost: i64,
    active_for: usize,
    down_for: usize,
    total_alive: usize,
    buildings: i64,
    special_effect: char,
    effect_value: i64,
}

impl Resource {
    pub fn modulo(&self) -> usize {
        self.active_for + self.down_for
    }
}

#[derive(Clone, Copy)]
struct Turn {
    min_buildings: i64,
    max_buildings: i64,
    profit_per_building: i64,
}

#[derive(Clone)]
struct Test {
    start_money: i64,
    resources: Vec<Resource>,
    turns: Vec<Turn>,
}

impl Test {
    pub fn max_time(&self) -> usize {
        self.turns.len()
    }

    pub fn max_mod(&self) -> usize {
        let mut res = 0;
        for r in self.resources.iter() {
            let time = r.active_for + r.down_for;
            res = res.max(time);
        }
        res + 1
    }
}

struct Emulator {
    t: Test,
    used_resources: Vec<Vec<usize>>,
    sum_buildings: MagicArray,
    money: i64,
    cur_turn: usize,
    maintenance: MagicArray,
    score: i64,
    building_num_perc_delta: MagicArray,
    profit_perc_delta: MagicArray,
    coverted_perc_delta: MagicArray,
    life_perc_delta: MagicArray,
}

fn apply_percent(value: i64, percent: i64) -> i64 {
    let offset = if percent > 0 {
        (value * percent) / 100
    } else {
        -((-value * percent) / 100)
    };
    value + offset
}

const MAX_RESOURCES_PER_TURN: usize = 50;

impl Emulator {
    pub fn new(t: Test) -> Self {
        Self {
            used_resources: vec![vec![]; t.turns.len()],
            sum_buildings: MagicArray::new(t.max_time(), t.max_mod()),
            building_num_perc_delta: MagicArray::new(t.max_time(), t.max_mod()),
            profit_perc_delta: MagicArray::new(t.max_time(), t.max_mod()),
            coverted_perc_delta: MagicArray::new(t.max_time(), t.max_mod()),
            life_perc_delta: MagicArray::new(t.max_time(), t.max_mod()),
            money: t.start_money,
            maintenance: MagicArray::new(t.max_time(), 2),
            t,
            cur_turn: 0,
            score: 0,
        }
    }

    pub fn print(&self, out: &mut Output) {
        for time in 0..self.t.turns.len() {
            let r = &self.used_resources[time];
            if !r.is_empty() {
                out.print(time);
                out.print(" ");
                out.print(r.len());
                out.print(" ");
                out.println(r.to_vec());
            }
        }
    }

    pub fn buy(&mut self, resource_id: usize, extra_life_perc: i64, cnt: i64) {
        self.used_resources[self.cur_turn].extend(vec![resource_id; cnt as usize]);
        assert!(self.used_resources[self.cur_turn].len() <= MAX_RESOURCES_PER_TURN);
        let r = &self.t.resources[resource_id];
        self.money -= r.activation_cost;
        assert!(self.money >= 0);
        let total_alive = apply_percent(
            r.total_alive as i64,
            self.life_perc_delta.get_value(self.cur_turn) + extra_life_perc,
        ) as usize;
        let till = (self.cur_turn + total_alive).min(self.t.turns.len());
        if r.special_effect == 'B' {
            self.building_num_perc_delta.add(
                self.cur_turn,
                till,
                r.modulo(),
                r.active_for,
                cnt * r.effect_value,
            );
        } else if r.special_effect == 'D' {
            self.profit_perc_delta.add(
                self.cur_turn,
                till,
                r.modulo(),
                r.active_for,
                cnt * r.effect_value,
            );
        } else if r.special_effect == 'A' {
            self.coverted_perc_delta.add(
                self.cur_turn,
                till,
                r.modulo(),
                r.active_for,
                cnt * r.effect_value,
            );
        } else if r.special_effect == 'C' {
            self.life_perc_delta.add(
                self.cur_turn,
                till,
                r.modulo(),
                r.active_for,
                cnt * r.effect_value,
            );
        }
        self.sum_buildings.add(
            self.cur_turn,
            till,
            r.modulo(),
            r.active_for,
            cnt * r.buildings,
        );
        self.maintenance
            .add(self.cur_turn, till, 1, 1, r.turn_cost * cnt);
        // for time in self.cur_turn..till {
        // self.maintenance[time] += r.turn_cost * cnt;
        // let offset = (time - self.cur_turn) % (r.active_for + r.down_for);
        // if offset < r.active_for {
        //     // self.sum_buildings[time] += r.buildings * cnt;
        //     if r.special_effect == 'B' {
        //         // self.building_num_perc_delta[time] += r.effect_value * cnt;
        //     } else if r.special_effect == 'D' {
        //         // self.profit_perc_delta[time] += r.effect_value * cnt;
        //     } else if r.special_effect == 'A' {
        //         // self.coverted_perc_delta[time] += r.effect_value * cnt;
        //     } else if r.special_effect == 'C' {
        //         if time != self.cur_turn {
        //             // self.life_perc_delta[time] += r.effect_value * cnt;
        //         }
        //     }
        // }
        // }
    }

    pub fn buy_many(&mut self, resource_ids: &[usize]) {
        let mut resource_ids = resource_ids.to_vec();
        resource_ids.sort();

        let mut extra_life_perc = 0;
        for &id in resource_ids.iter() {
            let r = &self.t.resources[id];
            if r.special_effect == 'C' {
                extra_life_perc += r.effect_value;
            }
        }
        let mut i = 0;
        while i < resource_ids.len() {
            let mut cnt = 1;
            while i + cnt < resource_ids.len() && resource_ids[i] == resource_ids[i + cnt] {
                cnt += 1;
            }
            self.buy(resource_ids[i], extra_life_perc, cnt as i64);
            let r = &self.t.resources[resource_ids[i]];
            if r.special_effect == 'C' {
                extra_life_perc -= r.effect_value * cnt as i64;
            }
            i += cnt;
        }
        assert!(extra_life_perc == 0);
        // self.life_perc_delta[self.cur_turn] += extra_life_perc;
    }

    pub fn can_buy_more(&self) -> usize {
        MAX_RESOURCES_PER_TURN - self.used_resources[self.cur_turn].len()
    }

    fn print_stats(&self) {
        let mut s = String::new();
        for resource_id in 0..self.t.resources.len() {
            let mut cnt = 0;
            for used in self.used_resources[self.cur_turn].iter() {
                if *used == resource_id {
                    cnt += 1;
                }
            }
            if cnt > 0 {
                s = format!("{s} {}:{} ", resource_id, cnt);
            }
        }
        dbg!(s);
    }

    fn get_turn_buildings(&mut self) -> (i64, i64) {
        let min_buildings = apply_percent(
            self.t.turns[self.cur_turn].min_buildings,
            self.building_num_perc_delta.get_value(self.cur_turn),
        );
        let max_buildings = apply_percent(
            self.t.turns[self.cur_turn].max_buildings,
            self.building_num_perc_delta.get_value(self.cur_turn),
        );
        (min_buildings, max_buildings)
    }

    pub fn next_turn(&mut self) {
        let mut buildings = self.sum_buildings.get_value(self.cur_turn);
        // dbg!(buildings);
        buildings = apply_percent(buildings, self.coverted_perc_delta.get_value(self.cur_turn));
        let (min_buildings, max_buildings) = self.get_turn_buildings();
        if buildings < min_buildings {
            buildings = 0;
        }
        if buildings > max_buildings {
            buildings = max_buildings;
        }
        let profit_per_building = apply_percent(
            self.t.turns[self.cur_turn].profit_per_building,
            self.profit_perc_delta.get_value(self.cur_turn),
        );
        let profit = buildings * profit_per_building;
        self.score += profit;
        self.money += profit;

        self.money -= self.maintenance.get_value(self.cur_turn);
        assert!(self.money >= 0);

        self.cur_turn += 1;
    }
}

fn read_test(input: &mut Input) -> Test {
    let start_money = input.i64();
    let n_resources = input.usize();
    let n_turns = input.usize();
    dbg!(start_money, n_resources, n_turns);

    let mut resources = Vec::with_capacity(n_resources);
    for i in 0..n_resources {
        let resource_id = input.usize();
        assert_eq!(i, resource_id);
        let activation_cost = input.i64();
        let turn_cost = input.i64();
        let active_for = input.usize();
        let down_for = input.usize();
        let total_alive = input.usize();
        let buildings = input.i64();
        let special_effect = input.string()[0] as char;
        let effect_value = if special_effect != 'X' {
            assert!(('A'..='E').contains(&special_effect));
            input.i64()
        } else {
            0
        };
        if ('C'..='E').contains(&special_effect) {
            assert_eq!(buildings, 0);
        }
        resources.push(Resource {
            activation_cost,
            turn_cost,
            active_for,
            down_for,
            total_alive,
            buildings,
            special_effect,
            effect_value,
        });
        dbg!(
            i,
            resource_id,
            effect_value,
            turn_cost,
            activation_cost,
            buildings,
            total_alive,
            special_effect
        );
    }

    let mut max_score = 0;
    let mut turns = vec![];
    for i in 0..n_turns {
        let min_buildings = input.i64();
        let max_buildings = input.i64();
        let profit_per_building = input.i64();
        turns.push(Turn {
            min_buildings,
            max_buildings,
            profit_per_building,
        });
        max_score += profit_per_building * max_buildings;
    }
    dbg!(max_score);
    Test {
        start_money,
        resources,
        turns,
    }
}

fn read_tourist_solution(t: &Test, test_case: usize) -> Vec<Vec<usize>> {
    let mut file = Input::new_file(format!("reply-outputs/{test_case}_tourist_best_old.txt"));
    let mut res = vec![vec![]; t.turns.len()];
    while file.has_more_elements() {
        let time = file.usize();
        let n = file.usize();
        for _ in 0..n {
            res[time].push(file.usize());
        }
        if time == t.turns.len() - 1 {
            break;
        }
    }
    res
}

fn human_readable(x: i64) -> String {
    let zz = 1e12;

    format!("{:.1} * 10^12", x as f64 / zz)
}

fn score(test: &Test, tourist: &[Vec<usize>]) -> i64 {
    let mut emulator = Emulator::new(test.clone());
    for time in 0..test.turns.len() {
        emulator.buy_many(&tourist[time]);
        emulator.next_turn();
    }
    emulator.score
}

fn save(tourist: &[Vec<usize>], test_case: usize, expected_score: i64) {
    let mut file = Output::new_file(format!("reply-outputs/{test_case}_tourist_best.txt"));
    for time in 0..tourist.len() {
        file.print(time);
        file.print(" ");
        file.print(tourist[time].len());
        file.print(" ");
        file.println(tourist[time].to_vec());
    }
    file.flush();
    let mut f2 = Output::new_file(format!("reply-outputs/{test_case}_tourist_score.txt"));
    f2.println(expected_score);
    f2.flush();
}

fn solve(input: &mut Input, out: &mut Output, test_id: usize) {
    let t = read_test(input);
    let mut emulator = Emulator::new(t.clone());

    let mut tourist = read_tourist_solution(&t, test_id);

    let mut all_types = BTreeSet::new();
    for time in 0..t.turns.len() {
        for &id in tourist[time].iter() {
            all_types.insert(id);
        }
    }
    let all_types: Vec<usize> = all_types.into_iter().collect();
    dbg!(all_types);

    let mut cur_score = score(&t, &tourist);
    let mut rnd = Random::new(234234);
    let start = Instant::now();
    for _iter in 0..100_000_000_0 {
        if _iter % 1000 == 0 {
            dbg!(_iter, cur_score, start.elapsed());
            save(&tourist, test_id, cur_score);
        }
        let pos = rnd.gen(50..tourist.len());
        let pos2 = rnd.gen(0..tourist[pos].len());
        let new_val = all_types[rnd.gen(0..all_types.len())];
        let prev_val = tourist[pos][pos2];
        tourist[pos][pos2] = new_val;

        let n_score = score(&t, &tourist);
        if n_score > cur_score {
            let delta = n_score - cur_score;
            cur_score = n_score;
            // dbg!("IMPROVED", human_readable(delta), cur_score);
        } else {
            tourist[pos][pos2] = prev_val;
        }
    }

    if test_id == 8 {
        // let ID_INCR_BUILDINGS = 21;
        // let ID_INCR_PROFIT = 20;
        // let start = Instant::now();
        // for time in 0..t.turns.len() {
        //     emulator.buy_many(&tourist[time]);
        //     emulator.next_turn();
        // }

        // dbg!(human_readable(emulator.score));
        // dbg!(start.elapsed());
    } else if test_id == 7 {
        //     const ID_INCR_BUILDINGS: usize = 30;
        //     const GOOD_B: usize = 33;
        //     for time in 0..t.turns.len() {
        //         loop {
        //             let (min_buildings, max_buildings) = emulator.get_turn_buildings();
        //             if emulator.money < 2000 {
        //                 break;
        //             }
        //             let cur_b = emulator.sum_buildings[time];
        //             let need_buildings = max_buildings - cur_b;
        //             if need_buildings
        //                 > (emulator.can_buy_more() as i64 - 1)
        //                     * t.resources[ID_INCR_BUILDINGS].buildings
        //             {
        //                 break;
        //             }

        //             if emulator.can_buy_more() == 0 {
        //                 break;
        //             }

        //             if emulator.money < t.resources[GOOD_B].activation_cost {
        //                 break;
        //             }
        //             emulator.buy(GOOD_B);
        //         }
        //         loop {
        //             let (min_buildings, max_buildings) = emulator.get_turn_buildings();
        //             let cur_b = emulator.sum_buildings[time];
        //             if cur_b > max_buildings {
        //                 break;
        //             }
        //             if emulator.money < t.resources[ID_INCR_BUILDINGS].activation_cost {
        //                 break;
        //             }
        //             if emulator.can_buy_more() == 0 {
        //                 break;
        //             }
        //             emulator.buy(ID_INCR_BUILDINGS);
        //         }
        //         emulator.next_turn();
        //     }
        // } else if test_id == 6 {
        //     emulator.next_turn();

        //     const ID_INCR_BUILDINGS: usize = 27;
        //     const ID_INCR_PROFIT: usize = 28;
        //     for time in 1..t.turns.len() {
        //         loop {
        //             let (min_buildings, max_buildings) = emulator.get_turn_buildings();
        //             let cur_b = emulator.sum_buildings[time];
        //             if cur_b > max_buildings {
        //                 break;
        //             }
        //             if emulator.money < t.resources[ID_INCR_BUILDINGS].activation_cost {
        //                 break;
        //             }
        //             emulator.buy(ID_INCR_BUILDINGS);
        //         }
        //         loop {
        //             if emulator.can_buy_more() == 0 {
        //                 break;
        //             }
        //             if emulator.money < t.resources[ID_INCR_PROFIT].activation_cost {
        //                 break;
        //             }
        //             emulator.buy(ID_INCR_PROFIT);
        //         }
        //         emulator.next_turn();
        // }
    }
    dbg!(emulator.score);
    emulator.print(out);
}

pub(crate) fn run(mut input: Input, mut output: Output, test_id: usize) -> bool {
    solve(&mut input, &mut output, test_id);
    output.flush();
    true
}

fn single_test(test_id: usize) {
    dbg!("TEST", test_id);
    let input = Input::new_file(format!("reply-inputs/{}.txt", test_id));
    let output = Output::new_file(format!("reply-outputs/{}.txt", test_id));

    run(input, output, test_id);
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "reply2025";
    use algo_lib::tester::helper::*;

    std::fs::copy(
        "/home/borys/rust-contests/main/src/main.rs",
        "/home/borys/rust-contests/reply-outputs/main.rs",
    )
    .unwrap();

    for test_id in [1, 2, 4, 6, 7, 8] {
        if test_id != 8 {
            continue;
        }
        single_test(test_id);
    }

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
