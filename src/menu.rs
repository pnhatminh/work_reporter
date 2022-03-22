use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

use crate::session::{Sessions, SingleSession};

pub enum MainMenu {
    CheckIn,
    CheckOut,
    ViewAll,
    ExportCSV,
    // Delete,
}

impl MainMenu {
    pub fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::CheckIn),
            "2" => Some(Self::CheckOut),
            "3" => Some(Self::ViewAll),
            "4" => Some(Self::ExportCSV),
            _ => None,
        }
    }

    pub fn show() {
        println!("");
        println!(" == Work Reporter ==");
        println!("1. Check in");
        println!("2. Check out");
        println!("3. View All");
        println!("4. Export CSV");
        println!("");
        println!("Enter selection: ");
    }
}

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn check_in(sessions: &mut Sessions) {
    println!("Please enter username to checkin: ");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    let personal_sessions = sessions.get_by_name(&name);
    let not_checked_out_session = personal_sessions
        .into_iter()
        .filter(|x| x.checkout_at.is_none() && x.total_working_hour.is_none())
        .clone()
        .collect::<Vec<SingleSession>>();
    if not_checked_out_session.len() == 1 {
        println!(
            "You cannot check in, as there is an already session that is not yet checked out."
        );
        return;
    } else {
        let checkin_at = chrono::offset::Utc::now();

        let session = SingleSession {
            username: name,
            checkin_at,
            checkout_at: None,
            total_working_hour: None,
        };
        sessions.add(session);
        println!("Checked in at {:?}", checkin_at);
    }
}

pub fn view_all(sessions: &Sessions) {
    println!("Please enter username. Empty username return all sessions: ");
    let output = match get_input() {
        Some(name) => sessions.get_by_name(&name),
        None => sessions.get_all(),
    };
    for elem in output {
        println!("{:?}", elem);
    }
}

pub fn check_out(sessions: &mut Sessions) {
    println!("Please enter username to check out: ");

    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    match sessions.update(&name) {
        Ok(x) => x.display(),
        Err(x) => x.display(),
    }
}

pub fn export_csv(sessions: &Sessions) {
    println!("Please enter username. Empty username return all sessions: ");
    let output = match get_input() {
        Some(name) => sessions.get_by_name(&name),
        None => sessions.get_all(),
    };
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("exported.csv")
        .unwrap();
    if let Err(e) = writeln!(file, "Username,CheckInAt,CheckOutAt,TotalWorkingHour") {
        eprintln!("Couldn't write header to file: {}", e);
    }
    for elem in output {
        if let Err(e) = writeln!(file, "{}", elem.line_display()) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    println!("Exported Ok");
}
