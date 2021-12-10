use crate::win32utils;
use crate::timer;
use crate::balance_data::BalanceData;
use win32utils::current_process_name;
use std::{collections::HashSet, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex} };

const HEAVY_SLEEP_SEC: u64 = 1;
const SLEEP_SEC: u64 = 5;
const MAX_SLACK: u64 = 180;
const WAY_TOO_SLACKY: u64 = 240;

enum Record {
    Slacking(String),
    Working,
    Undefined
}

pub struct Punisher<F1: Fn(), F2: Fn()> {
    callback_light: F1,
    callback_heavy: F2,
    data: Arc<Mutex<BalanceData>>,
    ignored_processes: HashSet<String>,
    work_processes: Arc<Mutex<HashSet<String>>>,
    rolling: Arc<AtomicBool>
}

fn create_ignored_processes() -> HashSet<String> {
    let mut to_ret = HashSet::new();
    to_ret.insert(String::from("cmd.exe"));
    to_ret.insert(String::from("mieszkania-olx.exe"));
    to_ret
}

impl<F1: Fn(), F2: Fn()> Punisher<F1, F2> {
    pub fn new(
        light_slack: F1, 
        heavy_slack: F2, 
        rolling_token: &Arc<AtomicBool>, 
        balance: &Arc<Mutex<BalanceData>>,
        work_processes: &Arc<Mutex<HashSet<String>>>) -> Punisher<F1, F2> {
        Punisher {
            callback_light: light_slack,
            callback_heavy: heavy_slack,
            data: balance.clone(),
            ignored_processes: create_ignored_processes(),
            work_processes: work_processes.clone(),
            rolling: rolling_token.clone()
        }
    }
    
    fn user_working_data_record(&mut self, value: u64) {
        self.data.lock().unwrap().add_work(value);
    }
    
    fn user_slacking(&mut self, slack_process: &str) {
        let curr_slack: u64;
        {
            let mut data = self.data.lock().unwrap();
            data.add_slack(SLEEP_SEC);
            println!("Slacking process {}", slack_process);
            curr_slack = data.current_slack();
        }
        if curr_slack >= WAY_TOO_SLACKY {
            timer::block_run_every(std::time::Duration::from_secs(HEAVY_SLEEP_SEC), || self.heavy_slack());
        } else if curr_slack >= MAX_SLACK {
            (self.callback_light)();
        }
    }
    
    fn is_user_slacking(&self) -> Record {
        match current_process_name() {
            Some(ignored) if self.ignored_processes.contains(&ignored) => Record::Undefined,
            Some(work) if self.work_processes.lock().unwrap().contains(&work) => Record::Working,
            Some(slack) => Record::Slacking(slack),
            _ => Record::Undefined
        }
    }
    
    fn heavy_slack(&mut self) -> bool {
        if self.rolling.load(Ordering::SeqCst) {
            (self.callback_heavy)();
            match self.is_user_slacking() {
                Record::Slacking(p_name) => {
                    self.data.lock().unwrap().add_slack(HEAVY_SLEEP_SEC);
                    println!("Slacking process {}", p_name);
                    true
                }
                Record::Working => {
                    let mut data = self.data.lock().unwrap();
                    data.add_work(HEAVY_SLEEP_SEC);
                    return data.current_slack() >= WAY_TOO_SLACKY / 2;
                }
                Record::Undefined => true
            }
        } else {
            true
        }
    }
    
    pub fn run(&mut self) {
        timer::block_run_every(std::time::Duration::from_secs(SLEEP_SEC), || {
            if self.rolling.load(Ordering::SeqCst) {
                match self.is_user_slacking() {
                    Record::Slacking(slack_process) => self.user_slacking(&slack_process),
                    Record::Working => self.user_working_data_record(SLEEP_SEC),
                    Record::Undefined => {}
                }
            } 
            true
        })
    }
}