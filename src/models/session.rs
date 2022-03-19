use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SingleSession {
    username: String,
    checkin_at: DateTime<Utc>,
    checkout_at: Option<DateTime<Utc>>,
    total_working_hour: Option<i32>,
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

    fn add(&mut self, session: SingleSession) {
        let username = session.username.to_string();
        match self.inner.get_mut(&username) {
            None => {
                let new_vec = vec![session];
                self.inner.insert(username, new_vec);
            }
            Some(sessions) => sessions.push(session),
        }
    }

    fn get_all(&mut self, name: &str) -> Option<&Vec<SingleSession>> {
        self.inner.get(name)
    }
}
