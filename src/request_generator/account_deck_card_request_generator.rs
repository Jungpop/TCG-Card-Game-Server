use serde_json::Value as JsonValue;

use crate::account_deck_card::controller::request_form::account_deck_card_list_request_form::AccountDeckCardListRequestFrom;
use crate::account_deck_card::controller::request_form::account_deck_configuration_request_form::AccountDeckConfigurationRequestForm;

pub fn create_account_deck_configuration_request_form(data: &JsonValue) -> Option<AccountDeckConfigurationRequestForm> {
    if let (Some(deck_id), Some(card_list)) = (
        data.get("deckId").and_then(|v| v.as_i64()),
        data.get("cardIdList").and_then(|v| v.as_array())
    ) {
        let deck_id_i32 = deck_id as i32;
        let mut card_vec_i32 = Vec::new();

        for card_id_value  in card_list.iter() {
            if let Some(card_id) = card_id_value.as_i64() {
                let card_id_i32 = card_id as i32;
                card_vec_i32.push(card_id_i32);
            }
        }
        Some(AccountDeckConfigurationRequestForm::new(deck_id_i32, card_vec_i32))
    } else {
        None
    }
}

pub fn create_account_deck_card_list_request_form(data: &JsonValue) -> Option<AccountDeckCardListRequestFrom> {
    if let Some(deck_id) = (
        data.get("deckId").and_then(|v| v.as_i64())
    ) {
        let deck_id_i32 = deck_id as i32;
        Some(AccountDeckCardListRequestFrom::new(deck_id_i32))
    } else {
        None
    }
}