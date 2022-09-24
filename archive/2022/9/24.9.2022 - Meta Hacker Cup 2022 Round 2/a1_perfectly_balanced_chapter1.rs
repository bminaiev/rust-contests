//{"name":"A1: Perfectly Balanced - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\nsingingbanana\n5\n8 12\n9 13\n8 10\n10 12\n1 7\nprepareintelligentopinion\n4\n1 7\n8 18\n19 25\n12 13\nphpservers\n6\n1 3\n4 10\n1 3\n2 2\n3 5\n1 10\n","output":"Case #1: 4\nCase #2: 3\nCase #3: 4\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"perfectly_balanced_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"perfectly_balanced_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1PerfectlyBalancedChapter1"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::rand::Random;
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
#[derive(Clone, Copy, Default)]
struct XorNode {
    xor: usize,
}

impl LazySegTreeNodeSpec for XorNode {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self { xor: l.xor ^ r.xor }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.xor = *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = usize;

    type Context = ();
}

#[derive(Clone, Copy, Default)]
struct SumNode {
    sum: u64,
}

impl LazySegTreeNodeSpec for SumNode {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        Self {
            sum: l.sum.wrapping_add(r.sum),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum = *update;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = u64;

    type Context = ();
}

struct Solver {
    mapping: Vec<u64>,
    sum_st: LazySegTree<SumNode>,
    xor_st: LazySegTree<XorNode>,
}

impl Solver {
    pub fn new(start: &[usize]) -> Self {
        const MAX_VAL: usize = 1_000_005;
        let mut rnd = Random::new(7897718);
        let mapping = gen_vec(MAX_VAL, |_| rnd.gen_u64());
        let mut sum_st = LazySegTree::new(&SumNode::default(), start.len(), ());
        let mut xor_st = LazySegTree::new(&XorNode::default(), start.len(), ());
        for i in 0..start.len() {
            sum_st.update(i..i + 1, mapping[start[i]]);
            xor_st.update(i..i + 1, start[i]);
        }
        Self {
            mapping,
            sum_st,
            xor_st,
        }
    }

    fn is_good_segment(&mut self, l: usize, r: usize) -> bool {
        let len = r - l;
        if len == 1 {
            return true;
        }
        if len % 2 == 0 {
            return false;
        }
        let xor = self.xor_st.get(l..r).xor;
        if xor >= self.mapping.len() {
            return false;
        }
        let conv_xor = self.mapping[xor];
        let mid = (l + r) / 2;
        for add in 0..2 {
            let mut left = self.sum_st.get(l..mid + add).sum;
            let mut right = self.sum_st.get(mid + add..r).sum;
            if add == 0 {
                right = right.wrapping_sub(conv_xor);
            } else {
                left = left.wrapping_sub(conv_xor);
            }
            if left == right {
                return true;
            }
        }
        return false;
    }
}

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        s: Vec<u8>,
        queries: Vec<(usize, usize)>,
        res: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.s = input.string();
            let cnt_q = input.usize();
            self.queries = gen_vec(cnt_q, |_| (input.usize() - 1, input.usize()));
        }

        fn solve(&mut self) {
            let mut res = 0;
            let start: Vec<_> = self.s.iter().map(|&x| x as usize).collect();
            let mut solver = Solver::new(&start);
            for &(l, r) in self.queries.iter() {
                if solver.is_good_segment(l, r) {
                    res += 1;
                }
            }
            self.res = res;
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.res));
        }
    }

    run_parallel::<Job>(input, Some(1));
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
        output: TaskIoType::File("perfectly_balanced_chapter__output.txt".to_string()),
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
