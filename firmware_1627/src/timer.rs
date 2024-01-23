#[derive(Copy, Clone, PartialEq)]
pub struct Timer {
    pub duration: u32,
    remaining: u32,
}

// Each timer manages the time of (one player's) chess clock. `tick` must be called
// externally every second while the timer is "running" (counting down).
impl Timer {
    // Create a stopped clock with `duration` seconds remaining.
    pub const fn new(duration: u32) -> Timer {
        Timer {
            duration,
            remaining: duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.remaining == 0
    }

    // Return the remaining time.
    pub fn remaining(&self) -> u32 {
        self.remaining
    }

    // Reset the remaining time to the initial duration.
    pub fn reset(&mut self) {
        self.remaining = self.duration;
    }

    // Add `duration` to the remaining clock time.
    pub fn increment(&mut self, duration: u32) {
        self.remaining = self.remaining + duration;
    }

    // Tick the clock, removing a second from the remaining time
    pub fn tick(&mut self)  {
        if self.remaining > 0 {
            self.remaining -= 1;
        }
    }
}
