//{"name":"hash-code-test","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hash-code-test"}}}

use algo_lib::collections::id_map::IdMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use marathon_utils::dynamic_plot::DynamicPlot;
use marathon_utils::hashcode_solver::{hashcode_solver, OneTest};

use crate::client::Client;
use crate::scorer::Scorer;

mod client;
mod scorer;

fn solve(input: &mut Input, test: &mut OneTest) {
    let num_clients = input.usize();
    test.report.add_value(&"Number of clients", &num_clients);
    dbg!(num_clients);
    let mut ingredients = IdMap::new();
    let clients = gen_vec(num_clients, |_| {
        let cnt_likes = input.usize();
        let likes = input
            .string_vec(cnt_likes)
            .iter()
            .map(|s| ingredients.get_or_add(s))
            .collect();
        let cnt_dislikes = input.usize();
        let dislikes = input
            .string_vec(cnt_dislikes)
            .iter()
            .map(|s| ingredients.get_or_add(s))
            .collect();
        Client { likes, dislikes }
    });
    dbg!(ingredients.len());

    let mut rnd = Random::new(787788);
    let mut best_scorer = Scorer::new(clients, ingredients.len());

    test.load_existing_result(|mut input: Input| {
        let n = input.usize();
        let used: Vec<_> = input
            .string_vec(n)
            .into_iter()
            .map(|str| ingredients.get_exn(&str))
            .collect();
        for &i_id in used.iter() {
            best_scorer.switch_ingredient(i_id);
        }
        dbg!("loaded solution", best_scorer.num_ok_clients());
    });

    let mut scorer = best_scorer.clone();

    let save_result = |test: &mut OneTest, best_scorer: &Scorer| {
        test.save_result(&mut || {
            let mut res = vec![];
            for i in 0..ingredients.len() {
                if best_scorer.use_ingredients(i) {
                    res.push(vec2str(&ingredients[i]));
                }
            }
            out!(res.len(), "");
            out_line!(res);
        });
    };

    let score_plot = test.report.add_dynamic_plot(DynamicPlot::new(
        &"Score on each iteration of SA:",
        &"time (ms)",
        &"score",
    ));

    let delta_plot = test.report.add_dynamic_plot(DynamicPlot::new(
        &"Delta of scores checked by SA:",
        &"time (ms)",
        &"delta",
    ));

    let temp_plot = test.report.add_dynamic_plot(DynamicPlot::new(
        &"Temperature of SA:",
        &"time (ms)",
        &"temperature",
    ));

    let mut sa = SimulatedAnnealing::new(3.0, SearchFor::MaximumScore, 20.0, 0.02);
    let mut iter = 0;
    while sa.should_continue() {
        score_plot.add_point(test, sa.elapsed_ms(), scorer.num_ok_clients());
        temp_plot.add_point(test, sa.elapsed_ms(), sa.current_temperature());
        delta_plot.add_point(test, sa.elapsed_ms(), sa.last_delta());
        iter += 1;
        if iter % 10000 == 0 {
            // dbg!(
            //     test.name,
            //     iter,
            //     scorer.num_ok_clients,
            //     best_scorer.num_ok_clients
            // );
            // test.report.save();
            // for _ in 0..100 {
            // scorer.switch_ingredient(rnd.gen_in_range(0..ingredients.len()));
            // }
        }
        if scorer.num_ok_clients() > best_scorer.num_ok_clients() {
            best_scorer = scorer.clone();

            save_result(test, &best_scorer);
        }
        let cur_score = scorer.num_ok_clients();

        let client = scorer.peek_random_client_exn(&mut rnd);
        let mut to_switch = vec![];
        for &x in client.likes.iter() {
            if !scorer.use_ingredients(x) {
                to_switch.push(x);
            }
        }
        for &x in client.dislikes.iter() {
            if scorer.use_ingredients(x) {
                to_switch.push(x);
            }
        }

        for x in to_switch.iter() {
            scorer.switch_ingredient(*x);
        }
        if !sa.should_go(cur_score, scorer.num_ok_clients()) {
            for x in to_switch.iter() {
                scorer.switch_ingredient(*x);
            }
            assert_eq!(scorer.num_ok_clients(), cur_score);
        }
    }

    test.report
        .add_value("score", &best_scorer.num_ok_clients());

    save_result(test, &best_scorer);
}

pub(crate) fn run(mut _input: Input) -> bool {
    hashcode_solver(
        &"hash-code-test",
        &"inputs",
        &"outputs",
        b'd'..=b'd',
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
