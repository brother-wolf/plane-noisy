use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_to_file(contents: &str, filename: &str) -> std::io::Result<()> {
    match File::create(filename) {
        Ok(file) => {
            let mut writer = BufWriter::new(&file);
            match writer.write_all(contents.as_bytes()) {
                Ok(_) => file.sync_all(),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}