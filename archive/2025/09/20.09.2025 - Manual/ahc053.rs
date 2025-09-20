//{"name":"ahc053","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BinaryHeap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

#[cfg(feature = "local")]
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const N: usize = 500;
const M: usize = 50;
const L: i64 = 998_000_000_000_000;
const R: i64 = 1_002_000_000_000_000;
const DELTA: i64 = 4000000000000;

fn c(x: i64) -> String {
    format!("{:.2}G", (x as f64) / 1e9)
}

const STRATEGY: [i64; 500] = [
    1001271572601299,
    1001104860649318,
    1000954846126376,
    1000783337189101,
    1000661424349608,
    1000613330541719,
    1000563193670503,
    1000484224759189,
    1000394627776706,
    1000319173813602,
    1000200089369004,
    1000039438425369,
    999982472480552,
    999949893943479,
    999833649399042,
    999663957763144,
    999640628174681,
    999572640637004,
    999458735773900,
    999451662096630,
    999336026694791,
    999263920165924,
    999231265508145,
    999103394830797,
    998985231175808,
    998860988901843,
    998821099890309,
    998791551751352,
    998743331732773,
    998693412231796,
    998641697069151,
    998596715042995,
    998489582906502,
    998452197176648,
    998410105305141,
    998388752448905,
    998339862904763,
    998274813006897,
    998239413442882,
    998225149780279,
    998178634365724,
    998121895703829,
    998088301728094,
    998040778784887,
    998024154273358,
    998020040417959,
    998006684017166,
    998001122354317,
    998000006148543,
    960878629726185,
    3105186998225,
    3082928163491,
    2006222266803,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    2000000000000,
    1999077515684,
    1546265191070,
    1526530842581,
    1298213179214,
    1263242295444,
    1263242295444,
    1022933020939,
    1019083620092,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    1000000000000,
    913756125565,
    830764414183,
    796820213363,
    769266037295,
    703730990384,
    702180846931,
    631390281459,
    630181669246,
    554197506508,
    503497113189,
    501178969692,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    500000000000,
    494863024812,
    485937056563,
    471487297499,
    464312796986,
    461833948385,
    426120382236,
    408436262276,
    308226612971,
    276674788672,
    271933092838,
    263572805690,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    250000000000,
    249984321243,
    246886927138,
    239804066675,
    233111278497,
    208831992377,
    208794050283,
    204978601364,
    202927407062,
    185275483508,
    176762083202,
    165259465354,
    158967441353,
    127337175624,
    126913902916,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    125000000000,
    124965461862,
    124965461862,
    120465540073,
    118542646490,
    118492248497,
    117772720276,
    116573722051,
    116136963080,
    113679004419,
    111829601560,
    107858359881,
    103331491257,
    99696524413,
    98378537776,
    95226127332,
    92060038674,
    87700519450,
    79358694203,
    62500000000,
    62500000000,
    62500000000,
    62500000000,
    62500000000,
    62500000000,
    62500000000,
    62500000000,
    61576698477,
    61450306802,
    60948656869,
    60422401350,
    60184072603,
    58137401440,
    58137401440,
    57556697579,
    57298256156,
    50889530403,
    49641643590,
    40952318406,
    36118452088,
    34005169720,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31250000000,
    31054647719,
    30329437759,
    30132286216,
    29841784869,
    29480182209,
    26685235981,
    25801496574,
    24177659603,
    23597993770,
    17351231442,
    17248623673,
    16761278463,
    16711015512,
    16650183121,
    15625000000,
    15625000000,
    15625000000,
    15625000000,
    15625000000,
    15625000000,
    15555106484,
    15382799144,
    15365350071,
    15015565750,
    14871400005,
    14862610771,
    14456356356,
    14304844520,
    11712956293,
    11168127800,
    10837092164,
    10785145349,
    10349680054,
    10252726696,
    9802373806,
    9216853195,
    8945576528,
    8803904562,
    8803904562,
    8427174992,
    7877392101,
    7813538967,
    7812112659,
    7808269524,
    7623668264,
    7418141691,
    7213517849,
    7135598983,
    6893606240,
    6235496961,
    6083809315,
    5922550401,
    5599219294,
    5237641301,
    4772475315,
    3960552656,
    3919294292,
    3906250000,
    3906250000,
    3906250000,
    3906250000,
    3884983161,
    3829031723,
    3793143792,
    3775172494,
    3458623558,
    3453005867,
    3374904144,
    3330681876,
    3106202544,
    2886716017,
    2868981401,
    2828013661,
    2786545492,
    2713035990,
    2635906711,
    2558225736,
    2466721322,
    2436867159,
    2413702514,
    2379020571,
    2149250118,
    1953125000,
    1953125000,
    1953125000,
    1951461188,
    1932308202,
    1922475071,
    1891489722,
    1835656821,
    1738054754,
    1620865699,
    1612634522,
    1567422100,
    1542098967,
    1456391086,
    1405868368,
    1380871305,
    1290805092,
    1231036635,
    1171258149,
    1038396340,
    985795380,
    976562500,
    976562500,
    976562500,
    957685028,
    923743744,
    895623188,
    833954289,
    799349917,
    737032036,
    721181637,
    701973257,
    683720504,
    651403541,
    565007886,
    563660410,
    466628005,
    450333109,
    433532766,
    427463376,
    411821216,
    396121119,
    377395933,
    361749699,
    359571392,
    354123681,
    339612521,
    328343219,
    315033938,
    305312794,
    296313695,
    284009331,
    267253065,
    267065088,
    257287689,
    243992828,
    238054083,
    231615839,
    222930426,
    215817335,
    206623402,
    202542139,
    196380110,
    187470624,
    177018185,
    170152744,
    160546553,
    159959334,
    153292353,
    140984850,
    137930436,
    131529343,
    130980628,
    123956084,
    123037812,
    117963959,
    113317380,
    107858134,
    104259740,
    99858437,
    95758506,
    92892946,
    88138582,
    83248720,
    82586784,
    80203965,
    78231085,
    75202932,
    69739267,
    69533157,
    64612925,
    61049327,
    57454287,
    56044963,
    55014577,
    52403706,
    49069882,
    46247147,
    44156506,
    38041442,
    33880443,
    29433463,
    29023981,
    28856595,
    28559633,
    27957832,
    27288556,
    25533956,
    24174780,
    23310373,
    21664353,
    19965948,
    19764853,
    18790339,
    17830732,
    17158082,
    16138479,
    15658154,
    15241212,
    14714638,
    14294806,
    13916058,
    13216416,
    12677203,
    12262880,
    11764701,
    11442592,
    10988117,
    10712743,
    10320704,
    9940851,
    9580815,
    9355027,
    9046694,
    8772466,
    8513996,
    8261408,
    7962996,
    7675169,
    7262391,
    7085538,
    6876907,
    6652801,
    6389625,
    6220924,
    6030229,
    5843314,
    5604940,
    5337779,
    5165931,
    4957841,
    4730690,
    4590540,
    4430449,
    4245483,
    4121614,
    3900227,
    3726235,
    3591622,
    3368287,
    3209629,
    3030938,
    2896584,
    2707570,
    2569818,
    2448155,
    2307132,
    2135147,
    2054788,
    1895119,
    1783385,
    1634793,
    1556699,
    1469945,
    1340495,
    1229842,
    1121414,
    1031165,
    948445,
    865940,
    774253,
    686447,
];

