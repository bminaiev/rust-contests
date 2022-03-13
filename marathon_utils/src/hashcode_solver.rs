use crate::distribution_stat::DistributionStat;
use crate::dynamic_plot::DynamicPlot;
use crate::html_merger::HtmlMerger;
use crate::html_report::{DynamicPlotId, HtmlReport, ImageData};
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::{set_global_output_to_file, set_global_output_to_none};
use std::fmt::Display;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::{Path, PathBuf};

// TODO: better directory structure

pub struct Report<'a> {
    html: HtmlReport,
    short_html: HtmlReport,
    html_merger: &'a HtmlMerger,
}

impl<'a> Report<'a> {
    pub fn add_value<V: Display>(&mut self, name: &str, value: &V) {
        self.html.add_value(name, value);
        self.short_html.add_value(name, value);
    }

    pub fn add_image(&mut self, name: &str, image: ImageData) {
        self.html.add_image(name, image);
    }

    pub fn add_distribution_stat(&mut self, stat: &DistributionStat<i32>) {
        self.short_html
            .add_value(&stat.name, &stat.to_text_format());
        self.html.add_distribution_stat(stat);
    }

    pub fn add_link(&mut self, text: &str, link: &str) {
        self.html.add_link(text, link);
        self.short_html.add_link(text, link);
    }

    pub fn add_dynamic_plot(&mut self, plot: DynamicPlot) -> DynamicPlotId {
        self.html.add_dynamic_plot(plot)
    }

    pub fn get_dynamic_plot(&mut self, id: DynamicPlotId) -> &mut DynamicPlot {
        self.html.get_dynamic_plot(id)
    }

    pub fn save(&self) {
        self.html.save().expect("Can't save report");
        self.short_html.save().expect("Can't save short report");
        self.html_merger.regenerate();
    }
}

pub struct OneTest<'a> {
    pub name: String,
    output_path: PathBuf,
    pub report: Report<'a>,
}

impl<'a> OneTest<'a> {
    pub fn new(
        base_dir: String,
        output_dir: String,
        name: String,
        output_path: String,
        html_merger: &'a HtmlMerger,
    ) -> Self {
        let mut html = HtmlReport::new(format!("{}/{}", &base_dir, &output_dir), &name);
        html.add_link("all tests", "index.html");
        let short_html = HtmlReport::new(
            format!("{}/{}", &base_dir, &output_dir),
            &format!("{}-short", &name),
        );
        if cfg!(debug_assertions) {
            html.add_text("Report was generated in DEBUG mode. Are you sure you don't want to compile in Release???");
        }
        let mut report = Report {
            html,
            short_html,
            html_merger,
        };
        report.short_html.add_hr();
        report.add_value(&"Test", &name);
        dbg!(&output_path);
        if !Path::new(&output_path).exists() {
            match File::create(&output_path) {
                Ok(_) => {}
                Err(err) => {
                    dbg!(err)
                }
            }
        }
        Self {
            name,
            output_path: PathBuf::from(&output_path).canonicalize().unwrap(),
            report,
        }
    }

    // TODO: make this atomic.. (or we can lose result)
    pub fn save_result(&self, f: &mut dyn FnMut()) {
        set_global_output_to_file(&self.output_path.to_str().unwrap());
        f();
        let symlink_path = &format!("/home/borys/{}.out", self.name);
        let should_remove = Path::new(symlink_path).exists() || fs::read_link(symlink_path).is_ok();
        if should_remove {
            fs::remove_file(symlink_path).expect(&format!("Can't delete file: {}", symlink_path));
        }
        std::os::unix::fs::symlink(&self.output_path, symlink_path)
            .expect("Can't create symbolic link");
        set_global_output_to_none();
        self.report
            .html
            .save()
            .unwrap_or_else(|_| dbg!("Failed  to save report"));
    }

    pub fn load_existing_result(&self, f: impl FnOnce(Input)) {
        if Path::new(&self.output_path).exists() {
            let input = Input::new_file(&self.output_path);
            f(input);
        } else {
            dbg!("No existing solution");
        }
    }
}

pub fn hashcode_solver(
    base_dir: &str,
    input_dir: &str,
    output_dir: &str,
    tasks: impl Iterator<Item = u8>,
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

    let tasks: Vec<_> = tasks.collect();
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

    {
        let full_output_dir = format!("{}/{}", base_dir, output_dir);
        create_dir_all(&full_output_dir)
            .expect(&format!("Can't create outputs dir: {}", &full_output_dir));
    }
    set_global_output_to_none();

    let html_merger = HtmlMerger::new(format!("{}/{}", &base_dir, &output_dir));
    for test_name in all_tests.iter() {
        println!("Running test {}", test_name);

        let input_file = &format!("{}/{}/{}", base_dir, input_dir, test_name);
        let mut input = Input::new_file(input_file);

        let mut test = OneTest::new(
            base_dir.to_string(),
            output_dir.to_string(),
            test_name.clone(),
            format!("{}/{}/{}.out", base_dir, output_dir, test_name),
            &html_merger,
        );

        test.report
            .short_html
            .add_link("details", test.report.html.relative_path());
        solver(&mut input, &mut test);
        test.report
            .add_link(&"input", &format!("../{}/{}", input_dir, test_name));
        test.report
            .add_link(&"output", &format!("{}.out", test_name));
        test.report.save();

        println!("Test finished\n");
    }
}
