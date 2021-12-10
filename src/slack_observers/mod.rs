pub mod single_threaded_observer;
pub mod multi_threaded_observer;

pub const WORKING_PROCESS: &str = "mstsc.exe";

pub struct BalanceData {
    work_duration: u64,
    slack_duration: u64,
    curr_slack: u64
}

impl BalanceData {
    pub fn new() -> BalanceData {
        BalanceData {
            work_duration: 0,
            slack_duration: 0,
            curr_slack: 0
        }
    }
}