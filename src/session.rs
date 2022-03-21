use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
}
