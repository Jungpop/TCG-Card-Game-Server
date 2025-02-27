#[derive(Debug)]
pub struct WaitQueueResponse {
    is_success:bool,

}

impl WaitQueueResponse {
    pub fn new(is_success:bool) -> Self {
        WaitQueueResponse {
            is_success
        }
    }

    pub fn get_is_success(&self) -> bool {
        self.is_success
    }

}
