use std::fs;
use std::io;
use std::process::Command;

fn main() {
    // get the source dir
    let mut source_dir = String::new();
    io::stdin().read_line(&mut source_dir).expect("failed to get source dir");

    // get the destination dir
    let mut destination_dir = String::new();
    io::stdin().read_line(&mut destination_dir).expect("failed to get destination dir");

    // run conversion
    let paths = fs::read_dir(source_dir.trim()).unwrap();

    for path in paths {
        let x = path.unwrap().path();
        println!("{}", x.display());

        let mut output = Command::new("ffmpeg")
            .arg("-i")
            .arg(x.as_os_str())
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
            .arg(format!("{}{:?}.mp4", destination_dir.trim(), x.file_stem().unwrap()))
            .spawn()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

        output.wait().unwrap();
    }
}