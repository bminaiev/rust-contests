use crate::distribution_stat::DistributionStat;
use crate::html_report::{HtmlReport, ImageData};
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::{set_global_output_to_file, set_global_output_to_none};
use std::fmt::Display;
use std::fs;
use std::fs::create_dir_all;
use std::ops::Range;
use std::path::{Path, PathBuf};

// TODO: better directory structure

pub struct Report<'a> {
    html: HtmlReport,
    global_html: &'a mut HtmlReport,
}

impl<'a> Report<'a> {
    pub fn add_value<V: Display>(&mut self, name: &str, value: &V) {
        self.html.add_value(name, value);
        self.global_html.add_value(name, value);
    }

    pub fn add_image(&mut self, name: &str, image: ImageData) {
        self.html.add_image(name, image);
    }

    pub fn add_distribution_stat<T: Ord + Clone>(&mut self, stat: &DistributionStat<T>)
    where
        f64: From<T>,
    {
        self.html.add_distribution_stat(stat);
    }
}

pub struct OneTest<'a> {
    base_dir: String,
    output_dir: String,
    name: String,
    output_path: PathBuf,
    pub report: Report<'a>,
}

impl<'a> OneTest<'a> {
    pub fn new(
        base_dir: String,
        output_dir: String,
        name: String,
        output_path: String,
        global_html: &'a mut HtmlReport,
    ) -> Self {
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
            base_dir,
            output_dir,
            name,
            output_path: PathBuf::from(&output_path).canonicalize().unwrap(),
            report: Report { html, global_html },
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
            &mut index_html,
        );

        test.report
            .global_html
            .add_link(test_name, test.report.html.relative_path());
        solver(&mut input, &mut test);
        test.report.html.save().expect("Can't save html report");

        println!("Test finished\n");
    }
    index_html.save().expect("Can't save index.html");
}
