let output: Box<dyn std::io::Write> = Box::new(std::io::stdout());

unsafe {
    algo_lib::io::output::OUTPUT = Some(algo_lib::io::output::Output::new(output));
}