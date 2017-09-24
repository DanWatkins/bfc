extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod batch_job;

use batch_job::*;
use serde_json::Error;
use std::fs::File;
use std::io;
use std::io::Write;

fn write_to_json_batch_job(batch_job: &BatchJob) -> Result<String, Error> {
    let json_result = serde_json::to_string_pretty(&batch_job)?;

    Ok(json_result)
}

fn main() {
    // get the source dir
    let mut source_dir = String::new();
    io::stdin().read_line(&mut source_dir).expect("failed to get source dir");

    // get the destination dir
    let mut destination_dir = String::new();
    io::stdin().read_line(&mut destination_dir).expect("failed to get destination dir");

    let mut bj = self::batch_job::BatchJob::new(source_dir.clone(), destination_dir.clone());
    bj.run();
    println!("JSON result:");
    
    match write_to_json_batch_job(&bj) {
        Ok(result) => {
            let config_filepath = format!("{}/dbfc.config", source_dir.trim());
            println!("Config filepath: {}", config_filepath);
            let mut file = File::create(config_filepath).expect("Unable to create config file");
            file.write_all(result.as_bytes()).expect("Unable to write to config file");
        },
        Err(e) => {
            println!("{:?}", e)
        }
    }
}