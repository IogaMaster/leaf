mod colors;
mod utils;

use colors::*;
use std::{env::var, str::from_boxed_utf8_unchecked};
use systemstat::{Duration, Platform, System};

fn main() {
    let sys = System::new();

    // get wm
    let wm = utils::get_wm().unwrap_or_else(|| String::from("unknown"));

    // get terminal
    let term = var("TERM").unwrap_or_else(|_| String::from("unknown"));

    // get shell
    let shell = utils::get_shell().unwrap_or_else(|| String::from("unknown"));

    // get uptime
    let uptime = sys.uptime().unwrap_or_else(|_| Duration::default());

    // format fetch text
    let fetch_text = vec![
        format!(
            "{WHITE}{}{RED}@{RESET}{}{BLUE}",
            whoami::username(),
            whoami::hostname()
        ),
        format!("{CYAN}os {WHITE}  ~ {CYAN}{}{BLUE}", whoami::distro()),
        format!("{YELLOW}up{WHITE} ~ {YELLOW}{uptime:?}{BLUE}"),
        format!("{GREEN}wm {WHITE}  ~ {GREEN}{wm}{BLUE}"),
        format!("{MAGENTA}term{WHITE} ~ {MAGENTA}{term}{BLUE}"),
        format!("{YELLOW_BRIGHT}sh {WHITE}  ~ {YELLOW_BRIGHT}{shell}{BLUE}"),
        format!("{RED}● {YELLOW}● {CYAN}● {BLUE}● {WHITE}●"),
    ]
    .join("\n");

    println!("{fetch_text}");
}
