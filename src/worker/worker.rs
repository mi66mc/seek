use std::{
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Receiver},
};

pub fn process_file(rx: Arc<Mutex<Receiver<PathBuf>>>) {
    loop {
        let msg = {
            let rx = rx.lock().unwrap();
            rx.recv()
        };

        match msg {
            Ok(path) => {
                println!("worker: {}", path.display());
            }
            Err(_) => break,
        }
    }
}
