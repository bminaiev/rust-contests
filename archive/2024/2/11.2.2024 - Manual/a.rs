//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use std::cmp::{max, min};
use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

#[derive(Clone, Copy, Debug)]
struct Submission {
    time: i64,
    ok: bool,
}

#[derive(Clone, Debug)]
struct Team {
    by_task: HashMap<u8, Vec<Submission>>,
    name: String,
}

#[derive(Debug)]
struct TeamScore {
    cur: Score,
    best: Score,
    worst: Score,
}

impl std::ops::Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        Score {
            tasks_solved: self.tasks_solved + other.tasks_solved,
            rev_penalty: self.rev_penalty + other.rev_penalty,
        }
    }
}

impl Team {
    fn worst_single(&self) -> Score {
        let mut res = Score {
            tasks_solved: 1,
            rev_penalty: i64::MAX,
        };
        for (_, submissions) in self.by_task.iter() {
            let mut penalty = 20 * (submissions.len() - 1) as i64;
            penalty += submissions[submissions.len() - 1].time;
            let rev_penalty = -penalty;
            if rev_penalty < res.rev_penalty {
                res.rev_penalty = rev_penalty;
            }
        }
        if res.rev_penalty == i64::MAX {
            res.tasks_solved = 0;
        }
        res
    }

    fn calc_scores(&self) -> TeamScore {
        let mut cur = Score {
            tasks_solved: 0,
            rev_penalty: 0,
        };
        let mut best = Score {
            tasks_solved: 0,
            rev_penalty: 0,
        };
        let mut worst = Score {
            tasks_solved: 0,
            rev_penalty: 0,
        };
        for (_, submissions) in &self.by_task {
            let mut task_cur = Score {
                tasks_solved: 0,
                rev_penalty: 0,
            };
            let mut task_best = Score {
                tasks_solved: 0,
                rev_penalty: 0,
            };
            let mut task_worst = Score {
                tasks_solved: 0,
                rev_penalty: 0,
            };
            let mut cur_penalty = 0;
            let mut skipped_ok = false;
            for sub in submissions {
                if task_best.tasks_solved == 0 {
                    task_best.tasks_solved = 1;
                    task_best.rev_penalty = -(sub.time);
                }
                if sub.ok {
                    if task_cur.tasks_solved == 0 {
                        task_cur.tasks_solved = 1;
                        task_cur.rev_penalty = -(sub.time + cur_penalty);
                    }
                    if skipped_ok {
                        task_worst.tasks_solved = 1;
                        task_worst.rev_penalty = -(sub.time + cur_penalty);
                        break;
                    } else {
                        skipped_ok = true;
                        cur_penalty += 20;
                    }
                } else {
                    cur_penalty += 20;
                }
            }
            best = max(best + task_cur, cur + task_best);
            worst = min(worst + task_cur, cur + task_worst);
            cur = cur + task_cur;
        }
        TeamScore { cur, best, worst }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Debug)]
