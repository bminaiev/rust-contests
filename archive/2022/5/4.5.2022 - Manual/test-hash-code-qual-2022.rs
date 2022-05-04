//{"name":"hash-code-test","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hash-code-test"}}}

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::RangeBounds;

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::id_map::IdMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use marathon_utils::{
    distribution_stat::DistributionStat,
    hashcode_solver::{hashcode_solver, OneTest},
};

struct Skill {
    id: usize,
    lvl: usize,
}

struct Person {
    id: usize,
    name: Vec<u8>,
    skills: Vec<Skill>,
}

struct Project {
    id: usize,
    name: Vec<u8>,
    spend_days: usize,
    score: i64,
    best_before: usize,
    required_roles: Vec<Skill>,
}

fn solve(input: &mut Input, test: &mut OneTest) {
    let num_people = input.usize();
    let num_projects = input.usize();

    let mut id_skills = IdMap::new();

    let people = gen_vec(num_people, |id| {
        let name = input.string();
        let num_skills = input.usize();
        let mut skills = vec![];
        for _ in 0..num_skills {
            let skill = input.string();
            let skill_id = id_skills.get_or_add(&skill);
            let lvl = input.usize();
            skills.push(Skill { id: skill_id, lvl });
        }
        Person { id, name, skills }
    });

    let mut peop_by_name = HashMap::new();
    for (id, p) in people.iter().enumerate() {
        peop_by_name.insert(p.name.clone(), id);
    }

    dbg!(num_projects);
    let projects = gen_vec(num_projects, |id| {
        let name = input.string();
        let spend_days = input.usize();
        let score = input.i64();
        let best_before = input.usize();
        let num_roles = input.usize();
        let required_roles = gen_vec(num_roles, |_| {
            let id = id_skills.get_exn(&input.string());
            let lvl = input.usize();
            Skill { id, lvl }
        });
        Project {
            id,
            name,
            spend_days,
            score,
            best_before,
            required_roles,
        }
    });

    let mut proj_by_name = HashMap::new();
    for (id, p) in projects.iter().enumerate() {
        proj_by_name.insert(p.name.clone(), id);
    }

    test.report.add_value("number of people", &people.len());
    test.report
        .add_value("number of different existing skills", &id_skills.len());

    test.report.add_value("number of projects", &projects.len());

    {
        let mut stat_lvl_required = DistributionStat::new("Level required for project");
        for p in projects.iter() {
            for s in p.required_roles.iter() {
                stat_lvl_required.add(s.lvl as i32);
            }
        }
        test.report.add_distribution_stat(&stat_lvl_required);
    }

    {
        let mut stat_project_score = DistributionStat::new("Score by project");
        for p in projects.iter() {
            stat_project_score.add(p.score as i32);
        }
        test.report.add_distribution_stat(&stat_project_score);
    }

    {
        let mut stat_project_required_rolls = DistributionStat::new("Project required rolls");
        for p in projects.iter() {
            stat_project_required_rolls.add(p.required_roles.len() as i32);
        }
        test.report
            .add_distribution_stat(&stat_project_required_rolls);
    }

    {
        let mut stat_project_score_by_hour =
            DistributionStat::new("Score by project / number of people / number of hours");
        for p in projects.iter() {
            let value = (p.score as i32) / (p.required_roles.len() as i32) / (p.spend_days as i32);
            stat_project_score_by_hour.add(value as i32);
        }
        test.report
            .add_distribution_stat(&stat_project_score_by_hour);
    }

    {
        let mut stat_best_before = DistributionStat::new("Project's best before");
        for p in projects.iter() {
            stat_best_before.add(p.best_before as i32);
        }
        test.report.add_distribution_stat(&stat_best_before);
    }

    {
        let mut potential_max_score = 0;
        for p in projects.iter() {
            potential_max_score += p.score;
        }
        test.report
            .add_value("potential max score", &potential_max_score);
    }

    let mut rnd = Random::new(4472);

    let mut answers_stable = vec![];

    test.load_existing_result(|mut input: Input| {
        let cnt = input.usize();
        answers_stable = gen_vec(cnt, |_| {
            let proj_name = input.string();
            let proj_id = *proj_by_name.get(&proj_name).unwrap();
            let len = projects[proj_id].required_roles.len();
            let peo = gen_vec(len, |_| *peop_by_name.get(&input.string()).unwrap());
            Answer {
                project_name: proj_name,
                people: peo,
            }
        });
    });

    let mut init_skills: Vec<FxHashMap<usize, usize>> = vec![FxHashMap::default(); people.len()];
    for p in people.iter() {
        for skill in p.skills.iter() {
            init_skills[p.id].insert(skill.id, skill.lvl);
        }
    }

    let mut answers_perm = gen_vec(1, |_| gen_vec(answers_stable.len(), id));
    let mut expected_score = calc_score(
        &answers_stable,
        &answers_perm[0],
        &people,
        &projects,
        &proj_by_name,
        &init_skills,
    );

    let save = |test: &mut OneTest, answers_perm: &[usize]| {
        test.save_result(&mut || {
            out_line!(answers_perm.len());
            for &ans_id in answers_perm.iter() {
                let ans = &answers_stable[ans_id];
                out_line!(vec2str(&ans.project_name));
                for &p_id in ans.people.iter() {
                    out!(vec2str(&people[p_id].name), "");
                }
                out_line!();
            }
        });
    };

    dbg!(expected_score);
    let mut sa =
        SimulatedAnnealing::new(60.0, SearchFor::MaximumScore, 1000.0, 0.1, expected_score);
    let mut sa_report = test.report.add_sa();
    while sa.should_continue() {
        sa_report.update(test, &sa);

        use rayon::prelude::*;

        let start_seed = rnd.gen_u64();
        let options: Vec<_> = answers_perm
            .par_iter_mut()
            .enumerate()
            .map(|(idx, answers_perm)| {
                let mut rnd = Random::new(start_seed + idx as u64);

                let x = rnd.gen_in_range(0..answers_perm.len());
                let y = rnd.gen_in_range(0..answers_perm.len());

                let mut new_answers_perm = answers_perm.clone();
                new_answers_perm.swap(x, y);
                let score = calc_score(
                    &answers_stable,
                    &new_answers_perm,
                    &people,
                    &projects,
                    &proj_by_name,
                    &init_skills,
                );
                (x, y, score)
            })
            .collect();

        for (x, y, score) in options.into_iter() {
            if sa.should_go(score) {
                for perm in answers_perm.iter_mut() {
                    perm.swap(x, y);
                }

                expected_score = score;
                break;
            }
        }

        if sa.should_save(false) {
            save(test, &answers_perm[0]);
        }
    }

    // {
    //     let mut stat_used_time = DistributionStat::new(&"Time worked by person");
    //     for &time in used_time_by_person.iter() {
    //         stat_used_time.add(time as i32);
    //     }
    //     test.report.add_distribution_stat(&stat_used_time);
    // }

    dbg!(expected_score);

    test.report.add_value("My score", &expected_score);
    test.report
        .add_value("Done projects", &answers_stable.len());

    if sa.should_save(true) {
        save(test, &answers_perm[0]);
    }
}

