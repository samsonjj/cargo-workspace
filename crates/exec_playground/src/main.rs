use std::{
    io::BufRead,
    io::BufReader,
    process::{self, Stdio},
};

fn main() {
    let mut child = process::Command::new("test_bin")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = child.stdout.as_mut().unwrap();

    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
        let line = line.unwrap();
        println!("{line}");
        println!("{line}");
    }
}