struct Score {
    tasks_solved: usize,
    rev_penalty: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct TeamAndScore {
    score: Score,
    team_id: usize,
}

fn get_gold_medels_cnt(teams: &[TeamAndScore]) -> usize {
    let mut cnt = teams.len();
    while cnt > 0 && teams[cnt - 1].score.tasks_solved == 0 {
        cnt -= 1;
    }
    if cnt == 0 {
        return 0;
    }
    let mut cnt = min(35, (cnt + 9) / 10);
    assert!(cnt > 0);
    let need_score = teams[cnt - 1].score;
    while cnt < teams.len() && teams[cnt].score == need_score {
        cnt += 1;
    }
    cnt
}

fn solve_case(teams: &[Team]) -> Vec<bool> {
    let mut scores = vec![];
    for t in teams.iter() {
        scores.push(t.calc_scores());
    }
    let mut teams_and_scores = vec![];
    for i in 0..scores.len() {
        teams_and_scores.push(TeamAndScore {
            score: scores[i].cur,
            team_id: i,
        });
    }
    teams_and_scores.sort();
    teams_and_scores.reverse();
    let mut can_gold = vec![false; teams.len()];
    let gold_limit = get_gold_medels_cnt(&teams_and_scores);
    for i in 0..gold_limit {
        can_gold[teams_and_scores[i].team_id] = true;
    }
    let mut cnt_non_zero = 0;
    for i in 0..teams.len() {
        if scores[i].cur.tasks_solved > 0 {
            cnt_non_zero += 1;
        }
    }
    for i in 0..teams.len() {
        let team_id = teams_and_scores[i].team_id;
        let best_score = scores[team_id].best;
        if best_score.tasks_solved == 0 {
            continue;
        }
        let mut now_non_zero = cnt_non_zero;
        if scores[team_id].cur.tasks_solved == 0 {
            now_non_zero += 1;
        }
        let cnt_better = binary_search_first_true(0..teams.len(), |pos| {
            teams_and_scores[pos].score <= best_score
        });
        let limit = min(35, (now_non_zero + 9) / 10);
        if cnt_better < limit {
            can_gold[team_id] = true;
        }
    }
    for i in 0..min(40, teams.len()) {
        let mut teams_and_scores = teams_and_scores.clone();
        teams_and_scores[i].score = scores[teams_and_scores[i].team_id].worst;
        teams_and_scores.sort();
        teams_and_scores.reverse();
        let gold_limit = get_gold_medels_cnt(&teams_and_scores);
        for j in 0..gold_limit {
            can_gold[teams_and_scores[j].team_id] = true;
        }
    }
    {
        let mut best_zero = vec![];
        for i in 0..teams.len() {
            if scores[i].cur.tasks_solved == 0 {
                let cur = teams[i].worst_single();
                if cur.tasks_solved == 1 {
                    best_zero.push((cur, i));
                }
            }
        }
        best_zero.sort();
        for best_zero in best_zero[..min(2, best_zero.len())].iter() {
            let mut teams_and_scores = teams_and_scores.clone();
            for pos in 0..teams_and_scores.len() {
                if teams_and_scores[pos].team_id == best_zero.1 {
                    teams_and_scores[pos].score = best_zero.0;
                    break;
                }
            }
            teams_and_scores.sort();
            teams_and_scores.reverse();
            let gold_limit = get_gold_medels_cnt(&teams_and_scores);
            for j in 0..gold_limit {
                can_gold[teams_and_scores[j].team_id] = true;
            }
        }
    }
    can_gold
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let m = input.usize();
        let mut teams = vec![];
        let mut by_name = HashMap::new();
        for _ in 0..m {
            let tname = input.string_as_string();
            if !by_name.contains_key(&tname) {
                let new_id = teams.len();
                teams.push(Team {
                    by_task: HashMap::new(),
                    name: tname.clone(),
                });
                by_name.insert(tname.clone(), new_id);
            }
            let t_id = by_name[&tname];
            let task = input.string()[0];
            let time = input.i64();
            let ok = input.string_as_string() == "accepted";
            teams[t_id]
                .by_task
                .entry(task)
                .or_insert_with(Vec::new)
                .push(Submission { time, ok });
        }
        let can_gold = solve_case(&teams);
        let mut cnt_can_gold = 0;
        let mut golds = vec![];
        for i in 0..teams.len() {
            if can_gold[i] {
                cnt_can_gold += 1;
                golds.push(teams[i].name.clone());
            }
        }
        out.println(cnt_can_gold);
        let all_golds = golds.join(" ");
        out.println(all_golds);
        // if 2 + 2 != 5 {
        //     break;
        // }
    }
}

fn solve_slow(teams: &[Team]) -> Vec<bool> {
    let mut res = vec![false; teams.len()];
    let mut update = |teams: &[Team]| {
        let mut teams_and_scores = vec![];
        for i in 0..teams.len() {
            let score = teams[i].calc_scores().cur;
            teams_and_scores.push(TeamAndScore { score, team_id: i });
        }
        teams_and_scores.sort();
        teams_and_scores.reverse();
        let gold_limit = get_gold_medels_cnt(&teams_and_scores);

        for j in 0..gold_limit {
            if !res[teams_and_scores[j].team_id] {
                // dbg!("Found", teams_and_scores[j].team_id);
                // dbg!(gold_limit);
                // for tt in teams_and_scores.iter() {
                //     dbg!(tt);
                // }
            }
            res[teams_and_scores[j].team_id] = true;
        }
    };
    update(teams);
    for t_id in 0..teams.len() {
        for task in teams[t_id].by_task.keys() {
            for sw_id in 0..teams[t_id].by_task[task].len() {
                let mut nteams = teams.to_vec();
                nteams[t_id]
                    .by_task
                    .get_mut(task)
                    .unwrap()
                    .get_mut(sw_id)
                    .unwrap()
                    .ok ^= true;
                update(&nteams);
            }
        }
    }
    res
}

fn stress() {
    for it in 146438.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const N: usize = 106;
        let teams_num = rnd.gen(1..N);
        let mut teams = vec![
            Team {
                by_task: HashMap::new(),
                name: "A".to_string(),
            };
            teams_num
        ];
        let num_submits = rnd.gen(1..2 * N);
        for _ in 0..num_submits {
            let team_id = rnd.gen(0..teams_num);
            let task = (b'A' + (rnd.gen(0..5)) as u8);
            let time = rnd.gen(1..100);
            let ok = rnd.gen_bool();
            teams[team_id]
                .by_task
                .entry(task)
                .or_insert_with(Vec::new)
                .push(Submission { time, ok });
        }
        for team in teams.iter_mut() {
            for (_, submissions) in team.by_task.iter_mut() {
                submissions.sort_by_key(|s| s.time);
            }
        }
        let teams: Vec<_> = teams.into_iter().filter(|t| t.by_task.len() > 0).collect();
        let ans = solve_case(&teams);
        let ans_slow = solve_slow(&teams);
        if ans != ans_slow {
            for (i, tt) in teams.iter().enumerate() {
                dbg!(i, tt);
            }
            dbg!(ans);
            dbg!(ans_slow);
            for i in 0..ans.len() {
                if ans[i] != ans_slow[i] {
                    dbg!(i, ans[i], ans_slow[i]);
                }
            }
            panic!();
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "5");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