fn gen_rng_array(rng: &mut Random, len: usize) -> Vec<i64> {
    let mut a = vec![0; len];
    for i in 0..len {
        a[i] = rng.next_in_range(L as usize, R as usize) as i64;
    }
    a
}

fn generate_simple_strategy() -> Vec<i64> {
    return STRATEGY.to_vec();
    let mut input = Input::new_file("strategy.txt");
    let mut full_line = String::new();
    loop {
        let mut line = input.read_line();
        if line.is_empty() {
            break;
        }
        full_line += &line;
    }
    // remove [ and ]
    let line = full_line[1..full_line.len() - 1].to_string();
    let s = line
        .split(", ")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    return s;

    let mut strategy = vec![];
    let mut rng = Random::new(123123234);
    let mut smallest = vec![i64::MAX; M];
    for _ in 0..10000 {
        let mut a = gen_rng_array(&mut rng, M);
        a.sort();
        for i in 0..M {
            smallest[i] = smallest[i].min(a[i]);
        }
    }
    dbg!(smallest);
    for i in 0..50 {
        strategy.push(smallest[i]);
    }

    let mut base = DELTA / 2;
    while strategy.len() < N {
        for _ in 0..35 {
            strategy.push(base);
        }
        base /= 2;
    }
    strategy.truncate(N);

    strategy.sort();
    strategy.reverse();
    strategy
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    more: i64,
    pos: usize,
}

