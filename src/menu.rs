use std::io;

use crate::session::{Sessions, SingleSession};

pub enum MainMenu {
    CheckIn,
    CheckOut,
    ViewAll,
    // Delete,
}

impl MainMenu {
    pub fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::CheckIn),
            "2" => Some(Self::CheckOut),
            "3" => Some(Self::ViewAll),
            _ => None,
        }
    }

    pub fn show() {
        println!("");
        println!(" == Work Reporter ==");
        println!("1. Check in");
        println!("2. Check out");
        println!("3. View All");
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
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }
}