fn calc_score(
    answers_stable: &[Answer],
    answers_perm: &[usize],
    people: &[Person],
    projects: &[Project],
    proj_by_name: &HashMap<Vec<u8>, usize>,
    init_skills: &Vec<FxHashMap<usize, usize>>,
) -> i64 {
    let mut person_available_at = vec![0; people.len()];

    let mut skill_overrides = FxHashMap::<(i32, i32), i32>::default();

    let get_skill =
        |p: usize, skill_id: usize, skill_overrides: &FxHashMap<(i32, i32), i32>| -> usize {
            let key = (p as i32, skill_id as i32);
            if let Some(r) = skill_overrides.get(&key) {
                return *r as usize;
            } else {
                *init_skills[p].get(&skill_id).unwrap_or(&0)
            }
        };

    let mut expected_score = 0;

    for &a_id in answers_perm.iter() {
        let a = &answers_stable[a_id];
        let project = &projects[*proj_by_name.get(&a.project_name).unwrap()];
        let used_people_vec = &a.people;

        let mut real_start_time = 0;
        for &p_id in used_people_vec.iter() {
            real_start_time.update_max(person_available_at[p_id]);
        }
        let available_at = real_start_time + project.spend_days;

        let penalty = if available_at <= project.best_before {
            0
        } else {
            available_at - project.best_before
        };

        let mut cnt_improved = 0;

        for (&p_id, req_skill) in used_people_vec.iter().zip(project.required_roles.iter()) {
            person_available_at[p_id] = available_at;
            let cur_lvl = get_skill(p_id, req_skill.id, &skill_overrides);
            if cur_lvl < req_skill.lvl {
                if cur_lvl < req_skill.lvl - 1 {
                    return 0;
                }
                let mut found_menter = false;

                for &p2 in used_people_vec.iter() {
                    if get_skill(p2, req_skill.id, &skill_overrides) >= req_skill.lvl {
                        found_menter = true;
                        break;
                    }
                }

                if !found_menter {
                    return 0;
                }
            }
        }

        for (&p_id, req_skill) in used_people_vec.iter().zip(project.required_roles.iter()) {
            person_available_at[p_id] = available_at;
            let cur_lvl = get_skill(p_id, req_skill.id, &skill_overrides);

            if cur_lvl <= req_skill.lvl {
                skill_overrides.insert((p_id as i32, req_skill.id as i32), (cur_lvl + 1) as i32);
                cnt_improved += 1;
            }
        }

        expected_score += project.score - penalty as i64;
    }

    expected_score
}

#[derive(Clone)]
struct Answer {
    project_name: Vec<u8>,
    people: Vec<usize>,
}

pub(crate) fn run(mut _input: Input) -> bool {
    hashcode_solver(
        &"test-hash-code-qual-2022",
        &"inputs",
        &"outputs",
        b'f'..=b'f',
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
