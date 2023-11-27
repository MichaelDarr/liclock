
#[derive(Copy, Clone)]
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

    // Returns true if the report activated the toggle
    pub fn report_alt<F>(self, active: bool, activation_listener: F) -> ToggleSwitch
    where
        F: FnOnce()
    {
        // no progress toward state change
        if self.awaiting_inactivity == active {
            return ToggleSwitch {
                awaiting_inactivity: self.awaiting_inactivity,
                change_confirmations: 0,
                required_confirmation_reports: self.required_confirmation_reports,
            }
        }

        // state change triggered
        if self.change_confirmations == self.required_confirmation_reports {
            if active {
                activation_listener();
            }
            return ToggleSwitch {
                awaiting_inactivity: !self.awaiting_inactivity,
                change_confirmations: 0,
                required_confirmation_reports: self.required_confirmation_reports,
            }
        }

        // progressing toward state change
        return ToggleSwitch {
            awaiting_inactivity: !self.awaiting_inactivity,
            change_confirmations: self.change_confirmations + 1,
            required_confirmation_reports: self.required_confirmation_reports,
        }
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
        if self.change_confirmations == self.required_confirmation_reports {
            self.change_confirmations = 0;
            self.awaiting_inactivity = !self.awaiting_inactivity;
            return active;
        }

        // progressing toward state change
        self.change_confirmations += 1;
        return false;
    }
}

// pub struct CycleSwitch {
//     active_duration: u32,
//     rising_switch: ToggleSwitch,
//     falling_confirmations: u16,
//     required_confirmation_reports: u16,
// }

// impl CycleSwitch {
//     pub const fn new(required_confirmation_reports: u16) -> Self {
//         CycleSwitch {
//             active_duration: 0,
//             rising_switch: ToggleSwitch::new(required_confirmation_reports),
//             falling_confirmations: 0,
//             required_confirmation_reports,
//         }
//     }

//     pub fn report(&mut self, active: bool) -> bool {
//         if self.active_duration == 0 {
//             if self.rising_switch.report(active) {
//                 self.active_duration = 1;
//             }
//             return false;
//         }

//         self.active_duration += 1;

//         if active {
//             self.falling_confirmations = 0;
//             return false;
//         }
        
//         if self.falling_confirmations == self.required_confirmation_reports {
//             self.active_duration = 0;
//             self.falling_confirmations = 0;
//             self.rising_switch = ToggleSwitch::new(self.required_confirmation_reports);
//             return true;
//         }


//         self.falling_confirmations += 1;
//         return false;
//     }
// }
