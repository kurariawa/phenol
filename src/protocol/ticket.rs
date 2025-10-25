// TODO: impl ticket sign here (may impossible)

use serde::Serialize;

pub struct Ticket {
    key: String, // key itself
}

impl Ticket {
    pub fn new(key: String) -> Self {
        Self { key }
    }
    pub fn key(&self) -> &String {
        &self.key
    }
}

impl Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.key.serialize(serializer)
    }
}
