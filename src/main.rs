use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let interests: Vec<_> = std::env::args().skip(1).collect();

    let mut cmd = Command::new("cargo")
        .arg("clippy")
        .arg("--color=always")
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    {
        // proccess output
        let stderr = cmd.stderr.as_mut().unwrap();
        let br = BufReader::new(stderr);
        let mut buf = vec![];
        let mut next_line_is_file = false;
        let mut skip = false;
        for line in br.lines() {
            let line = line.unwrap();
            let escaped_line_raw = strip_ansi_escapes::strip(&line).unwrap();
            let escaped_line = String::from_utf8_lossy(&escaped_line_raw);
            if next_line_is_file {
                // parse file name
                let file = escaped_line
                    .split(' ')
                    .last()
                    .unwrap()
                    .split(':')
                    .next()
                    .unwrap();
                if !interests.iter().any(|i| i == file) {
                    skip = true
                }
                next_line_is_file = false
            }
            match escaped_line.starts_with("warning:") {
                true => {
                    skip = false;
                    next_line_is_file = true;
                    if buf.len() > 1 {
                        for output in buf.drain(..) {
                            println!("{output}");
                        }
                    }
                    buf.clear();
                    buf.push(line);
                }
                false => {
                    if !skip {
                        buf.push(line)
                    }
                }
            };
        }

        if !buf.is_empty() {
            for output in buf.drain(..) {
                println!("{output}");
            }
        }
    }

    cmd.wait().unwrap();
}
