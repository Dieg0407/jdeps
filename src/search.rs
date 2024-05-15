use termion::raw::RawTerminal;
use termion::style;
use std::io::{StdoutLock, Write};
use std::sync::mpsc::Receiver;

const ARROW: char = '\u{2192}';

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
    Exit,
    Up,
    Down,
    Backspace,
    CharacterInputed { character: u8 } ,
    DependenciesUpdated { dependencies: Vec<Dependency> }
}

pub struct SearchEngine {
    dependenices: Vec<Dependency>,
    stdout: RawTerminal<StdoutLock<'static>>,
    input_buffer: Vec<u8>,

    // controls
    selected_dependency_index: i32
}

impl SearchEngine {
    pub fn new(stdout: RawTerminal<StdoutLock<'static>>) -> SearchEngine {
        let mut deps = vec![];
        (0..200).for_each(|i| {
            deps.push(Dependency { artifact_id: format!("artifact-{}", i), group_id: format!("group-{}", i), version: format!("version-{}", i) });
        });
        SearchEngine {
            dependenices: deps,
            stdout,
            input_buffer: vec![],
            selected_dependency_index: 0
        }
    }

    pub fn listen(&mut self, event_listener: Receiver<SearchCommand>) -> Result<(), Box<dyn std::error::Error>> {
        self.render()?;
        for message in &event_listener {
            match message {
                SearchCommand::DependenciesUpdated { dependencies } => {
                    self.dependenices = dependencies;
                    self.render()?;
                }
                SearchCommand::CharacterInputed { character } => {
                    self.input_buffer.push(character);
                    self.render()?;
                }
                SearchCommand::Exit => {
                    self.clear();
                    break;
                }
                SearchCommand::Backspace => {
                    self.input_buffer.pop();
                    self.render()?;
                }
                SearchCommand::Down => {
                    if self.selected_dependency_index > 0 {
                        self.selected_dependency_index -= 1;
                        self.render()?;
                    }
                }
                SearchCommand::Up => {
                    if self.selected_dependency_index < self.dependenices.len() as i32 {
                        self.selected_dependency_index += 1;
                        self.render()?;
                    }
                }
            }
                    
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        let _ = write!(self.stdout, "{}", termion::clear::All);
        let _ = write!(self.stdout, "{}", termion::cursor::Goto(1, 1));
    }

    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // setup selector
        if !self.dependenices.is_empty() && self.selected_dependency_index == -1 {
            self.selected_dependency_index = 0;
        }
        else if self.dependenices.is_empty() {
            self.selected_dependency_index = -1;
        }
        else if self.selected_dependency_index > self.dependenices.len() as i32 - 1 {
            self.selected_dependency_index = self.dependenices.len() as i32 - 1;
        }

        // get terminal size
        let (width, height) = termion::terminal_size()?;
        write!(self.stdout, "{}", termion::clear::All)?;
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        write!(self.stdout, "{}Dependencies", termion::style::Bold)?;
        write!(self.stdout, "{}", termion::style::Reset)?;
        write!(self.stdout, "\n\r")?;

        let start = height - 2;
        for (i, dep) in self.dependenices.iter().enumerate() {
            if start - i as u16 == 0 {
                break;
            }
            let selector = if i as i32 == self.selected_dependency_index { ARROW } else { ' ' };
            write!(self.stdout, "{}", termion::cursor::Goto(1, start - i as u16))?;
            write!(self.stdout, "{} {}|{}:{}:{}", selector, i, dep.group_id, dep.artifact_id, dep.version).unwrap();
            write!(self.stdout, "\r")?;
        }

        // print separator
        write!(self.stdout, "{}", termion::cursor::Goto(1, height - 1))?;
        (0..width).for_each(|_| write!(self.stdout, "{}", '\u{2500}').unwrap());
        
        // print the current buffer
        let input_buffer = String::from_utf8(self.input_buffer.clone())?;
        write!(self.stdout, "{}>{}", style::Bold, style::Reset)?;
        write!(self.stdout, "{}{}", termion::cursor::Goto(3, height), input_buffer)?;
        self.stdout.flush().unwrap();
        Ok(())
    }
}

