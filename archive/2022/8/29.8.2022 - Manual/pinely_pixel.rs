//{"name":"pinely_pixel","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"pinely_pixel"}}}

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
use reqwest::Url;
use tungstenite::{connect, Message};

use serde::{Deserialize, Serialize};

// Add token here.
const TOKEN: &'static str = "";

#[derive(Serialize, Deserialize, Debug)]
struct RoundInfo {
    n: usize,
    m: usize,
    ticks: usize,
    auction: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeamInfo {
    team_id: usize,
    coins: usize,
    locked_coins: usize,
    rps: usize,
    score: usize,
    targets: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Painting {
    id: i32,
    painting: Vec<Vec<Vec<i32>>>,
    score: i32,
    rps_increase: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllPaintings {
    paintings: Vec<Painting>,
}

fn send_request(url: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://pixelbattle.pinely.io/{}", url);
    let resp = client
        .get(url)
        .body("{}")
        .header("content-type", "application/json")
        .header("X-Pinely-Token", TOKEN)
        .send()
        .unwrap();
    // println!("{:#?}", resp);
    resp.text().unwrap()
}

fn get_round_info() {
    let text = send_request(&"round_info");
    let info: RoundInfo = serde_json::from_str(&text).unwrap();
    dbg!(info);
}

fn get_team_info() {
    let text = send_request(&"team_info");
    let info: TeamInfo = serde_json::from_str(&text).unwrap();
    dbg!(info);
}

fn get_all_paintings() -> AllPaintings {
    let text = send_request(&"all_paintings");
    let info: AllPaintings = serde_json::from_str(&text).unwrap();
    for p in info.paintings.iter() {
        dbg!(p.id, p.score, p.rps_increase);
    }
    info
}

fn send_recolor(x: i32, y: i32) {
    let client = reqwest::blocking::Client::new();
    let url = "https://pixelbattle.pinely.io/paint";
    let resp = client
        .post(url)
        .body(format!(
            "{{\"x\":{},\"y\":{},\"r\":1,\"g\":2,\"b\":3}}",
            x, y
        ))
        .header("content-type", "application/json")
        .header("X-Pinely-Token", TOKEN)
        .send()
        .unwrap();
    println!("{:#?}", resp);
    let text = resp.text().unwrap();
    println!("test: {}", text);
}

fn websockets() {
    let (mut socket, response) =
        connect(Url::parse("ws://echo.websocket.events").unwrap()).expect("Can't connect");
    socket
        .write_message(Message::Text("Hello world".into()))
        .unwrap();
    let msg = socket.read_message().unwrap();
    eprint!("{:?}", msg);
}

fn websockets2() {
    let (mut socket, response) =
        connect(Url::parse("wss://pixelbattle.pinely.io/events?token=viewer").unwrap())
            .expect("Can't connect");
    // socket
    //     .write_message(Message::Text("Hello world".into()))
    //     .unwrap();
    let msg = socket.read_message().unwrap();
    eprint!("{:?}", msg);
}

#[derive(Clone, Copy, Default)]
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn win() {
    let filename = "/home/borys/log.txt";

    let mut start = Instant::now();
    let mut last_showed_debug = 0;

    loop {
        let mut input = Input::new_file(filename);
        let n = input.usize();
        let m = input.usize();
        dbg!(n, m);
        let mut field = Array2D::new(Color::default(), n, m);
        const MAX_TEAM_ID: usize = 30;
        let mut scores = vec![0; MAX_TEAM_ID];
        let mut bought: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for team_id in 0..MAX_TEAM_ID {
            bought.insert(team_id, BTreeSet::new());
        }

        let paintings = get_all_paintings();

        loop {
            let mut cnt = 0;
            while !input.has_more_elements() {
                cnt += 1;
                std::thread::sleep(Duration::from_millis(50));
                if cnt == 10 {
                    dbg!("too many....");
                    break;
                }
            }
            if !input.has_more_elements() {
                break;
            }
            let q_type = input.i32();
            if q_type == -1 {
                dbg!("Round finished...");
                break;
            }
            if q_type == 1 {
                let team_id = input.usize();
                let score = input.i64();
                dbg!("score update", team_id, score);
                scores[team_id] = score;
            }
            if q_type == 2 {
                let team_id = input.usize();
                let painting_id = input.usize();
                dbg!("bought", team_id, painting_id);
                bought.entry(team_id).or_default().insert(painting_id);
            }
            if q_type == 3 {
                let x = input.usize();
                let y = input.usize();
                let r = input.i32();
                let g = input.i32();
                let b = input.i32();
                field[x][y] = Color { r, g, b };
                // recolor...
            }
            {
                let mut another_team_id = 9;
                for team_id in 0..MAX_TEAM_ID {
                    if team_id != 13 && scores[team_id] > scores[another_team_id] {
                        if let Some(b) = bought.get(&team_id) {
                            if !b.is_empty() {
                                another_team_id = team_id;
                            }
                        }
                    }
                }
                let cur_ms = start.elapsed().as_millis();
                if cur_ms > last_showed_debug + 300 {
                    last_showed_debug = cur_ms;
                    dbg!(another_team_id);

                    let mut best_change = (std::usize::MAX, std::usize::MAX);
                    let mut best_f = 0.3;

                    if let Some(they_bought) = bought.get(&another_team_id) {
                        dbg!(they_bought);
                        for &id in they_bought.iter() {
                            dbg!(paintings.paintings[id].score);

                            let need = &paintings.paintings[id].painting;
                            let need_n = need.len();
                            let need_m = need[0].len();

                            let mut max_cnt_ok = 0;
                            let mut max_start_x = 0;
                            let mut max_start_y = 0;

                            for start_x in 0..(n - need_n + 1) {
                                for start_y in 0..(m - need_m + 1) {
                                    let mut can_change = (0, 0);
                                    let mut cnt_ok = 0;
                                    for dx in 0..need_n {
                                        for dy in 0..need_m {
                                            let cur_need = &need[dx][dy];
                                            let cur = &field[start_x + dx][start_y + dy];
                                            if cur.r == cur_need[0]
                                                && cur.g == cur_need[1]
                                                && cur.b == cur_need[2]
                                            {
                                                cnt_ok += 1;
                                                can_change = (start_x + dx, start_y + dy);
                                            }
                                        }
                                    }
                                    if cnt_ok > max_cnt_ok {
                                        max_cnt_ok = cnt_ok;
                                        max_start_x = start_x;
                                        max_start_y = start_y;
                                        let full = need_n * need_m;
                                        let f = (cnt_ok as f64) / (full as f64);
                                        if f > best_f {
                                            best_change = can_change;
                                            best_f = f;
                                        }
                                    }
                                }
                            }
                            dbg!(
                                max_cnt_ok,
                                need_n * need_m,
                                max_start_x,
                                max_start_y,
                                best_change
                            );
                        }
                    }
                    if best_change.0 != std::usize::MAX {
                        dbg!("recolor!!!", best_change, best_f, another_team_id);
                        send_recolor(best_change.0 as i32, best_change.1 as i32);
                    }
                }
            }
        }
        dbg!("Sleeping 5s...");
        std::thread::sleep(Duration::from_millis(5_000));
    }
}

fn main_fun() {
    // dbg!("hello!");
    // get_round_info();
    // get_team_info();
    // get_all_paintings();
    // websockets2();
    win();
}

fn solve(input: &mut Input, _test_case: usize) {}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    main_fun();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
