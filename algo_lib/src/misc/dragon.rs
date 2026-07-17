use crate::io::{input::Input, output::Output};

pub fn run_dragon(
    solve: fn(&mut Input, &mut Output),
    problem_name: &str,
    tests: std::ops::Range<usize>,
) {
    const PREFIX: &str = "/home/borys/Downloads/";
    for test_id in tests {
        dbg!("Running problem", problem_name, "test", test_id);
        let mut input = Input::new_file(format!("{PREFIX}{problem_name}{test_id}.in"));
        let mut output = Output::new_file(format!("{PREFIX}{problem_name}{test_id}.out"));
        solve(&mut input, &mut output);
        output.flush();
        dbg!("Done");
    }
}
