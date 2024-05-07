use termion::raw::RawTerminal;
use std::io::{StdoutLock, Write};
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Dependency {
    pub artifact_id: &'static str,
    pub group_id: &'static str,
    pub version: &'static str 
}

#[derive(Debug)]
pub enum SearchCommand {
    CharacterInputed { character: u8 } ,
    DependenciesUpdated { dependencies: Vec<Dependency> }
}

pub struct SearchEngine {
    render_listener: Receiver<SearchCommand>,
    stdout: RawTerminal<StdoutLock<'static>>
}

impl SearchEngine {
    pub fn new(render_listener: Receiver<SearchCommand>, stdout: RawTerminal<StdoutLock<'static>>) -> SearchEngine {
        SearchEngine {
            render_listener,
            stdout
        }
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for message in &self.render_listener {
            write!(self.stdout, "{:?}", message)?;
            self.stdout.flush()?;
        }
        Ok(())
    }
}


/*
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
}*/
