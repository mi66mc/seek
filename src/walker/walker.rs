use std::{fs, path::PathBuf, sync::mpsc};

pub fn walk(path: &str, tx: &mpsc::Sender<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let e_path = entry.path();

                if e_path.is_dir() {
                    walk(e_path.to_str().unwrap(), tx);
                } else {
                    tx.send(e_path).unwrap();
                }
            }
        }
    }
}
