
#[derive(Copy, Clone)]
pub struct Timer {
    duration_millis: u32,
    is_running: bool,
    remaining_millis: u32,
    tick_interval_millis: u32,
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
        }
    }

    // Add `duration` to the remaining clock time.
    // The return value indicates the remaining time.
    pub fn increment(&mut self, duration_millis: u32) -> u32 {
        self.remaining_millis = self.remaining_millis + duration_millis;
        self.remaining_millis
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_millis == 0
    }

    // Pause the timer.
    // This has no effect if the timer is not running (paused or elapsed).
    // The return value indicates the remaining time.
    pub fn halt(&mut self) -> u32 {
        self.is_running = false;
        self.remaining_millis
    }

    // Return the remaining time.
    pub fn remaining(&self) -> u32 {
        self.remaining_millis
    }

    // Stop the clock and reset the remaining time to the initial duration.
    // The return value indicates the remaining time.
    pub fn reset(&mut self) -> u32 {
        self.halt();
        self.remaining_millis = self.duration_millis;
        self.remaining_millis
    }

    // Begin running down the clock.
    // This has no effect if the clock is already running or is elapsed.
    // The return value indicates the remaining time.
    pub fn run(&mut self) -> u32 {
        if !self.is_expired() {
            self.is_running = true;
        }
        self.remaining_millis
    }

    // Tick the clock.
    pub fn tick(&mut self) -> u32  {
        if self.is_expired() {
            self.is_running = false;
        } else if self.is_running {
            self.remaining_millis = self.remaining_millis - self.tick_interval_millis;
        }
        self.remaining_millis
    }
}
