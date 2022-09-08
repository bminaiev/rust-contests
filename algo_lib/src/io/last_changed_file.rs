use std::{fs, path::PathBuf, time::SystemTime};

pub fn find_last_changed_file(dir: &str) -> Option<PathBuf> {
    let paths = fs::read_dir(dir).unwrap();

    struct FileWithModificationDate {
        modified: SystemTime,
        file: PathBuf,
    }

    let mut files = vec![];
    for path in paths {
        let file = path.unwrap().path().clone();
        let metadata = fs::metadata(&file).unwrap();
        if metadata.is_dir() {
            continue;
        }
        let modified = metadata.modified().unwrap();
        files.push(FileWithModificationDate { file, modified });
    }
    files.sort_by_key(|f| f.modified);
    files.last().map(|f| f.file.clone())
}
