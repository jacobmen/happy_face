use std::io;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use std::thread::{self, JoinHandle};

use std::error::Error;

type MtError = Box<dyn Error + Send + Sync>;

pub struct InputCollector {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

// Heavily relied on https://github.com/lemunozm/termchat/blob/master/src/terminal_events.rs
// for code and figuring out multithreading
impl InputCollector {
    pub fn new<C>(callback: C) -> Result<InputCollector, MtError>
    where
        C: Fn(Result<String, MtError>) + Send + 'static,
    {
        let running = Arc::new(AtomicBool::new(true));

        let handle = {
            let thread_running = running.clone();

            thread::Builder::new()
                .name("Input collector".into())
                .spawn(move || {
                    let read_input = || -> Result<(), MtError> {
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        callback(Ok(input));
                        Ok(())
                    };

                    while thread_running.load(Ordering::Relaxed) {
                        if let Err(e) = read_input() {
                            callback(Err(e));
                        }
                    }
                })
        }?;

        Ok(InputCollector {
            running,
            handle: Some(handle),
        })
    }
}

impl Drop for InputCollector {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.handle
            .take()
            .unwrap()
            .join()
            .expect("Couldn't join thread");
    }
}
