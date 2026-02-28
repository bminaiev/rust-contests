//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let type_ = input.usize();
    let arrays = gen_arrays();
    let (chooser, to_use) = stress();
    let colors = ["red", "green", "blue"];
    for _ in 0..tc {
        if type_ == 1 {
            let mut my_array = vec![0; 4];
            for i in 0..4 {
                let s = input.string_as_string();
                let color = colors.iter().position(|&x| x == s).unwrap();
                my_array[i] = color;
            }
            let pos = arrays.iter().position(|a| a == &my_array).unwrap();
            let idx = to_use[pos];
            out.println(idx + 1);
        } else {
            assert_eq!(type_, 2);
            let idx = input.usize() - 1;
            let color_left_str = input.string_as_string();
            let color_right_str = input.string_as_string();
            let color_left = colors.iter().position(|&x| x == color_left_str).unwrap();
            let color_right = colors.iter().position(|&x| x == color_right_str).unwrap();
            let mut ok_colors = vec![];
            for c in 0..3 {
                if chooser.ways[color_left][color_right][idx].cnt[c] != 0 {
                    ok_colors.push(colors[c]);
                }
            }
            assert!(ok_colors.len() <= 1);
            let use_color = if ok_colors.len() == 1 {
                ok_colors[0]
            } else {
                "red"
            };
            out.println(use_color);
        }
        out.flush();
    }
}

#[derive(Copy, Clone)]
struct Cnt {
    cnt: [i32; 3],
}

struct Chooser {
    // left, right, idx
    ways: [[[Cnt; 4]; 3]; 3],
}

impl Chooser {
    fn new() -> Self {
        let ways = [[[Cnt { cnt: [0; 3] }; 4]; 3]; 3];
        Self { ways }
    }

    fn add(&mut self, left: usize, right: usize, idx: usize, color: usize, delta: i32) {
        self.ways[left][right][idx].cnt[color] += delta;
    }

    fn wrongs(&self) -> usize {
        let mut res = 0;
        for left in 0..3 {
            for right in 0..3 {
                for idx in 0..4 {
                    let mut non_zero = 0;
                    for color in 0..3 {
                        if self.ways[left][right][idx].cnt[color] != 0 {
                            non_zero += 1;
                        }
                    }
                    if non_zero > 1 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

fn gen_arrays() -> Vec<Vec<usize>> {
    let mut arrays = vec![];
    for idx in 0..81usize {
        let mut a = vec![];
        let mut tmp = idx;
        for _ in 0..4 {
            a.push(tmp % 3);
            tmp /= 3;
        }
        arrays.push(a);
    }
    arrays
}

fn stress() -> (Chooser, Vec<usize>) {
    let arrays = gen_arrays();
    for _glob_iter in 366799.. {
        let mut rnd = Random::new(_glob_iter);
        let mut to_use = vec![0; arrays.len()];
        for i in 0..to_use.len() {
            to_use[i] = rnd.gen_range(0..4);
        }
        let mut chooser = Chooser::new();
        for i in 0..to_use.len() {
            let idx = to_use[i];
            let left = arrays[i][(idx + 3) % 4];
            let right = arrays[i][(idx + 1) % 4];
            let color = arrays[i][idx];
            chooser.add(left, right, idx, color, 1);
        }
        let mut wrongs = chooser.wrongs();
        for it in 0..10_000 {
            let i = rnd.gen_range(0..to_use.len());
            let prev_idx = to_use[i];
            let prev_left = arrays[i][(prev_idx + 3) % 4];
            let prev_right = arrays[i][(prev_idx + 1) % 4];
            let prev_color = arrays[i][prev_idx];
            chooser.add(prev_left, prev_right, prev_idx, prev_color, -1);
            let new_idx = rnd.gen_range(0..4);
            let new_left = arrays[i][(new_idx + 3) % 4];
            let new_right = arrays[i][(new_idx + 1) % 4];
            let new_color = arrays[i][new_idx];
            chooser.add(new_left, new_right, new_idx, new_color, 1);
            to_use[i] = new_idx;
            let new_wrongs = chooser.wrongs();
            if new_wrongs <= wrongs {
                // dbg!(it, new_wrongs);
                wrongs = new_wrongs;
            } else {
                chooser.add(new_left, new_right, new_idx, new_color, -1);
                chooser.add(prev_left, prev_right, prev_idx, prev_color, 1);
                to_use[i] = prev_idx;
            }
        }
        // dbg!(_glob_iter, wrongs);
        if wrongs == 0 {
            return (chooser, to_use);
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
