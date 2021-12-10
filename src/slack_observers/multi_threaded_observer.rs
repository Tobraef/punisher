use crate::block_service;
use crate::punisher;
use crate::win32utils;
use crate::balance_data;

use std::{
    thread, 
    sync::{
        Mutex, Arc, 
        atomic::{
            AtomicBool, Ordering
        } 
    }, 
    collections::HashSet,
    iter:: {
        FromIterator, once
    } 
};

fn instructions() -> &'static str {
"    info -> current balance information
    break -> stops the punisher
    resume -> resumes the punisher
    add [name] -> adds a process to work processes
    remove [name] -> removes a process from work processes
    exit -> exits the program"
}

fn handle_process_add(work_processes: &mut Arc<Mutex<HashSet<String>>>, add_line: &str) {
    let split_iter = add_line.split_ascii_whitespace();
    if let Some(process_name) = split_iter.skip(1).next() {
        if win32utils::does_process_exist(&process_name) {
            let mut guard = work_processes.lock().unwrap();
            guard.insert(String::from(process_name));
            print!("Added \"{}\" to list of work processes, currently treating as work:\n{}", 
                process_name, 
                guard.iter().fold(String::new(), |mut a, b| { a.push_str(b); a.push_str("\n"); a }));
        } else {
            println!("Didn't find active process named {}", process_name);
        }
    } else {
        println!("Didn't find anything behind a space after add");
    }
}

fn handle_process_remove(work_processes: &mut Arc<Mutex<HashSet<String>>>, remove_line: &str) {
    let split_iter = remove_line.split_ascii_whitespace();
    if let Some(process_name) = split_iter.skip(1).next() {
        let mut guard = work_processes.lock().unwrap();
        if guard.remove(process_name) {
            print!("Removed \"{}\" from list of work processes, currently treating as work:\n{}", 
                process_name, 
                guard.iter().fold(String::new(), |mut a, b| { a.push_str(b); a.push_str("\n"); a }));
        } else {
            print!("Didn't find \"{}\" among working processes, currently treating as work:\n{}",
                process_name,
                work_processes.lock().unwrap().iter().fold(String::new(), |mut a, b| { a.push_str(b); a.push_str("\n"); a }));
        }
    } else {
        println!("Didn't find anything behind a space after remove");
    }
}

pub fn multi_threaded_observer() {
    let service = Arc::new(block_service::BlockService::new());
    let service_arc_1 = service.clone();
    let service_arc_2 = service.clone();
    let sync_token = Arc::new(AtomicBool::new(true));
    let mut work_processes: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(
        HashSet::from_iter(once(String::from("mstsc.exe")))));
    let balance = Arc::new(Mutex::new(balance_data::BalanceData::new()));
    let mut punisher = punisher::Punisher::new(
        move || if let Some(n) = win32utils::current_process_name() { service_arc_1.block_process(&n, "Stop pls") },
        move || if let Some(n) = win32utils::current_process_name() { service_arc_2.block_process(&n, "STOP PLS") },
        &sync_token,
        &balance,
        &work_processes);
    thread::spawn(move || punisher.run());
    loop {
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.as_str().trim_end() {
                "info" => println!("{}", balance.lock().unwrap().report()),
                "break" => { sync_token.store(false, Ordering::SeqCst); println!("Break time!"); },
                "resume" => { sync_token.store(true, Ordering::SeqCst); println!("Resumed"); },
                add if add.starts_with("add") => handle_process_add(&mut work_processes, add),
                remove if remove.starts_with("remove") => handle_process_remove(&mut work_processes, remove),
                "exit" => break,
                _ => println!("Available options\n{}", instructions())
            }
            Err(e) => println!("Error {}", e)
        }
    }
}