pub const WELCOME: &str = "\u{23F2} Welcome to dottimer!\u{1F680}";
pub const ENTER_DESC: &str = "First, please enter the description of the timer.";
pub const TIMER_KIND: &str = "systemd timers are difined as one of two types:\nMonotonic timers activate after a time span relative to a varying starting point.\nRealtime timers activate on a calendar event, the same way that cronjobs do.";
pub const ENTER_KIND: &str = "Which kind of timer do you want?";
pub const KIND_LIST: &str = "1 Monotonic timers
2 Realtime timers
3 Both
";
pub const CHOOSE_MONOTONIC_KIND: &str = "Choose kinds of Monotonic timer(i.e. \"1\" or \"2 3 4\"):";
pub const PARSE_ERROR: &str = "Parse error: Enter again. > ";
pub const ENTER_SPAN :&str = "\nEnter the time span for each timer.\nExample: \"50\" for OnBootSec means 50s after boot-up. \nThe argument may also include time units.\nAnother example: \"5h 30min\" for OnBootSec means 5 hours and 30 minutes after boot-up.\nFor details about the syntax of time spans, see systemd.time(7).";
pub const OK_YN: &str = "OK? [Y/n] ";
pub const ENTER_AGAIN: &str = "Enter again. > ";
pub const IS_INTERACTIVE: &str = "Do you want the interactive input for time spec? [y/N] ";
pub const DOW: &str = "[1/5] the Day of Week:\n<Mon, Tue, Wed, Thu, Fri, Sat, Sun>";
pub const DOW_Q: &str = "Enter the days you want (i.e. \"Mon Wed\") default: None";
pub const YEAR: &str = "[2/5] Year:";
pub const YEAR_Q: &str =
    "Enter the year (i.e. \"2022\", \"2023..2025\", or \"2024 2025 2028..2030\") default: *";
pub const MONTH: &str = "[3/5] Month:";
pub const MONTH_Q: &str = "Enter the month (i.e. \"1\", \"3..5\", or \"2 4 6..11\") default: *";
pub const DAY: &str = "[4/5] Day:";
pub const DAY_Q: &str = "Enter the day (i.e. \"2\", \"13..15\", or \"20 24 26..28\") default: *";
pub const TIME: &str = "[5/5] Time:";
pub const TIME_Q: &str = "Enter the time (i.e. \"12:00:00\") default: 00:00:00";
pub const ENTER_SPEC: &str = "Enter the time spec > ";
pub const MORE_DETAIL: &str = "For more details, see systemd.timer(5) and systemd.time(7).";

pub const MONOTONIC_KIND: &str = "                     (Relative to)
1 OnActiveSec        the moment the timer unit is activated
2 OnBootSec          when the machines was booted up
3 OnStartupSec       when the service manager was first started
4 OnUnitActiveSec    when the unit is activating was last activated
5 OnUnitInactiveSec  when the unit is activating was last deactivated";

pub const OPTION_KIND: &str =
    "1 AccuracySec         specify the accuracy the timer shall elapse with
2 RandomizedDelaySec  delay the timer randomly (max: the value)
3 FixedRandomDelay    if true, the randomized offset is reused for all
                      (default: false)
4 OnClockChange       if true, the service unit will be triggered
                      when the system clock jumps relative to the monotonic clock
                      (default: false)
5 OnTimezoneChange    if true, the service unit will be triggered
                      when the local system timezone is modified
                      (default: false)
6 Persistent          If true, the time when the service unit was
                      last triggered is stored on disk.
                      When the timer is activated,
                      the service unit is triggered immediately if
                      it would have been triggered at least once
                      during the time when the timer was inactive.
                      (default: false)
7 WakeSystem          If true, an elapsing timer will cause
                      the system to resume from suspend
                      (default: false)
8 RemainAfterElapse   If true, a timer will stay loaded,
                      and its state remains queryable
                      (default: false)
";
pub const OPTION_Q: &str = "Choose kinds of option(i.e. \"1\" or \"2 3 4\"):";

pub const HELP: &str = "
dottimer: systemd timer generator that is not insane

OPTIONS:
    -H, --help
            Print help information
    -o
            Enable to choose options like AccuracySec or Persistent
";
