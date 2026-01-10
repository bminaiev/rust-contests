//{"name":"bulls","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use itertools::Itertools;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

const URL: &str = "https://2026.andgein.ru/api/tasks/bulls-and-cows";
const KEY: &str = "30511344aafe488de6dfe4f742cd63b63212be2f516c0de162acfcb2cb6f2b93";

fn solve(input: &mut Input, out: &mut Output) {}

#[derive(Debug, Deserialize, Clone)]
struct GuessInfo {
    numbers: Vec<usize>,
    bulls: usize,
    cows: usize,
}

#[derive(Debug, Deserialize)]
struct Parameters {
    guesses: Vec<GuessInfo>,
}

#[derive(Debug, Deserialize)]
struct Task {
    parameters: Parameters,
    current_level: usize,
}

#[derive(Debug, Deserialize)]
struct SubmitResponse {
    is_correct: bool,
    checker_output: serde_json::Value,
}

fn potentially_valid(
    guesses: &[GuessInfo],
    used: &HashSet<usize>,
    not_used: &HashSet<usize>,
    guess_idx: usize,
    all_used: &mut HashSet<Vec<usize>>,
) {
    let n = guesses[0].numbers.len();
    if used.len() > n {
        return;
    }
    if guess_idx >= guesses.len() {
        dbg!("WOW", &used);
        let mut used_vec: Vec<usize> = used.iter().copied().collect();
        used_vec.sort_unstable();
        all_used.insert(used_vec);
        return;
    }
    for guess_idx2 in guess_idx..guesses.len() {
        let mut cnt_seen = 0;
        for x in &guesses[guess_idx2].numbers {
            if used.contains(x) {
                cnt_seen += 1;
            }
        }
        let need_seen = guesses[guess_idx2].bulls + guesses[guess_idx2].cows;
        if cnt_seen > need_seen {
            return;
        }
        let more = need_seen - cnt_seen;
        if used.len() + more > n {
            return;
        }
    }
    let guess = &guesses[guess_idx];
    let total = guess.bulls + guess.cows;
    for mask in (0..n).combinations(total) {
        // if guess_idx == 0 {
        //     dbg!(&mask);
        // }
        let mut present = vec![false; n];
        for idx in mask {
            present[idx] = true;
        }
        let mut new_used = used.clone();
        let mut new_not_used = not_used.clone();
        let mut ok = true;
        for i in 0..n {
            let expected = guess.numbers[i];
            if present[i] {
                if new_not_used.contains(&expected) {
                    ok = false;
                    break;
                }
                new_used.insert(expected);
            } else {
                if new_used.contains(&expected) {
                    ok = false;
                    break;
                }
                new_not_used.insert(expected);
            }
        }
        if ok {
            potentially_valid(guesses, &new_used, &new_not_used, guess_idx + 1, all_used);
        }
    }
}

fn is_valid_answer(guesses: &[GuessInfo], candidate: &[usize]) -> bool {
    for guess_info in guesses {
        let bulls = candidate
            .iter()
            .zip(guess_info.numbers.iter())
            .filter(|(c, g)| c == g)
            .count();
        if bulls != guess_info.bulls {
            return false;
        }
        let mut cand_count: HashMap<usize, usize> = HashMap::new();
        let mut guess_count: HashMap<usize, usize> = HashMap::new();
        for &d in candidate {
            *cand_count.entry(d).or_insert(0) += 1;
        }
        for &d in &guess_info.numbers {
            *guess_count.entry(d).or_insert(0) += 1;
        }
        let mut overlap = 0usize;
        for (d, &gc) in &guess_count {
            let cc = cand_count.get(d).copied().unwrap_or(0);
            overlap += gc.min(cc);
        }
        let cows = overlap - bulls;
        if bulls != guess_info.bulls || cows != guess_info.cows {
            return false;
        }
    }
    true
}

