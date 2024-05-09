use std::thread;

use jdeps::search::Dependency;
use jdeps::search::SearchEngine;
use jdeps::search::SearchCommand::CharacterInputed;
use jdeps::search::SearchCommand::DependenciesUpdated;
use termion::{input::TermRead, raw::IntoRawMode};
use std::io::{stdout, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode()?;

    let (producer, consumer) = std::sync::mpsc::channel();
    let mut search_engine = SearchEngine::new(stdout);

    thread::spawn(move || {
        let mut dependencies = vec![];
        let stdin = stdin();
        for key in stdin.keys() {
            let key = key.unwrap();
            match key {
                termion::event::Key::Char('a') => {
                    producer.send(CharacterInputed { character: 'a' as u8 }).unwrap();
                    dependencies.push(Dependency { artifact_id: "new".to_string(), group_id: "new".to_string(), version: "new".to_string() });
                    producer.send(DependenciesUpdated { dependencies: dependencies.clone() }).unwrap();
                },
                termion::event::Key::Char(c) => {
                    producer.send(CharacterInputed { character: c as u8 }).unwrap();
                },
                termion::event::Key::Ctrl('c') => {
                    break;
                }
                _ => {}
            }
        }
    });

    search_engine.listen(consumer)
}
