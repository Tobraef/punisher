use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::win32utils;

pub struct BlockService {
    blocked_processes: Arc<Mutex<HashSet<String>>>
}

impl BlockService {
    pub fn new() -> BlockService {
        BlockService {
            blocked_processes: Arc::new(Mutex::new(HashSet::new()))
        }
    }

    fn schedule_block(&self, process_name: &str, block_text: &str) {
        let process_name = String::from(process_name);
        let block_text = String::from(block_text);
        let processes_arc = self.blocked_processes.clone();
        std::thread::spawn(move || {
            {
                let mut guard = processes_arc.lock().unwrap();
                guard.insert(String::from(&process_name));
            }
            win32utils::display_window("title", &block_text);
            println!("BLOCKED {}", process_name);
            {
                let mut guard = processes_arc.lock().unwrap();
                guard.remove(&process_name);
            }
        });
    }

    pub fn block_process(&self, process_name: &str, block_text: &str) {
        if !self.blocked_processes.lock().unwrap().contains(process_name) {
            self.schedule_block(process_name, block_text);
        }
    }
}