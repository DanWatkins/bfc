#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

pub mod batch_job;

use batch_job::BatchJob;
use clap::{App, Arg, SubCommand};
use std::env;

fn main() {
    let matches = App::new("dbfc")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("init")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the batch job")
                        .required(true),
                )
                .arg(
                    Arg::with_name("source_path")
                        .short("s")
                        .long("source")
                        .takes_value(true)
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
        .subcommand(
            SubCommand::with_name("run")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the batch job")
                        .required(true),
                )
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("directory")
                        .help(
                            "Directory containing the .dbfc directory. Defaults to the current directory.",
                        )
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches_init) = matches.subcommand_matches("init") {
        let name = matches_init.value_of("name").unwrap();
        let destination_path = matches_init.value_of("destination_path").unwrap();

        // use the current directory unless dir has been specified
        let source_path: String = match matches_init.value_of("source_path") {
            Some(v) => String::from(v),
            None => {
                let cd = env::current_dir().unwrap();
                String::from(cd.as_path().to_str().unwrap())
            }
        };

        let mut bj = BatchJob::new(name, &source_path, destination_path);

        if let Err(e) = bj.init() {
            println!("Error occured while running batch job:\n{}", e);
            return;
        }

        if let Err(why) = bj.save_to_file() {
            println!("Error while writing batch job to file: {}", why);
            return;
        }
    } else if let Some(matches_run) = matches.subcommand_matches("run") {
        if let Err(e) = run(matches_run) {
            println!("{}", e);
            return;
        }
    }
}

fn run(arg_matches: &clap::ArgMatches) -> Result<(), Box<std::error::Error>> {
    // use the current directory unless dir has been specified
    let dir: String = match arg_matches.value_of("dir") {
        Some(v) => String::from(v),
        None => {
            let cd = env::current_dir()?;
            String::from(cd.as_path().to_str().unwrap())
        }
    };

    let name = arg_matches.value_of("name").unwrap();

    let mut bj = BatchJob::load_from_file(dir.as_str(), name)?;
    bj.run();

    Ok(())
}
