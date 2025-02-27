#[derive(Debug)]
pub struct WaitQueueRequest {
    session_id: String,
    choice: String,

}

impl WaitQueueRequest {
    pub fn new(session_id: String,choice:String) -> Self {
        WaitQueueRequest {
            session_id,
            choice
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_choice(&self) -> &str {
        &self.choice
    }
}
