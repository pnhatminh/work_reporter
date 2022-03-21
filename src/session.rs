use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Exception {
    CheckoutOk,
    UserNotFound,
    NoUnfinishedSession,
    // UnfinishedSessionExisted,
}

impl Exception {
    pub fn display(&self) {
        match self {
            Exception::CheckoutOk => println!("Checkout ok"),
            Exception::UserNotFound => println!("User not found"),
            Exception::NoUnfinishedSession => {
                println!("User cannot check out as there is no unfinished session")
            } // Exception::UnfinishedSessionExisted => {
              //     println!("User cannot check in as there is one unfinished session")
              // }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleSession {
    pub username: String,
    pub checkin_at: DateTime<Utc>,
    pub checkout_at: Option<DateTime<Utc>>,
    pub total_working_hour: Option<i32>,
}

pub struct Sessions {
    inner: HashMap<String, Vec<SingleSession>>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, session: SingleSession) {
        let username = session.username.to_string();
        match self.inner.get_mut(&username) {
            None => {
                let new_vec = vec![session];
                self.inner.insert(username, new_vec);
            }
            Some(sessions) => sessions.push(session),
        }
    }

    pub fn get_all(&self) -> Vec<SingleSession> {
        self.inner
            .values()
            .into_iter()
            .flatten()
            .cloned()
            .collect::<Vec<SingleSession>>()
    }

    pub fn get_by_name(&self, name: &str) -> Vec<SingleSession> {
        match self.inner.get(name) {
            None => vec![],
            Some(values) => values.to_vec(),
        }
    }

    pub fn update(&mut self, name: &str) -> Result<Exception, Exception> {
        if let Some(sessions) = self.inner.get_mut(name) {
            for session in sessions {
                if session.total_working_hour.is_none() && session.checkout_at.is_none() {
                    let checkout_at = chrono::offset::Utc::now();
                    let duration = checkout_at
                        .signed_duration_since(session.checkin_at)
                        .to_std()
                        .unwrap();
                    let total_working_hour = duration.as_millis() as i32 / 36000000;
                    session.checkout_at = Some(checkout_at);
                    session.total_working_hour = Some(total_working_hour);
                    return Ok(Exception::CheckoutOk);
                } else {
                    continue;
                }
            }
            return Err(Exception::NoUnfinishedSession);
        } else {
            Err(Exception::UserNotFound)
        }
    }
}
