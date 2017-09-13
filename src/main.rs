pub mod batch_job;

use std::io;

fn main() {
    // get the source dir
    let mut source_dir = String::new();
    io::stdin().read_line(&mut source_dir).expect("failed to get source dir");

    // get the destination dir
    let mut destination_dir = String::new();
    io::stdin().read_line(&mut destination_dir).expect("failed to get destination dir");

    let bj = self::batch_job::BatchJob::new(source_dir.clone(), destination_dir.clone());

    bj.run();
}