use std::{time::Duration, thread::sleep };

pub fn run_every<F: 'static + FnMut() + Send>(delay: Duration, f: &'static mut F) {
    std::thread::spawn(move || {
        sleep(delay);    
        loop {
            f();
            sleep(delay);
        }
    });
}

pub fn block_run_every<F>(delay: Duration, mut f: F)
    where F: FnMut() -> bool {
    sleep(delay);
    loop {
        if !f() { break; }
        sleep(delay);
    }
}