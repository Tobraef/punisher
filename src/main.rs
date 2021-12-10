mod win32utils;
mod timer;
mod block_service;
mod slack_observers;
mod punisher;
mod balance_data;

fn main() {
    slack_observers::multi_threaded_observer::multi_threaded_observer();
}
