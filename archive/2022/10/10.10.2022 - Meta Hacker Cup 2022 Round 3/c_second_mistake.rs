//{"name":"C: Second Mistake","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n1\nmetamate\n5\nteammate\nmeatmate\nmetatame\nmememate\nmetameme\n3\nmeet\nemma\ntate\n2\ntata\nmaam\n3\nmem\nmet\nmat\n3\ntam\nmat\ntea\n","output":"Case #1: 4\nCase #2: 0\nCase #3: 5\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_mistake_.*input[.]txt"},"output":{"type":"file","fileName":"second_mistake_output.txt","pattern":null},"languages":{"java":{"taskClass":"CSecondMistake"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::fx_hash_map::{FxHashMap, FxHashSet};
use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    run_parallel::<Job>(input, Some(4), &());
}

#[derive(Clone, Default)]
struct Job {
    words: Vec<Vec<usize>>,
    queries: Vec<Vec<usize>>,
    res: i64,
}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {
        let n = input.usize();

        let pattern = vec![b'm', b'e', b't', b'a'];
        let read_str = |input: &mut Input| -> Vec<usize> {
            let s = input.string();
            gen_vec(s.len(), |pos| pattern.index_of(&s[pos]).unwrap())
        };
        for _ in 0..n {
            self.words.push(read_str(input));
        }
        let q = input.usize();
        for _ in 0..q {
            self.queries.push(read_str(input));
        }
    }

    fn solve(&mut self, context: &Self::Context) {
        let mut rnd = Random::new(7817788);
        let n = self.words[0].len();
        let mut m1 = Array2D::new(0, 4, n);
        let mut m2 = Array2D::new(0, 4, n);
        for i in 0..4 {
            for j in 0..n {
                m1[i][j] = rnd.gen_u64();
                m2[i][j] = rnd.gen_u64();
            }
        }
        let mut all_words = FxHashSet::default();
        let mut changed_words: std::collections::HashMap<
            (u64, u64),
            i32,
            std::hash::BuildHasherDefault<algo_lib::collections::fx_hash_map::FxHasher>,
        > = FxHashMap::default();
        for w in self.words.iter() {
            let mut h = (0, 0);
            for i in 0..n {
                h.0 ^= m1[w[i]][i];
                h.1 ^= m2[w[i]][i];
            }
            assert!(all_words.insert(h));
            for i in 0..n {
                let mut h2 = h;
                let my = w[i];
                h2.0 ^= m1[my][i];
                h2.1 ^= m2[my][i];
                for new_ in 0..4 {
                    if new_ == my {
                        continue;
                    }
                    let mut h3 = h2;
                    h3.0 ^= m1[new_][i];
                    h3.1 ^= m2[new_][i];
                    *changed_words.entry(h3).or_default() += 1i32;
                }
            }
        }
        for w in self.queries.iter() {
            let mut h = (0, 0);
            for i in 0..n {
                h.0 ^= m1[w[i]][i];
                h.1 ^= m2[w[i]][i];
            }
            if all_words.contains(&h) {
                self.res -= 3 * (n as i64);
            }
            for i in 0..n {
                let mut h2 = h;
                let my = w[i];
                h2.0 ^= m1[my][i];
                h2.1 ^= m2[my][i];
                for new_ in 0..4 {
                    if new_ == my {
                        continue;
                    }
                    let mut h3 = h2;
                    h3.0 ^= m1[new_][i];
                    h3.1 ^= m2[new_][i];
                    if all_words.contains(&h3) {
                        self.res -= 2;
                    }
                    let almost = *changed_words.get(&h3).unwrap_or(&0);
                    self.res += almost as i64;
                }
            }
        }
    }

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: {}", test_case, self.res / 2));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("second_mistake_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    tester::run_with_last_downloaded_file();
}
//END MAIN
