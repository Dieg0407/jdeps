mod engine;

use engine::SearchEngine;
use engine::SearchCommand::Up;
use engine::SearchCommand::Down;
use engine::SearchCommand::Exit;
use engine::SearchCommand::UpdatedInput;
use engine::SearchCommand::DependenciesUpdated;

use std::error::Error;
use std::io::stdin;
use std::io::stdout;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

use crate::debouncer::Debouncer;
use crate::models::Dependency;

use self::engine::SearchCommand;

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode()?;

    let (engine_event_sender, engine_event_receiver) = std::sync::mpsc::channel();
    let mut search_engine = SearchEngine::new(stdout);

    let (fetch_deps_sender, fetch_deps_receiver) = std::sync::mpsc::channel();
    let debouncer = Debouncer::new();
    debouncer.start(fetch_deps_sender);

    let _ = start_fetch_deps_listener(engine_event_sender.clone(), fetch_deps_receiver);
    let _ = start_input_listener(engine_event_sender, debouncer);

    search_engine.listen(engine_event_receiver)
}

fn start_input_listener(engine_sender: Sender<SearchCommand>, debouncer: Debouncer) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut input_buffer = vec![];
        let stdin =stdin();
        for key in stdin.keys() {
            let key = key.unwrap();
            match key {
                termion::event::Key::Ctrl('c') => {
                    let _ = engine_sender.send(Exit);
                    debouncer.stop();
                    break;
                }
                termion::event::Key::Char(c) => { 
                    input_buffer.push(c as u8);
                    let input = String::from_utf8(input_buffer.clone()).unwrap();

                    debouncer.debounce(input.clone(), Duration::from_millis(500));
                    let _ = engine_sender.send(UpdatedInput { value: input });
                },
                termion::event::Key::Backspace => {
                    input_buffer.pop();
                    let input = String::from_utf8(input_buffer.clone()).unwrap();

                    debouncer.debounce(input.clone(), Duration::from_millis(500));
                    let _ = engine_sender.send(UpdatedInput { value: input });
                },
                termion::event::Key::Up => engine_sender.send(Up).unwrap(),
                termion::event::Key::Down => engine_sender.send(Down).unwrap(),
                _ => {}
            }
        }
    })
}

fn start_fetch_deps_listener(engine_sender: Sender<SearchCommand>, fetch_deps_recevier: Receiver<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        for query in fetch_deps_recevier {
            // hit the api
            let dependencies = (0..query.len()).map(|i| {
                Dependency {
                    artifact_id: format!("artifact-{}", i),
                    group_id: format!("group-{}", i),
                    version: format!("version-{}", i)
                }
            }).collect();

            // send the response to the search engine
            let _ = engine_sender.send(DependenciesUpdated { dependencies });
        }
    })
}
