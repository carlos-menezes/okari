use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use walkdir::WalkDir;

use crate::converter::Converter;

pub struct Builder {}

impl Builder {
    pub fn run<P>(path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let base_path = Path::new("./build");
        let walker = WalkDir::new(&path);
        for entry in walker {
            let entry = entry?;
            if !entry.metadata()?.is_dir() {
                // We only want the following capture group (pretend `okari` is the source folder): okari/(tree/to/some/file)/file.md

                // Split by `/` and discard the first element, so we're left with `(tree/to/some/file)/file.md`
                let mut path_without_build_dir = entry
                    .path()
                    .components()
                    .skip(1) // <folder> is not nedeed, so skip the first part of the path (i.e. before the first `/`)
                    .map(|c| c.as_os_str().to_str().unwrap())
                    .collect::<Vec<&str>>();

                // Remove the last element of the vector, effectively leaving `path_without_build_dir` with an array of strings `(tree/to/some/file)`
                path_without_build_dir.drain(path_without_build_dir.len() - 1..);
                let output_dir = path_without_build_dir.join(std::path::MAIN_SEPARATOR_STR);

                // Get the file name (without the extension)
                let file_name = format!(
                    "{}.html",
                    entry.path().file_stem().unwrap().to_str().unwrap()
                );

                // We can now read the file
                let file_content: String = std::fs::read(&entry.path())?
                    .iter()
                    .map(|b| *b as char)
                    .collect();

                let html = Converter::run(&file_content);

                let file_path = base_path.clone().join(&output_dir).join(&file_name);
                // Calling `.parent()` will return the entire path except the actual file name
                std::fs::create_dir_all(&file_path.parent().unwrap())?;

                let mut file = File::create(&file_path)?;
                file.write(html.as_bytes())?;

                println!(
                    "> Generated {} (to: {})",
                    &file_name,
                    &file_path.to_str().unwrap()
                );
            }
        }
        Ok(())
    }
}
