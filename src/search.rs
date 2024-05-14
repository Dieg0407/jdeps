use termion::raw::RawTerminal;
use std::io::{StdoutLock, Write};
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Dependency {
    pub artifact_id: String,
    pub group_id: String,
    pub version: String
}

impl Clone for Dependency {
    fn clone(&self) -> Self {
        Dependency {
            artifact_id: self.artifact_id.clone(),
            group_id: self.group_id.clone(),
            version: self.version.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.artifact_id = source.artifact_id.clone();
        self.group_id = source.group_id.clone();
        self.version = source.version.clone();
    }
}
#[derive(Debug)]
pub enum SearchCommand {
    CharacterInputed { character: u8 } ,
    DependenciesUpdated { dependencies: Vec<Dependency> }
}

pub struct SearchEngine {
    dependenices: Vec<Dependency>,
    stdout: RawTerminal<StdoutLock<'static>>,
    input_buffer: Vec<u8>
}

impl SearchEngine {
    pub fn new(stdout: RawTerminal<StdoutLock<'static>>) -> SearchEngine {
        SearchEngine {
            stdout,
            dependenices: vec![],
            input_buffer: vec![],
        }
    }

    pub fn listen(&mut self, event_listener: Receiver<SearchCommand>) -> Result<(), Box<dyn std::error::Error>> {
        self.render()?;
        for message in &event_listener {
            match message {
                SearchCommand::DependenciesUpdated { dependencies } => {
                    self.dependenices = dependencies;
                    self.render()?;
                },
                SearchCommand::CharacterInputed { character } => {
                    self.input_buffer.push(character);
                    self.render()?;
                }
            }
                    
        }
        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get terminal size
        let (width, height) = termion::terminal_size()?;
        write!(self.stdout, "{}", termion::clear::All)?;
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        write!(self.stdout, "{}Dependencies", termion::style::Bold)?;
        write!(self.stdout, "{}", termion::style::Reset)?;
        write!(self.stdout, "\n\r")?;
        for (i, dep) in self.dependenices.iter().enumerate() {
            write!(self.stdout, "{} {}. {}:{}:{}",'\u{2192}', i + 1, dep.group_id, dep.artifact_id, dep.version).unwrap();
            write!(self.stdout, "\n\r")?;
        }

        // print separator
        write!(self.stdout, "{}", termion::cursor::Goto(1, height - 1))?;
        (0..width).for_each(|_| write!(self.stdout, "{}", '\u{2500}').unwrap());
        
        // print the current buffer
        let input_buffer = String::from_utf8(self.input_buffer.clone()).unwrap();
        write!(self.stdout, "> {}{}", termion::cursor::Goto(3, height), input_buffer)?;
        self.stdout.flush().unwrap();
        Ok(())
    }
}

