#[derive(Debug)]
pub struct SearchSpecificDeckCardRequest {
    account_unique_id: i32,
    target_card_id: i32,
}

impl SearchSpecificDeckCardRequest {
    pub fn new(account_unique_id: i32, target_card_id: i32) -> Self {
        SearchSpecificDeckCardRequest {
            account_unique_id,
            target_card_id,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_target_card_id(&self) -> i32 { self.target_card_id }
}