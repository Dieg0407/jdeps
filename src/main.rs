use std::thread;
use std::time::Duration;

use jdeps::debouncer::Debouncer;
use jdeps::search::Dependency;
use jdeps::search::SearchEngine;
use jdeps::search::SearchCommand::UpdatedInput;
use jdeps::search::SearchCommand::Exit;
use jdeps::search::SearchCommand::Up;
use jdeps::search::SearchCommand::Down;
use jdeps::search::SearchCommand::DependenciesUpdated;
use termion::{input::TermRead, raw::IntoRawMode};
use std::io::{stdout, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode()?;

    let (repo_producer, repo_consumer) = std::sync::mpsc::channel();

    let (render_producer, render_consumer) = std::sync::mpsc::channel();
    let render_producer_for_repo = render_producer.clone();
    let mut search_engine = SearchEngine::new(stdout);

    let debouncer = Debouncer::new();
    debouncer.start(repo_producer);

    let _ = thread::spawn(move || {
        let mut input_buffer = vec![];
        let stdin = stdin();
        for key in stdin.keys() {
            let key = key.unwrap();
            match key {
                termion::event::Key::Ctrl('c') => {
                    let _ = render_producer.send(Exit);
                    debouncer.stop();
                    break;
                }
                termion::event::Key::Char(c) => { 
                    input_buffer.push(c as u8);
                    let input = String::from_utf8(input_buffer.clone()).unwrap();

                    debouncer.debounce(input.clone(), Duration::from_millis(500));
                    let _ = render_producer.send(UpdatedInput { value: input });
                },
                termion::event::Key::Backspace => {
                    input_buffer.pop();
                    let input = String::from_utf8(input_buffer.clone()).unwrap();

                    debouncer.debounce(input.clone(), Duration::from_millis(500));
                    let _ = render_producer.send(UpdatedInput { value: input });
                },
                termion::event::Key::Up => render_producer.send(Up).unwrap(),
                termion::event::Key::Down => render_producer.send(Down).unwrap(),
                _ => {}
            }
        }
    });


    thread::spawn(move || {
        for query in repo_consumer {
            // hit the api
            let dependencies = (0..query.len()).map(|i| {
                Dependency {
                    artifact_id: format!("artifact-{}", i),
                    group_id: format!("group-{}", i),
                    version: format!("version-{}", i)
                }
            }).collect();

            // send the response to the search engine
            let _ = render_producer_for_repo.send(DependenciesUpdated { dependencies });
        }
    });

    search_engine.listen(render_consumer)
}
