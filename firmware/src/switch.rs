pub struct ToggleSwitch {
    awaiting_inactivity: bool,
    change_confirmations: u16,
    required_confirmation_reports: u16,
}

impl ToggleSwitch {
    // create a new inactive switch state
    pub const fn new(required_confirmation_reports: u16) -> ToggleSwitch {
        ToggleSwitch {
            awaiting_inactivity: true,
            change_confirmations: 0,
            required_confirmation_reports,
        }
    }

    pub fn report<F>(&mut self, active: bool, activation_listener: F)
    where
        F: FnOnce()    
    {
        if self.awaiting_inactivity == active {
            // no progress toward state change
            if self.change_confirmations != 0 {
                self.change_confirmations = 0;
            }
        } else if self.change_confirmations == self.required_confirmation_reports {
            // state change triggered
            self.change_confirmations = 0;
            self.awaiting_inactivity = !self.awaiting_inactivity;
            if active {
                activation_listener();
            }
        } else {
            // progressing toward state change
            self.change_confirmations += 1;
        }
    }
}

pub struct CycleSwitch {
    active_duration: u32,
    rising_switch: ToggleSwitch,
    falling_confirmations: u16,
    required_confirmation_reports: u16,
}

impl CycleSwitch {
    pub const fn new(required_confirmation_reports: u16) -> CycleSwitch {
        CycleSwitch {
            active_duration: 0,
            rising_switch: ToggleSwitch::new(required_confirmation_reports),
            falling_confirmations: 0,
            required_confirmation_reports,
        }
    }

    pub fn report<F>(&mut self, active: bool, activation_listener: F)
    where
        F: FnOnce(u32)  
    {
        if self.active_duration == 0 {
            self.rising_switch.report(active, || {
                self.active_duration = 1;
            })
        } else {
            self.active_duration += 1;

            if active {
                self.falling_confirmations = 0;
            } else if self.falling_confirmations == self.required_confirmation_reports {
                activation_listener(self.active_duration);
                self.active_duration = 0;
                self.falling_confirmations = 0;
                self.rising_switch = ToggleSwitch::new(self.required_confirmation_reports);
            } else {
                self.falling_confirmations += 1;
            }
        }
    }
}
