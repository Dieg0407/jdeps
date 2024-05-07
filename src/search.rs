use termion::color;
use termion::terminal_size;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};

#[derive(Debug)]
struct Dependency {
    artifact_id: &'static str,
    group_id: &'static str,
    version: &'static str 
}

const DEPENDENCIES: [Dependency; 2] = [
    Dependency {
        artifact_id: "junit",
        group_id: "junit",
        version: "4.12"
    },
    Dependency {
        artifact_id: "hamcrest-core",
        group_id: "org.hamcrest",
        version: "1.3"
    }
];

pub fn render()  -> Result<(), Box<dyn std::error::Error>> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut bytes = stdin.bytes();

    loop {
        write!(stdout, "{}", termion::clear::All)?;
        write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
        write!(stdout, "{}Dependencies", termion::style::Bold)?;
        write!(stdout, "{}", termion::style::Reset)?;
        write!(stdout, "\n\r")?;
        for (i, dep) in DEPENDENCIES.iter().enumerate() {
            write!(stdout, "{}. {}:{}:{}", i + 1, dep.group_id, dep.artifact_id, dep.version).unwrap();
            write!(stdout, "\n\r")?;
        }
        write!(stdout, "{}", termion::cursor::Goto(1, 10))?;
        write!(stdout, "Press 'q' to quit")?;
        stdout.flush().unwrap();
        let b = bytes.next().unwrap().unwrap();
        match b {
            b'q' => return Ok(()),
            _ => {}
        }
    }
}
