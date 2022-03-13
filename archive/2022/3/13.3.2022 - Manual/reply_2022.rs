//{"name":"reply_2022","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"reply_2022"}}}

use std::cmp::min;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
use algo_lib::{dbg, out, out_line};
use marathon_utils::distribution_stat::DistributionStat;
use marathon_utils::hashcode_solver::{hashcode_solver, OneTest};

struct Demon {
    need_health: i32,
    time_to_recover: usize,
    add_health: i32,
    sum_points_by_day: Vec<i32>,
    total_points: i32,
}

impl Demon {
    fn score_by_day(&self, more_days: usize) -> i32 {
        if more_days >= self.sum_points_by_day.len() {
            self.total_points
        } else {
            self.sum_points_by_day[more_days]
        }
    }
}

struct Task<'a> {
    demons: &'a [Demon],
    health_start: i32,
    health_max: i32,
    turns: usize,
}

#[derive(Debug)]
struct Answer {
    score: i32,
    demons_killed: usize,
}

fn calc_score(perm: &[usize], t: &Task) -> Answer {
    let mut health = t.health_start;
    let mut score = 0;
    let mut time = 0;
    let mut iter = 0;
    let mut add_health_by_day = vec![0; t.turns];
    while time != t.turns && iter != perm.len() {
        health += add_health_by_day[time];
        health = min(health, t.health_max);
        let d = &t.demons[perm[iter]];
        if d.need_health <= health {
            health -= d.need_health;
            let more_days = t.turns - time;
            score += d.score_by_day(more_days);
            if d.time_to_recover + time < t.turns {
                add_health_by_day[time + d.time_to_recover] += d.add_health;
            }
            iter += 1;
        }
        time += 1;
    }
    Answer {
        demons_killed: iter,
        score,
    }
}

fn solve(input: &mut Input, test: &mut OneTest) {
    let health_start = input.i32();
    let health_max = input.i32();
    let turns = input.usize();
    let num_demons = input.usize();

    test.report.add_value("start health", &health_start);
    test.report.add_value("max health", &health_max);
    test.report.add_value("turns", &turns);
    test.report.add_value("num demons", &num_demons);

    let demons = gen_vec(num_demons, |_| {
        let need_health = input.i32();
        let time_to_recover = input.usize();
        let add_health = input.i32();
        let num_turns = input.usize();
        let sum_points_by_day = input.vec(num_turns).pref_sum();
        Demon {
            need_health,
            time_to_recover,
            add_health,
            total_points: *sum_points_by_day.last_exn(),
            sum_points_by_day,
        }
    });

    let t = Task {
        demons: &demons,
        health_start,
        health_max,
        turns,
    };

    {
        let mut ds = DistributionStat::new("total score by demon");
        for d in demons.iter() {
            ds.add(d.total_points as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("need health by demon");
        for d in demons.iter() {
            ds.add(d.need_health as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("add health by demon");
        for d in demons.iter() {
            ds.add(d.add_health as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("time to recover by demon");
        for d in demons.iter() {
            ds.add(d.time_to_recover as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    let mut perm = vec![];

    let mut rnd = Random::new(89799);

    test.load_existing_result(|mut input: Input| {
        while input.has_more_elements() {
            perm.push(input.usize());
        }
        let mut used = vec![false; num_demons];
        for &p in perm.iter() {
            used[p] = true;
        }
        let mut not_used = vec![];
        for x in 0..num_demons {
            if !used[x] {
                not_used.push(x);
            }
        }
        perm.append(&mut not_used);
        assert_eq!(perm.len(), num_demons);
    });

    // let mut perm = gen_vec(num_demons, id);
    let mut prev_score = calc_score(&perm, &t);

    dbg!(prev_score);

    let mut sa = SimulatedAnnealing::new(600.0, SearchFor::MaximumScore, 10.0, 0.001);
    let mut last_saved = 0.0;
    while sa.should_continue() {
        let mut new_perm = perm.clone();

        let idx1 = rnd.gen_in_range(0..prev_score.demons_killed + 1);
        let idx2 = if rnd.gen_bool() {
            rnd.gen_in_range(0..perm.len())
        } else {
            min(num_demons - 1, idx1 + rnd.gen_in_range(0..10))
        };
        if rnd.gen_bool() {
            new_perm.swap(idx1, idx2);
        } else {
            let value = new_perm[idx2];
            new_perm.remove(idx2);
            new_perm.insert(idx1, value);
        }

        let new_score = calc_score(&new_perm, &t);
        if sa.should_go(prev_score.score, new_score.score) {
            perm = new_perm;
            prev_score = new_score;
        }

        if sa.elapsed_ms() > last_saved + 10_000.0 {
            last_saved = sa.elapsed_ms();
            test.save_result(&mut || {
                for &x in perm.iter() {
                    out_line!(x);
                }
            });
        }
    }

    dbg!(prev_score);

    test.save_result(&mut || {
        for &x in perm.iter() {
            out_line!(x);
        }
    });
}

pub(crate) fn run(mut _input: Input) -> bool {
    hashcode_solver(
        &"reply_2022",
        &"inputs",
        &"outputs",
        b'5'..=b'5',
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
