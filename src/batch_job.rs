extern crate serde_json;

use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    Done,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    source_path: String,
    source_sha256sum: String,
    destination_path: String,
    destination_sha256sum: String,

    status: JobStatus,
}

#[derive(Serialize, Deserialize)]
pub struct BatchJob {
    source_dir: String,
    destination_dir: String,
    rules: HashMap<String, String>,
    jobs: Vec<RefCell<Job>>,
}

impl BatchJob {
    pub fn new(source_dir: String, destination_dir: String) -> BatchJob {
        let mut bj = BatchJob {
            source_dir: source_dir,
            destination_dir: destination_dir,
            rules: HashMap::new(),
            jobs: vec![],
        };

        bj.rules.insert(
            String::from("avi"),
            String::from("ffmpeg -i $file_path -c:v libx264 -preset slow $file_path_out"),
        );

        bj
    }

    pub fn init(&mut self) -> Result<(), Box<Error>> {
        let mut lst = vec![];

        // flatten
        {
            let mut add_it = |x: &DirEntry| {
                lst.push(x.path());
            };

            self.visit_dirs(Path::new(&self.source_dir.trim()), &mut add_it)?;
        }

        println!("scanned {} files", lst.len());

        for file in lst.iter() {
            let source_path = file.to_str().unwrap();
            println!("processing {}", source_path);
            let mut file = File::open(source_path).expect("Unable to create config file");
            let hash = Sha256::digest_reader(&mut file)?;

            let job = Job {
                source_path: String::from(source_path),
                source_sha256sum: String::from(format!("{:x}", hash)),
                destination_path: String::new(),
                destination_sha256sum: String::new(),
                status: JobStatus::Pending,
            };

            self.jobs.push(RefCell::new(job));
        }

        Ok(())
    }

    pub fn load_from_file(dir: &str) -> Result<BatchJob, Box<Error>> {
        let filepath = format!("{}/dbfc.config", dir);
        let mut file = File::open(filepath)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let bj: BatchJob = serde_json::from_str(&buffer)?;

        Ok(bj)
    }

    pub fn save_to_file(&self) -> Result<(), Box<Error>> {
        let json_result = serde_json::to_string_pretty(&self)?;

        let config_filepath = format!("{}/dbfc.config", self.source_dir);
        let mut file = File::create(config_filepath).expect("Unable to create config file");
        file.write_all(json_result.as_bytes())
            .expect("Unable to write to config file");

        Ok(())
    }

    pub fn run(&mut self) {
        let pending_jobs = self.jobs
            .iter()
            .filter(|j| j.borrow().status == JobStatus::Pending)
            .collect::<Vec<&RefCell<Job>>>();

        println!("{} pending jobs", pending_jobs.len());

        for job_ref in pending_jobs {
            let mut job = job_ref.borrow_mut();

            match self.run_job(&job, self.source_dir.as_str()) {
                Ok(_) => {
                    job.status = JobStatus::Done;
                }
                Err(why) => {
                    job.status = JobStatus::Error;
                    println!("   {}", why);
                }
            }
        }

        if let Err(why) = self.save_to_file() {
            println!("Failed to save batch job state: {}", why);
        }
    }

    fn run_job(&self, job: &Job, source_dir: &str) -> Result<(), String> {
        let path = Path::new(job.source_path.as_str());
        let sub_path = match path.strip_prefix(source_dir) {
            Ok(sp) => sp,
            Err(e) => {
                return Err(format!("   Could not get sub-path: {}", e));
            }
        };

        println!("Processing: {:?}", sub_path);

        if !path.exists() {
            return Err(format!("   Path does not exist"));
        } else if !path.is_file() {
            return Err(format!("   Path is not a file"));
        }

        let extension = match path.extension() {
            Some(e) => match e.to_str() {
                Some(es) => es,
                None => {
                    return Err(format!("   Error converting to string"));
                }
            },
            None => {
                return Err(format!("   No extension"));
            }
        };

        let rule = match self.rules.get(extension) {
            Some(r) => r,
            None => {
                return Err(format!("   No rule defined for extension '{}'", extension));
            }
        };

        let out_path = Path::new(self.destination_dir.as_str()).join(sub_path);
        println!("   Writting to {:?}", out_path);

        // get args and replace any variables
        let raw_args: Vec<&str> = rule.split_whitespace().collect();
        let mut args = Vec::with_capacity(raw_args.len());
        for arg in raw_args.iter() {
            args.push(String::from(match *arg {
                "$file_path" => path.to_str().unwrap(),
                "$file_path_out" => out_path.to_str().unwrap(),
                _ => arg,
            }));
        }
        println!("   {:?}", args);

        // create the output path
        if let Err(why) = fs::create_dir_all(out_path.parent().unwrap()) {
            return Err(format!("   Unable to create out file directory: {}", why));
        }

        let command_name = String::from(args[0].as_str());
        let command_args = &args[1..args.len()];
        let command = Command::new(&command_name)
            .args(command_args)
            .output()
            .expect(format!("Failed to execute command").as_str());

        println!("stdout: {}", String::from_utf8_lossy(&command.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&command.stderr));

        if !command.status.success() {
            return Err(format!(
                "    Command '{}' failed with exit code {:?}",
                &command_name,
                command.status.code()
            ));
        }

        Ok(())
    }

    fn visit_dirs(&self, dir: &Path, cb: &mut FnMut(&DirEntry)) -> Result<(), Box<Error>> {
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
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        output.wait().unwrap();
    }
}