fn split_into_groups(strategy: &[i64], need: &[i64]) -> Vec<usize> {
    let mut res = vec![0; strategy.len()];
    let mut items = BinaryHeap::new();
    for i in 0..need.len() {
        items.push(Item {
            more: need[i],
            pos: i,
        });
    }
    let mut strategy_items = strategy
        .iter()
        .enumerate()
        .map(|(i, &v)| Item { more: v, pos: i })
        .collect::<Vec<_>>();
    strategy_items.sort_by_key(|item| -item.more);
    for strategy_item in strategy_items.iter() {
        let i = strategy_item.pos;
        if let Some(item) = items.pop() {
            if item.more >= strategy[i] {
                res[i] = item.pos + 1;
                items.push(Item {
                    more: item.more - strategy[i],
                    pos: item.pos,
                });
            } else {
                items.push(item);
            }
        }
    }
    let mut not_used_items = vec![];
    for i in 0..strategy_items.len() {
        if res[strategy_items[i].pos] == 0 {
            not_used_items.push(strategy_items[i]);
        }
    }
    let mut iter = 0;
    while let Some(item) = items.pop() {
        while iter < not_used_items.len() {
            let cur_diff = item.more;
            let applied_diff = (item.more - strategy[item.pos]).abs();
            if applied_diff > cur_diff {
                iter += 1;
                continue;
            }
            res[not_used_items[iter].pos] = item.pos + 1;
            iter += 1;
            break;
        }
    }

    res
}

struct Scorer {
    need: Vec<i64>,
    sums: Vec<i64>,
    score: i64,
}

impl Scorer {
    fn new(need: Vec<i64>) -> Self {
        Self {
            sums: vec![0; need.len() + 1],
            score: need.iter().sum::<i64>(),
            need,
        }
    }

    fn change(&mut self, idx: usize, delta: i64) {
        if idx == 0 {
            return;
        }
        let idx = idx - 1;
        self.score -= (self.sums[idx] - self.need[idx]).abs();
        self.sums[idx] += delta;
        self.score += (self.sums[idx] - self.need[idx]).abs();
    }
}

fn split_into_groups_hill_climbing(strategy: &[i64], need: &[i64], iters: usize) -> Vec<usize> {
    let mut res = split_into_groups(strategy, need);
    let mut rng = Random::new(123123);
    let mut scorer = Scorer::new(need.to_vec());
    for i in 0..strategy.len() {
        scorer.change(res[i], strategy[i]);
    }
    let mut cur_score = scorer.score;
    // dbg!(cur_score);
    for _ in 0..iters {
        let idx = rng.gen_range(0..strategy.len());
        if rng.gen_bool() {
            // pick one, and move to another group
            let cur_group = res[idx];
            let new_group = rng.gen_range(0..need.len() + 1);
            scorer.change(cur_group, -strategy[idx]);
            scorer.change(new_group, strategy[idx]);
            let new_score = scorer.score;
            if new_score < cur_score {
                cur_score = new_score;
                res[idx] = new_group;
            } else {
                scorer.change(cur_group, strategy[idx]);
                scorer.change(new_group, -strategy[idx]);
            }
        } else {
            // swap two
            let idx2 = rng.gen_range(0..strategy.len());
            if idx == idx2 {
                continue;
            }
            let cur_group = res[idx];
            let cur_group2 = res[idx2];
            scorer.change(cur_group, -strategy[idx]);
            scorer.change(cur_group2, -strategy[idx2]);
            scorer.change(cur_group2, strategy[idx]);
            scorer.change(cur_group, strategy[idx2]);
            let new_score = scorer.score;
            if new_score < cur_score {
                cur_score = new_score;
                res[idx] = cur_group2;
                res[idx2] = cur_group;
            } else {
                scorer.change(cur_group, strategy[idx]);
                scorer.change(cur_group2, strategy[idx2]);
                scorer.change(cur_group2, -strategy[idx]);
                scorer.change(cur_group, -strategy[idx2]);
            }
        }
    }
    // dbg!(cur_score);
    res
}

