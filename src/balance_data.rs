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

    pub fn current_slack(&self) -> u64 {
        self.curr_slack
    }

    pub fn add_slack(&mut self, value: u64) {
        self.slack_duration += value;
        self.curr_slack += value;
    }

    pub fn add_work(&mut self, value: u64) {
        self.work_duration += value; 
        self.curr_slack = if value >= self.curr_slack { 0 } else { self.curr_slack - value };
    }

    pub fn report(&self) -> String {
        format!("Working: {}, Slacking: {}, Current slack balance: {}", self.work_duration, self.slack_duration, self.curr_slack)
    }
}