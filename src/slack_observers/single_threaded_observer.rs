use std::thread;
use crate::win32utils::{current_process_name, display_window};
use crate::slack_observers::{BalanceData, WORKING_PROCESS};

const SLEEP_SEC: u64 = 5;
const MAX_SLACK: u64 = 180;
const WAY_TOO_SLACKY: u64 = 240;

fn slacking_loop(data: &mut BalanceData) {
    display_window("THAT'S OVER TOY BOY", "DON'T FUCK WITH ME DOG");
    const SLEEP_SLACK_SEC: u64 = 1;
    loop {
        match current_process_name() {
            Some(name) if name.as_str() == WORKING_PROCESS => { 
                data.curr_slack -= SLEEP_SLACK_SEC; data.work_duration += SLEEP_SLACK_SEC; 
            },
            Some(slack) => { 
                data.curr_slack += 1; data.slack_duration += 1; 
                display_window("TIRED? BACK TO WORK", format!("STOP IT, GET SOME HEALTH OUTSIDE OF {}", slack).as_str());
            },
            None => ()
        }
        thread::sleep(std::time::Duration::from_secs(SLEEP_SLACK_SEC));
        if data.curr_slack <= 100 {
            break;
        }
    }
}

pub fn single_threaded_observer() {
    let mut data = BalanceData::new();
    loop {
        match current_process_name() {
            Some(name) if name.as_str() == WORKING_PROCESS => { data.work_duration += SLEEP_SEC; data.curr_slack = if data.curr_slack == 0 { 0 } else { data.curr_slack - 5 } },
            Some(slack) => { 
                data.slack_duration += SLEEP_SEC; 
                data.curr_slack += SLEEP_SEC; println!("Slacking process {}", slack);
                if data.curr_slack >= WAY_TOO_SLACKY {
                    slacking_loop(&mut data)
                } else if data.curr_slack >= MAX_SLACK {
                    display_window("Too much slacking!", "Stop that goddamn slacking!");
                }
            },
            None => ()
        }
        println!("Work: {}, Slack: {}", data.work_duration, data.slack_duration);
        thread::sleep(std::time::Duration::from_secs(SLEEP_SEC));
    }
}