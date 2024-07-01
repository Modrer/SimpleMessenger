use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Message {
    pub message_type: String,
    pub message: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "{start} message:{}, type: \"{}\" {end}",
            // self.message,
            // self.message_type,
            // start = "{",
            // end = "}"
            "{}",
            serde_json::to_string(self).unwrap()
        )
    }
}
