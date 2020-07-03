use std::thread::{JoinHandle, self};
use std::sync::mpsc::{channel, Sender};
use rayon;

use dummy_discord::Discord;

mod dummy_discord;

pub enum Command {}

struct WorkerPool {
    receiver: Option<JoinHandle<()>>,
    sender: Option<Sender<Command>>
}

impl WorkerPool {
    fn new() -> Self {
        let (tx, rx) = channel();

        let receiver_thread = thread::spawn(move || {		
            let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

            while let Ok(_command) = rx.recv() {
                /* do matching logic */
                // pool.spawn(|| /* do_command(command) */);
            }
            eprintln!("Worker loop done.");
        });

        Self {
            receiver: Some(receiver_thread),
            sender: Some(tx)
        }
    }

    fn tx(&self) -> Sender<Command> {
        // SAFETY
        // safe because option is set to None only when dropped.
        self.sender.clone().unwrap()
    }
}

/* Ensure receiver thread is joined when WorkerPool goes out of scope. */
impl Drop for WorkerPool {
    fn drop(&mut self) {
      self.sender.take();
      self.receiver.take().and_then(|handle| handle.join().ok());
      eprintln!("dropped worker pool receiver thread");
    }
}

fn main() {
    let worker_pool = WorkerPool::new();
    let discord = Discord::new(worker_pool.tx());
    let dref = discord.clone();
    ctrlc::set_handler(move || dref.shutdown());

    discord.start();
    eprintln!("Good bye!");
}
