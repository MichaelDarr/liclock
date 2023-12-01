pub const REQUIRED_CONFIRMATION_REPORTS: u8 = 49;
pub const LONGPRESS_CONFIRMATION_REPORTS: u8 = 255;

#[derive(Copy, Clone)]
pub struct Timer {
    duration_millis: u32,
    is_running: bool,
    remaining_millis: u32,
    tick_interval_millis: u32,

    awaiting_inactivity: bool,
    change_confirmations: u8,
}

// Each timer manages the time of (one player's) chess clock. `tick` must be called
// externally every millisecond regardless of whether the timer is "running" (counting down).
impl Timer {
    // Create a stopped clock with `duration` remaining.
    // If the provided value cannot be represented as a u32 millisecond count,
    // the remaining time on the returned timer is sqashed to zero.
    pub const fn new(duration_millis: u32, tick_interval_millis: u32) -> Timer {
        Timer {
            duration_millis,
            remaining_millis: duration_millis,
            is_running: false,
            tick_interval_millis,

            awaiting_inactivity: true,
            change_confirmations: 0,
        }
    }

    // Add `duration` to the remaining clock time.
    pub fn increment(&mut self, duration_millis: u32) {
        self.remaining_millis = self.remaining_millis + duration_millis;
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_millis == 0
    }

    // Pause the timer.
    // This has no effect if the timer is not running (paused or elapsed).
    pub fn halt(&mut self) {
        self.is_running = false;
    }

    // Return the remaining time.
    pub fn remaining(&self) -> u32 {
        self.remaining_millis
    }

    // Returns true if the report activated the toggle
    pub fn report(&mut self, active: bool) -> bool {
        // no progress toward state change
        if self.awaiting_inactivity == active {
            if self.change_confirmations != 0 {
                self.change_confirmations = 0;
            }
            return false;
        }

        // state change triggered
        if self.change_confirmations == REQUIRED_CONFIRMATION_REPORTS {
            self.change_confirmations = 0;
            self.awaiting_inactivity = !self.awaiting_inactivity;
            return active;
        }

        // progressing toward state change
        self.change_confirmations += 1;
        return false;
    }

    // Stop the clock and reset the remaining time to the initial duration.
    pub fn reset(&mut self) {
        self.halt();
        self.remaining_millis = self.duration_millis;
    }

    // Begin running down the clock.
    // This has no effect if the clock is already running or is elapsed.
    pub fn run(&mut self) {
        if !self.is_expired() {
            self.is_running = true;
        }
    }

    // Tick the clock.
    // Returns true if this tick resulted in a digit change. 
    pub fn tick(&mut self) -> bool  {
        if !self.is_running {
            return false;
        }

        if self.is_expired() {
            self.is_running = false;
            return true;
        }

        let will_change = (self.remaining_millis % 1000) < self.tick_interval_millis;
        self.remaining_millis = self.remaining_millis - self.tick_interval_millis;
        return will_change;
    }
}
