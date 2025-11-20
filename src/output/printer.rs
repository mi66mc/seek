use crate::colors::painter::{paint_blue, paint_red};

pub fn print_info(message: &str) {
    println!("{} {}", paint_blue("[SIK INFO]:"), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", paint_red("[SIK ERROR]:"), message);
}
