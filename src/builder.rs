use std::{fs::File, io::Write, path::Path};

use crate::converter::Converter;

pub struct Builder {}

impl Builder {
    pub fn run<P>(path: P)
    where
        P: AsRef<Path>,
    {
        let entries = std::fs::read_dir(path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            // Read the file as UTF-8
            let file_bytes = std::fs::read(&path).unwrap();
            let file_str = match String::from_utf8(file_bytes) {
                Ok(s) => s,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let content = Converter::run(&file_str);
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split_once('.')
                .unwrap()
                .0;
            let file_path = Path::new("./build").join(format!("{}.html", file_name));
            let parent = file_path.parent().unwrap();
            std::fs::create_dir_all(parent).unwrap();
            let mut file = File::create(file_path).unwrap();
            file.write(content.as_bytes()).unwrap();
            println!("> Generated {}.html", file_name)
        }
    }
}
