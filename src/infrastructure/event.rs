use crate::infrastructure::utils::now;

/////////////////////////////////////////
/// EventInfo
////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct EventInfo {
    event_id: String,
    time: u64,
}
impl EventInfo {
    pub fn new() -> Self {
        let time = now();
        Self {
            event_id: time.to_string(),
            time: time as u64,
        }
    }

    pub fn event_id(&self) -> String {
        self.event_id.clone()
    }

    pub fn time(&self) -> u64 {
        self.time
    }
}