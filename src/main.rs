use seek::cli::args::Args;
use seek::walker::walk;
use seek::worker::process_file;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let path = args.path;
    let (tx, rx) = mpsc::channel();
    let mut workers = Vec::new();

    let walker = thread::spawn(move || {
        walk(&path, &tx);
        drop(tx);
    });

    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..args.threads {
        let rx = Arc::clone(&rx);
        workers.push(thread::spawn(move || {
            process_file(rx);
        }));
    }

    walker.join().unwrap();

    for w in workers {
        w.join().unwrap();
    }

    Ok(())
}
