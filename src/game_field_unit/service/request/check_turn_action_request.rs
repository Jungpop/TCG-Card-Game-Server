#[derive(Debug)]
pub struct CheckTurnActionRequest {
    account_unique_id: i32,
    unit_card_index: i32,
}

impl CheckTurnActionRequest {
    pub fn new(account_unique_id: i32, unit_card_index: i32) -> Self {
        CheckTurnActionRequest {
            account_unique_id,
            unit_card_index,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

}
