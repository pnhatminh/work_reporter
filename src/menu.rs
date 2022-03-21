use std::io;

use crate::session::{Sessions, SingleSession};

pub enum MainMenu {
    CheckIn,
    CheckOut,
    ViewAll,
    // ViewOne,
    // ViewUser,
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
    println!("Please enter username: ");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };

    let checkin_at = chrono::offset::Utc::now();

    let session = SingleSession {
        username: name,
        checkin_at: checkin_at,
        checkout_at: None,
        total_working_hour: None,
    };
    sessions.add(session);
    println!("Session added at {:?}", checkin_at);
}
