use std::fs;

pub struct HtmlMerger {
    output_dir: String,
}

impl HtmlMerger {
    pub fn new(path: String) -> Self {
        Self { output_dir: path }
    }

    pub fn regenerate(&self) {
        let mut files_to_merge = vec![];
        for path in fs::read_dir(&self.output_dir).unwrap() {
            let path = path.unwrap().path();
            if path.as_os_str().to_str().unwrap().ends_with("-short.html") {
                files_to_merge.push(path);
            }
        }
        files_to_merge.sort();
        let mut merged = String::new();
        for path in files_to_merge.iter() {
            let content = fs::read_to_string(path).unwrap();
            merged += &content;
        }
        fs::write(format!("{}/index.html", self.output_dir), merged)
            .expect("Can't save index.html");
    }
}
