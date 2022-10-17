#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        $CARET
        out_line!(format!("Case #{}: ", test_case));
    }
    // run_parallel::<Job>(input, Some(1), &());
}

#[derive(Clone, Default)]
struct Job {}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {}

    fn solve(&mut self, context: &Self::Context) {}

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: ", test_case));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    true
}