fn find_smallest_permutation(arr: &[usize], guesses: &[GuessInfo]) -> Option<Vec<usize>> {
    let n = arr.len();
    let arr_set: HashSet<usize> = arr.iter().copied().collect();
    let arr_sorted: Vec<usize> = {
        let mut tmp = arr.to_vec();
        tmp.sort_unstable();
        tmp
    };

    fn complete_smallest(
        guesses: &[GuessInfo],
        arr_sorted: &[usize],
        candidate: &Vec<Option<usize>>,
        used_vals: &HashSet<usize>,
    ) -> Option<Vec<usize>> {
        let n = arr_sorted.len();
        let remaining: Vec<usize> = arr_sorted
            .iter()
            .copied()
            .filter(|v| !used_vals.contains(v))
            .collect();
        let mut used = vec![false; remaining.len()];
        let mut cur = candidate.clone();

        fn dfs(
            pos: usize,
            remaining: &[usize],
            used: &mut [bool],
            cur: &mut Vec<Option<usize>>,
            guesses: &[GuessInfo],
        ) -> Option<Vec<usize>> {
            if pos == cur.len() {
                let candidate_vec: Vec<usize> = cur.iter().map(|v| v.unwrap()).collect();
                if is_valid_answer(guesses, &candidate_vec) {
                    return Some(candidate_vec);
                }
                return None;
            }
            if cur[pos].is_some() {
                return dfs(pos + 1, remaining, used, cur, guesses);
            }
            for i in 0..remaining.len() {
                if used[i] {
                    continue;
                }
                cur[pos] = Some(remaining[i]);
                used[i] = true;
                if let Some(ans) = dfs(pos + 1, remaining, used, cur, guesses) {
                    return Some(ans);
                }
                used[i] = false;
                cur[pos] = None;
            }
            None
        }

        dfs(0, &remaining, &mut used, &mut cur, guesses)
    }

    fn dfs_guesses(
        guess_idx: usize,
        guesses: &[GuessInfo],
        arr_set: &HashSet<usize>,
        arr_sorted: &[usize],
        candidate: &Vec<Option<usize>>,
        used_vals: &HashSet<usize>,
    ) -> Option<Vec<usize>> {
        if guess_idx == guesses.len() {
            return complete_smallest(guesses, arr_sorted, candidate, used_vals);
        }
        let guess = &guesses[guess_idx];
        let total = guess.bulls + guess.cows;
        let mut present_positions = Vec::new();
        for (i, &v) in guess.numbers.iter().enumerate() {
            if arr_set.contains(&v) {
                present_positions.push(i);
            }
        }
        if present_positions.len() != total {
            return None;
        }

        let mut must_bull = Vec::new();
        let mut available = Vec::new();
        for &pos in &present_positions {
            match candidate[pos] {
                Some(val) => {
                    if val == guess.numbers[pos] {
                        must_bull.push(pos);
                    }
                }
                None => {
                    available.push(pos);
                }
            }
        }
        if must_bull.len() > guess.bulls {
            return None;
        }
        let need = guess.bulls - must_bull.len();
        if available.len() < need {
            return None;
        }

        let mut best: Option<Vec<usize>> = None;
        for extra in available.iter().copied().combinations(need) {
            let mut new_candidate = candidate.clone();
            let mut new_used = used_vals.clone();
            let mut ok = true;
            for &pos in must_bull.iter().chain(extra.iter()) {
                let val = guess.numbers[pos];
                if let Some(cur) = new_candidate[pos] {
                    if cur != val {
                        ok = false;
                        break;
                    }
                } else {
                    if new_used.contains(&val) {
                        ok = false;
                        break;
                    }
                    new_candidate[pos] = Some(val);
                    new_used.insert(val);
                }
            }
            if ok {
                if let Some(ans) = dfs_guesses(
                    guess_idx + 1,
                    guesses,
                    arr_set,
                    arr_sorted,
                    &new_candidate,
                    &new_used,
                ) {
                    if best.as_ref().map_or(true, |b| ans < *b) {
                        best = Some(ans);
                    }
                }
            }
        }
        best
    }

    let candidate = vec![None; n];
    let used_vals: HashSet<usize> = HashSet::new();
    dfs_guesses(0, guesses, &arr_set, &arr_sorted, &candidate, &used_vals)
}

fn solve_task(guesses: &mut Vec<GuessInfo>) -> Vec<usize> {
    guesses.sort_by_key(|g| Reverse(g.bulls + g.cows));
    let mut all_multisets: HashSet<Vec<usize>> = HashSet::new();
    potentially_valid(
        guesses,
        &HashSet::new(),
        &HashSet::new(),
        0,
        &mut all_multisets,
    );
    dbg!(all_multisets.len());
    let n = guesses[0].numbers.len();
    let mut seen_numbers: HashSet<usize> = HashSet::new();
    for guess in guesses.iter() {
        for d in &guess.numbers {
            seen_numbers.insert(*d);
        }
    }
    let mut all_perms: Vec<Vec<usize>> = Vec::new();
    for used in all_multisets.iter() {
        let mut multiset: HashSet<usize> = used.iter().copied().collect();
        let mut it: usize = 1;
        while multiset.len() < n {
            let candidate = it;
            if !seen_numbers.contains(&candidate) {
                multiset.insert(candidate);
            }
            it += 1;
        }
        let mut arr: Vec<usize> = multiset.iter().copied().collect();
        arr.sort_unstable();
        dbg!(arr);
        guesses.sort_by_key(|g| Reverse(g.bulls));
        if let Some(candidate) = find_smallest_permutation(&arr, guesses) {
            all_perms.push(candidate);
        }
    }
    all_perms.sort();
    for candidate in all_perms {
        if is_valid_answer(guesses, &candidate) {
            return candidate;
        }
    }
    panic!("no valid answer found");
}

fn format_answer(answer: &[usize]) -> String {
    answer
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn stress() {
    let mut headers = HeaderMap::new();
    headers.insert("Key", HeaderValue::from_static(KEY));
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("client build");
    loop {
        // let zz = client.get(URL).send().expect("task request");
        // dbg!(&zz.json::<serde_json::Value>().expect("json"));
        let task: Task = client
            .get(URL)
            .send()
            .expect("task request")
            .json()
            .expect("task json");
        dbg!(task.current_level);
        let mut guesses = task.parameters.guesses;
        dbg!(guesses);
        let answer = format_answer(&solve_task(&mut guesses));
        dbg!(&answer);
        let response: SubmitResponse = client
            .post(URL)
            .json(&serde_json::json!({
                "level": task.current_level,
                "answer": answer
            }))
            .send()
            .expect("submit request")
            .json()
            .expect("submit json");
        dbg!(&response);
        if !response.is_correct {
            unreachable!();
            break;
        }
        // break;
        sleep(Duration::from_millis(100));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "bulls";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
