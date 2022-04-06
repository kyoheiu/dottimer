pub const WELCOME: &str = "\u{23F2} Welcome to dottimer!\u{1F680}";
pub const ENTER_DESC: &str = "First, please enter the description of the timer.";
pub const TIMER_KIND: &str = "systemd timers are difined as one of two types:\nMonotonic timers activate after a time span relative to a varying starting point.\nRealtime timers activate on a calendar event, the same way that cronjobs do.";
pub const ENTER_KIND: &str = "Which kind of timer do you want?";
pub const CHOOSE_MONOTONIC_KIND: &str = "Choose kinds of Monotonic timer(i.e. \"1\" or \"2 3 4\"):";
pub const KIND1: &str = "1) OnActiveSec: Relative to the mooment the timer unit is activated.";
pub const KIND2: &str = "2) OnBootSec: Relative to when the machines was booted up.";
pub const KIND3: &str = "3) OnStartupSec: Relative to when the service manager was first started.";
pub const KIND4: &str =
    "4) OnUnitActiveSec: Relative to when the unit is activating was last activated.";
pub const KIND5: &str =
    "5) OnUnitInactiveSec: Relative to when the unit is activating was last deactivated.";
pub const PARSE_ERROR: &str = "Parse error: Enter again. > ";
pub const ENTER_SPAN :&str = "\nEnter the time span for each timer.\nExample: \"50\" for OnBootSec means 50s after boot-up. \nThe argument may also include time units.\nAnother example: \"5h 30min\" for OnBootSec means 5 hours and 30 minutes after boot-up.\nFor details about the syntax of time spans, see systemd.time(7).";
pub const OK_YN: &str = "OK? [Y/n] ";
pub const ENTER_AGAIN: &str = "Enter again. > ";
pub const IS_INTERACTIVE: &str = "Do you want the interactive input for time spec? [y/N]";
pub const DOW :&str = "1. the Day of Week:\n[Mon, Tue, Wed, Thu, Fri, Sat, Sun]\nEnter the days you want (i.e. \"Mon, Wed\") default: None";
pub const YEAR: &str =
    "2. Year:\nEnter year (i.e. \"2022\", \"2023..2025\", or \"2024 2025 2028..2030\") default: *";
pub const MONTH: &str =
    "3. Month:\nEnter month (i.e. \"1\", \"3..5\", or \"2 4 6..11\") default: *";
pub const DAY: &str = "4. Day:\nEnter day (i.e. \"2\", \"13..15\", or \"20 24 26..28\") default: *";
pub const TIME: &str = "5. Time:\nEnter time (i.e. \"12:00:00\") default: 00:00:00";
pub const ENTER_SPEC: &str = "Enter the time spec > ";
pub const MORE_DETAIL: &str = "For more details, see systemd.timer(5) and systemd.time(7).";

pub const MONOTONIC_KIND: &str = "                     Relative to
1 OnActiveSec        the moment the timer unit is activated
2 OnBootSec          when the machines was booted up
3 OnBootSec          when the service manager was first started
4 OnUnitActiveSec    when the unit is activating was last activated
5 OnUnitInactiveSec  when the unit is activating was last deactivated";
