use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;


pub struct Debouncer {
    data: Arc<Mutex<Option<(String, Instant)>>>,
    stopper: Arc<Mutex<bool>>
}

impl Debouncer {
    pub fn new() -> Debouncer {
        let data = Arc::new(Mutex::new(Option::<(String,Instant)>::None));
        let stopper = Arc::new(Mutex::new(false));
        Debouncer { 
            data,
            stopper
        }
    }

    pub fn start(&self, sender: Sender<String>) -> JoinHandle<()> {
        let data_ref = Arc::clone(&self.data);
        let stopper_ref = Arc::clone(&self.stopper);
        thread::spawn(move || {
            let data = data_ref;
            let stopper = stopper_ref;
            let mut wait = Duration::from_millis(0);
            loop {
                thread::sleep(wait);
                let mut data = data.lock().unwrap();
                let stopper = stopper.lock().unwrap();
                if *stopper && (*data).is_none() {
                    break;
                }
                if let Some((string, instant)) = data.clone() {
                    if Instant::now() < instant {
                        wait = instant.duration_since(Instant::now());
                        continue;
                    }
                    sender.send(string).unwrap();
                    *data = Option::None;
                }
            }
        })
    }

    pub fn debounce(&self, string: String, duration: Duration) {
        let now = Instant::now();
        let time_in_the_future = now.checked_add(duration).unwrap();
        let mut data = self.data.lock().unwrap();
        *data = Option::Some((string, time_in_the_future));
    }

    pub fn stop(&self) {
        *self.stopper.lock().unwrap() = true;
    }
}


#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use super::Debouncer;

    #[test]
    fn should_debounce() {
        let (sender, receiver) = channel();

        let debouncer = Debouncer::new();
        let thread = debouncer.start(sender);

        for i in 0..1000 {
            debouncer.debounce(i.to_string(), std::time::Duration::from_millis(800));
        }
        debouncer.stop();

        let mut events = Vec::<String>::new();
        for event in receiver {
            events.push(event);
        }
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "999");

        let _ = thread.join();
    }
}
