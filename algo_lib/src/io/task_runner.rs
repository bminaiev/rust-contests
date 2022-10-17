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
        OUTPUT = Some(Output::new(output));
    }

    let input = match io.input {
        TaskIoType::Std => {
            let sin = std::io::stdin();
            Input::new(Box::new(sin))
        }
        TaskIoType::File(file) => Input::new_file(file),
    };

    run(input)
}
