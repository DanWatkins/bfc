use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct BatchJob {
    source_dir: String,
    destination_dir: String,
}

impl BatchJob {
    pub fn new(source_dir: String, destination_dir: String) -> BatchJob {
        BatchJob {
            source_dir: source_dir,
            destination_dir: destination_dir,
        }
    }

    pub fn run(&self) {
        let mut lst = vec!();

        // flatten
        {
            let mut add_it = |x: &DirEntry| {
                lst.push(x.path());
            };

            match self.visit_dirs(Path::new(&self.source_dir.trim()), &mut add_it) {
                Ok(x) => {},
                Err(e) => {
                    println!("{:?}", e);
                    return;
                }
            };
        }

        println!("scanned {} files", lst.len());

        for file in lst.iter() {
            println!("converting {}", file.display());
            self.convert_video(&file, self.destination_dir.trim());
        }
    }

    fn visit_dirs(&self, dir: &Path, cb: &mut FnMut(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }

    fn convert_video(&self, source_filepath: &Path, output_dir: &str) {
        let mut out_file = PathBuf::new();
        out_file.push(output_dir);
        out_file.push(source_filepath.file_stem().unwrap());
        out_file.set_extension("mp4");

        let mut output = Command::new("ffmpeg")
            .arg("-loglevel")
            .arg("warning")
            .arg("-i")
            .arg(source_filepath)
            .arg("-y")
            .arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("ultrafast")
            .arg("-framerate")
            .arg("30")
            .arg("-vsync")
            .arg("1")
            .arg("-crf")
            .arg("20")
            .arg(out_file.as_path())
            .spawn()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

        output.wait().unwrap();
    }
}