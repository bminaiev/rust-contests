let output: Box<dyn std::io::Write> = Box::new(std::fs::File::create("$OUT_FILE").unwrap());

unsafe {
    algo_lib::io::output::OUTPUT = Some(algo_lib::io::output::Output::new(output));
}