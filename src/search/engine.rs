use crate::models::Dependency;
use termion::raw::RawTerminal;
use termion::style;
use std::io::{StdoutLock, Write};
use std::sync::mpsc::Receiver;

const ARROW: char = '\u{2192}';

#[derive(Debug)]
pub enum SearchCommand {
    Exit,
    Up,
    Down,
    UpdatedInput { value: String } ,
    DependenciesUpdated { dependencies: Vec<Dependency> }
}

pub struct SearchEngine {
    dependenices: Vec<Dependency>,
    stdout: RawTerminal<StdoutLock<'static>>,
    input: String,

    // controls
    selected_dependency_index: i32,
    skipped_rows: usize
}

impl SearchEngine {
    pub fn new(stdout: RawTerminal<StdoutLock<'static>>) -> SearchEngine {
        SearchEngine {
            dependenices: vec![],
            stdout,
            input: "".to_string(),
            selected_dependency_index: 0,
            skipped_rows: 0
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
                SearchCommand::UpdatedInput { value } => {
                    self.input = value;
                    self.render()?;
                }
                SearchCommand::Exit => {
                    self.clear();
                    break;
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
        self.clear();

        // get terminal size
        let (width, height) = termion::terminal_size()?;
        let max_visible_rows = height - 2;

        // setup selector
        if !self.dependenices.is_empty() && self.selected_dependency_index == -1 {
            self.selected_dependency_index = 0;
            self.skipped_rows = 0;
        }
        else if self.dependenices.is_empty() {
            self.selected_dependency_index = -1;
            self.skipped_rows = 0;
        }
        else if self.selected_dependency_index > self.dependenices.len() as i32 - 1 {
            self.selected_dependency_index = self.dependenices.len() as i32 - 1;
            let possible_skipped_rows = self.dependenices.len() as i32 - max_visible_rows as i32;
            self.skipped_rows = if possible_skipped_rows > 0 { possible_skipped_rows as usize } else { 0 };
        }
        
        if max_visible_rows as i32 + self.skipped_rows as i32 == self.selected_dependency_index {
            self.skipped_rows += 1;
        }
        else if self.selected_dependency_index == self.skipped_rows as i32 - 1 && self.skipped_rows > 0 {
            self.skipped_rows -= 1;
        }

        let mut counter = 0;
        for (i, dep) in self.dependenices.iter().enumerate().skip(self.skipped_rows) {
            if max_visible_rows - counter as u16 == 0 {
                break;
            }
            let selector = if i as i32 == self.selected_dependency_index { ARROW } else { ' ' };
            write!(self.stdout, "{}", termion::cursor::Goto(1, max_visible_rows - counter as u16))?;
            write!(self.stdout, "{} {}|{}:{}:{}", selector, i, dep.group_id, dep.artifact_id, dep.version).unwrap();
            write!(self.stdout, "\r")?;
            counter += 1;
        }

        // print separator
        write!(self.stdout, "{}", termion::cursor::Goto(1, height - 1))?;
        (0..width).for_each(|_| write!(self.stdout, "{}", '\u{2500}').unwrap());
        
        // print the current buffer
        write!(self.stdout, "{}>{}", style::Bold, style::Reset)?;
        write!(self.stdout, "{}{}", termion::cursor::Goto(3, height), self.input)?;
        self.stdout.flush().unwrap();
        Ok(())
    }
}

