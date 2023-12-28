let output: Box<dyn std::io::Write> = Box::new(std::fs::File::create().unwrap());
let mut output = algo_lib::io::output::Output::new_file(output);