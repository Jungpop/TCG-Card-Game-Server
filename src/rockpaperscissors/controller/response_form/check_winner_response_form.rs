use serde::{Deserialize, Serialize};
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CheckWinnerResponseForm {
    am_i_winner:bool,
    check_draw:bool,
}

impl CheckWinnerResponseForm {
    pub fn new(am_i_winner:bool,
               check_draw:bool, ) -> Self {
        CheckWinnerResponseForm {
            am_i_winner,
            check_draw

        }
    }
    pub fn get_am_i_winner(&self) -> bool { self.am_i_winner }
    pub fn get_check_draw(&self) -> bool { self.check_draw }


    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, support_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, support_card_number)
    }
    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }
}