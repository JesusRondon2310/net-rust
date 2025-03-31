use chrono::NaiveDateTime;
// use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct System {
    pub memory: String,
    pub timestamp: NaiveDateTime,
}

impl System {
    pub fn to_json(&self) -> String {
        match to_string_pretty(&self.clone()) {
            Ok(d) => d,
            Err(e) => {
                let e_msg = format!("error in json format : {e:#?}");
                eprint!("{e_msg}");
                return String::new();
            }
        }
    }
}
