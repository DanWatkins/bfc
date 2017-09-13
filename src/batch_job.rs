use std::fs;
use std::path;
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
        // run conversion
        let paths = fs::read_dir(self.source_dir.trim()).unwrap();

        for path in paths {
            self.convert_video(&path.unwrap().path(), self.destination_dir.trim());
        }
    }

    fn convert_video(&self, source_filepath: &path::Path, output_dir: &str) {
        let output_filepath = format!("{}{:?}.mp4", output_dir, source_filepath.file_stem().unwrap());

        let mut output = Command::new("ffmpeg")
            .arg("-i")
            .arg(source_filepath)
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
            .arg("-c:a")
            .arg("copy")
            .arg(output_filepath)
            .spawn()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

        output.wait().unwrap();
    }
}