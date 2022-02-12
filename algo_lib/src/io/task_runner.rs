use std::io::Write;

use super::{
    input::Input,
    output::{Output, OUTPUT},
    task_io_settings::{TaskIoSettings, TaskIoType},
};

pub fn run_task<Res>(io: TaskIoSettings, run: impl FnOnce(Input) -> Res) -> Res {
    let output: Box<dyn Write> = match io.output {
        TaskIoType::Std => Box::new(std::io::stdout()),
        TaskIoType::File(file) => {
            let out_file = std::fs::File::create(file).unwrap();
            Box::new(out_file)
        }
    };

    unsafe {
        if io.is_interactive {
            OUTPUT = Some(Output::new_with_auto_flush(output));
        } else {
            OUTPUT = Some(Output::new(output));
        }
    }

    match io.input {
        TaskIoType::Std => {
            let mut sin = std::io::stdin();
            let input = if io.is_interactive {
                Input::new_with_size(&mut sin, 1)
            } else {
                Input::new(&mut sin)
            };
            run(input)
        }
        TaskIoType::File(file) => {
            let mut in_file = std::fs::File::open(file).unwrap();
            let input = if io.is_interactive {
                Input::new_with_size(&mut in_file, 1)
            } else {
                Input::new(&mut in_file)
            };
            run(input)
        }
    }
}
