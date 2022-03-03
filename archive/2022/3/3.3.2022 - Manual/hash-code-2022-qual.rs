//{"name":"hash-code-test","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hash-code-test"}}}

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::RangeBounds;

use algo_lib::collections::id_map::IdMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use marathon_utils::distribution_stat::DistributionStat;
use marathon_utils::hashcode_solver::{hashcode_solver, OneTest};

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
    let mut projects = gen_vec(num_projects, |id| {
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

    let mut answers = vec![];

    test.load_existing_result(|mut input: Input| {
        let cnt = input.usize();
        answers = gen_vec(cnt, |_| {
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

    let mut expected_score = calc_score(&answers, &people, &projects, &proj_by_name);

    let mut sa = SimulatedAnnealing::new(5.0, SearchFor::MaximumScore, 10000.0, 0.1);
    while sa.should_continue() {
        // dbg!(it);
        let x = rnd.gen_in_range(0..answers.len());
        let y = rnd.gen_in_range(0..answers.len());
        if x == y {
            continue;
        }
        answers.swap(x, y);
        let new_score = calc_score(&answers, &people, &projects, &proj_by_name);
        if sa.should_go(expected_score, new_score) && new_score != 0 {
            expected_score = new_score;
        } else {
            answers.swap(x, y);
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
    test.report.add_value("Done projects", &answers.len());

    test.save_result(&mut || {
        out_line!(answers.len());
        for ans in answers.iter() {
            out_line!(vec2str(&ans.project_name));
            for &p_id in ans.people.iter() {
                out!(vec2str(&people[p_id].name), "");
            }
            out_line!();
        }
    });
}

fn calc_score(
    answers: &[Answer],
    people: &[Person],
    projects: &[Project],
    proj_by_name: &HashMap<Vec<u8>, usize>,
) -> i64 {
    let mut person_available_at = vec![0; people.len()];

    let mut current_skills: Vec<HashMap<usize, usize>> = vec![HashMap::new(); people.len()];
    for p in people.iter() {
        for skill in p.skills.iter() {
            current_skills[p.id].insert(skill.id, skill.lvl);
        }
    }

    let mut expected_score = 0;

    for a in answers.iter() {
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
            let cur_lvl = *current_skills[p_id].get(&req_skill.id).unwrap_or(&0);
            if cur_lvl < req_skill.lvl {
                if cur_lvl < req_skill.lvl - 1 {
                    return 0;
                }
                let mut found_menter = false;

                for &p2 in used_people_vec.iter() {
                    if *current_skills[p2].get(&req_skill.id).unwrap_or(&0) >= req_skill.lvl {
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
            let cur_lvl = *current_skills[p_id].get(&req_skill.id).unwrap_or(&0);

            if cur_lvl <= req_skill.lvl {
                *current_skills[p_id].entry(req_skill.id).or_default() += 1;
                cnt_improved += 1;
            }
        }

        expected_score += project.score - penalty as i64;
    }

    expected_score
}

struct Answer {
    project_name: Vec<u8>,
    people: Vec<usize>,
}

pub(crate) fn run(mut _input: Input) -> bool {
    hashcode_solver(
        &"hash-code-2022-qual",
        &"inputs",
        &"outputs",
        b'b'..=b'b',
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
