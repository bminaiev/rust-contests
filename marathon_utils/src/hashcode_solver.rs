use crate::html_report::HtmlReport;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::{set_global_output_to_file, set_global_output_to_none};
use std::fs;
use std::fs::create_dir_all;
use std::ops::Range;
use std::path::{Path, PathBuf};

// TODO: better directory structure

pub struct OneTest {
    base_dir: String,
    output_dir: String,
    name: String,
    output_path: PathBuf,
    html: HtmlReport,
}

impl OneTest {
    pub fn new(base_dir: String, output_dir: String, name: String, output_path: String) -> Self {
        let mut html = HtmlReport::new(
            format!("{}/{}", &base_dir, &output_dir),
            name.clone(),
            format!("{}.html", &name),
        );
        html.add_link("all tests", "index.html");
        html.add_text(&format!("Test: {}", &name));
        if cfg!(debug_assertions) {
            html.add_text("Report was generated in DEBUG mode. Are you sure you don't want to compile in Release???");
        }
        Self {
            html,
            base_dir,
            output_dir,
            name,
            output_path: PathBuf::from(&output_path).canonicalize().unwrap(),
        }
    }

    // TODO: make this atomic.. (or we can lose result)
    pub fn save_result(&mut self, f: &mut dyn FnMut()) {
        set_global_output_to_file(&self.output_path.to_str().unwrap());
        f();
        let symlink_path = &format!("/home/borys/{}.out", self.name);
        if Path::new(symlink_path).exists() {
            fs::remove_file(symlink_path).expect(&format!("Can't delete file: {}", symlink_path));
        }
        std::os::unix::fs::symlink(&self.output_path, symlink_path)
            .expect("Can't create symbolic link");
        set_global_output_to_none();
    }

    pub fn additional_file_name(&self, suffix: &str) -> String {
        format!(
            "{}/{}/{}{}",
            self.base_dir, self.output_dir, self.name, suffix
        )
    }

    pub fn html(&mut self) -> &mut HtmlReport {
        &mut self.html
    }
}

pub fn hashcode_solver(
    base_dir: &str,
    input_dir: &str,
    output_dir: &str,
    tasks: Range<u8>,
    solver: &mut dyn FnMut(&mut Input, &mut OneTest),
) {
    println!("Hello to the hashcode solver!");
    if cfg!(debug_assertions) {
        println!(
            "\nCurrently running in DEBUG mode. Are you sure you don't want to compile in Release???\n"
        );
    }
    let inputs = {
        let input_dir = &format!("{}/{}", base_dir, input_dir);
        fs::read_dir(input_dir).expect(&format!("Can't read {}", input_dir))
    };

    let good_test = |input: &str| -> bool {
        let first_char = input.as_bytes()[0];
        tasks.contains(&first_char)
    };

    let mut all_tests = vec![];
    for path in inputs {
        let path = path.unwrap();
        let file_name = path.file_name().into_string().unwrap();
        let good_test = good_test(&file_name);
        if good_test {
            println!("Use test: {}", &file_name);
            all_tests.push(file_name);
        } else {
            println!("Skipping test: {}", &file_name);
        }
    }
    all_tests.sort();

    println!();

    create_dir_all(output_dir).expect(&format!("Can't create outputs dir: {}", &output_dir));
    set_global_output_to_none();

    // TODO: it will contain only subset of tests?
    let mut index_html = HtmlReport::new(
        format!("{}/{}", &base_dir, &output_dir),
        "".to_string(),
        "index.html".to_string(),
    );
    for test_name in all_tests.iter() {
        println!("Running test {}", test_name);

        let input_file = &format!("{}/{}/{}", base_dir, input_dir, test_name);
        let mut input = Input::new_file(input_file);

        let mut test = OneTest::new(
            base_dir.to_string(),
            output_dir.to_string(),
            test_name.clone(),
            format!("{}/{}/{}.out", base_dir, output_dir, test_name),
        );

        index_html.add_link(test_name, test.html.relative_path());
        solver(&mut input, &mut test);
        test.html.save().expect("Can't save html report");

        println!("Test finished\n");
    }
    index_html.save().expect("Can't save index.html");
}
