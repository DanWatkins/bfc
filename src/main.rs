#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

pub mod batch_job;

use batch_job::*;
use clap::{App, Arg, SubCommand};
use serde_json::Error;
use std::fs::File;
use std::io::Write;

fn write_to_json_batch_job(batch_job: &BatchJob) -> Result<String, Error> {
    let json_result = serde_json::to_string_pretty(&batch_job)?;

    Ok(json_result)
}

fn main() {
    let matches = App::new("dbfc")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("source_path")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("destination_path")
                        .short("d")
                        .long("destination")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rule")
                .arg(
                    Arg::with_name("file_extension")
                        .short("e")
                        .long("extension")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("command")
                        .short("c")
                        .long("command")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(mathes_init) = matches.subcommand_matches("init") {
        let source_path = mathes_init.value_of("source_path").unwrap();
        let destination_path = mathes_init.value_of("destination_path").unwrap();

        let mut bj = self::batch_job::BatchJob::new(
            String::from(source_path),
            String::from(destination_path),
        );

        if let Err(e) = bj.run() {
            println!("Error occured while running batch job:\n{}", e);

            return;
        }

        println!("Writing file out");

        match write_to_json_batch_job(&bj) {
            Ok(result) => {
                let config_filepath = format!("{}/dbfc.config", source_path.trim());
                println!("Config filepath: {}", config_filepath);
                let mut file = File::create(config_filepath).expect("Unable to create config file");
                file.write_all(result.as_bytes())
                    .expect("Unable to write to config file");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