#[cfg(feature = "local")]
fn eval_strategy(strategy: &[i64], base_seed: u64) -> f64 {
    // for i in 0..strategy.len() {
    //     dbg!(i, c(strategy[i]));
    // }
    let mut sum = 0.0;
    const CNT_ITERS: usize = 500;
    // calc sum in parallel
    let sum = (0..CNT_ITERS)
        .into_par_iter()
        .map(|i| {
            let mut rng = Random::new(123123 * base_seed + i as u64);
            let need = gen_rng_array(&mut rng, M);
            let groups = split_into_groups_hill_climbing(strategy, &need, 1_000);
            let score = calc_score(strategy, &groups, &need);
            score as f64
        })
        .sum::<f64>();
    // for i in 0..CNT_ITERS {
    //     if i != 77 {
    //         // continue;
    //     }
    //     let mut rng = Random::new(123123 * base_seed + i as u64);
    //     let need = gen_rng_array(&mut rng, M);
    //     let groups = split_into_groups(strategy, &need);
    //     let score = calc_score(strategy, &groups, &need);
    //     sum += score as f64;
    // }
    sum / CNT_ITERS as f64
}

#[cfg(feature = "local")]
fn stress2() {
    for it in 1.. {
        dbg!(it);
        let strategy = generate_simple_strategy();
        let score = eval_strategy(&strategy, it);
        dbg!(score);
    }
}

#[cfg(feature = "local")]
fn stress() {
    let mut strategy = generate_simple_strategy();
    let mut rng = Random::new(1231);
    const BASE_SEED: u64 = 1244348;
    for iter in 0.. {
        let idx = rng.gen_range(0..strategy.len());
        let idx2 = rng.gen_range(0..strategy.len());
        if idx == idx2 {
            continue;
        }
        let coef = rng.gen_double() * 2.0;
        let mut new_val = ((strategy[idx] as f64) * coef).round() as i64;
        let mut new_val2 = strategy[idx2];
        if rng.gen_bool() {
            let mut sum = (strategy[idx] + strategy[idx2]) as f64;
            sum *= 0.98 + rng.gen_double() * 0.04;
            let part = rng.gen_double();
            new_val = (sum * part).round() as i64;
            new_val2 = (sum * (1.0 - part)).round() as i64;
        }
        let mut cur_score = eval_strategy(&strategy, BASE_SEED);

        let old_val = strategy[idx];
        let old_val2 = strategy[idx2];
        strategy[idx] = new_val;
        strategy[idx2] = new_val2;
        let new_score = eval_strategy(&strategy, BASE_SEED);
        if new_score > cur_score {
            cur_score = new_score;
            dbg!(iter, c(new_score as i64));
        } else {
            strategy[idx] = old_val;
            strategy[idx2] = old_val2;
        }

        if iter % 1000 == 0 {
            strategy.sort();
            strategy.reverse();
            let mut f = Output::new_file("strategy.txt");
            f.print("[");
            for i in 0..strategy.len() {
                f.print(strategy[i]);
                if i != strategy.len() - 1 {
                    f.println(", ");
                }
            }
            f.println("]");
            f.flush();
        }
    }
    // dbg!(score);
}

fn calc_score(strategy: &[i64], groups: &[usize], need: &[i64]) -> i64 {
    let mut sums = vec![0; need.len()];
    for i in 0..strategy.len() {
        if groups[i] > 0 {
            sums[groups[i] - 1] += strategy[i];
        }
    }
    let mut res = 1;
    let mut max_delta = 0;
    for i in 0..need.len() {
        let delta = (sums[i] - need[i]).abs();
        res += delta;
        if delta > max_delta {
            max_delta = delta;
        }
    }
    let res = res as f64;
    // dbg!(res.log10(), c(max_delta));
    (((20.0 - res.log10()) * 5e7).round()) as i64
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let l = input.i64();
    let r = input.i64();

    assert!(n == N);
    assert!(m == M);
    assert!(l == L);
    assert!(r == R);
    assert!(r - l == DELTA);

    let strategy = generate_simple_strategy();
    out.println(strategy.clone());
    out.flush();

    let need = input.vec::<i64>(m);
    let groups = split_into_groups_hill_climbing(&strategy, &need, 1_000_000);
    let score = calc_score(&strategy, &groups, &need);
    dbg!(score);
    out.println(groups);
    out.flush();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "ahc053";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
