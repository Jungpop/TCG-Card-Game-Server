use async_trait::async_trait;
use crate::notify_player_action::service::request::notify_to_opponent_you_deploy_unit_request::NotifyToOpponentYouDeployUnitRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_draw_support_card_request::NotifyToOpponentYouUseDrawSupportCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_boost_card_request::NotifyToOpponentYouUseEnergyBoostCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_card_request::NotifyToOpponentYouUseEnergyCardRequest;

use crate::notify_player_action::service::request::notify_to_opponent_you_use_field_energy_remove_support_card_request::NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_field_energy_increase_request::NotifyOpponentYouUseItemFieldEnergyIncreaseRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_instant_death_request::NotifyToOpponentYouUseItemInstantDeathRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_item_instant_death_alternatives_request::NotifyToOpponentYouUseItemInstantDeathAlternativesRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_search_support_card_request::NotifyOpponentYouUseSearchSupportRequest;
use crate::notify_player_action::service::response::notify_to_opponent_you_deploy_unit_response::NotifyToOpponentYouDeployUnitResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_draw_support_card_response::NotifyToOpponentYouUseDrawSupportCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_boost_card_response::NotifyToOpponentYouUseEnergyBoostCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_card_response::NotifyToOpponentYouUseEnergyCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_item_card_response::NotifyToOpponentYouUseItemCardResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_item_instant_death_alternatives_response::NotifyToOpponentYouUseItemInstantDeathAlternativesResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_support_card_response::NotifyOpponentYouUseSupportCardResponse;

#[async_trait]
pub trait NotifyPlayerActionService {
    async fn notify_to_opponent_what_you_do(&mut self, notify_to_opponent_what_you_do_request: NotifyToOpponentYouDeployUnitRequest) -> NotifyToOpponentYouDeployUnitResponse;
    async fn notify_to_opponent_you_use_energy_card(&mut self, notify_to_opponent_you_use_energy_card_request: NotifyToOpponentYouUseEnergyCardRequest) -> NotifyToOpponentYouUseEnergyCardResponse;
    async fn notify_to_opponent_you_use_energy_boost(&mut self, notify_to_opponent_you_use_energy_boost_card_request: NotifyToOpponentYouUseEnergyBoostCardRequest) -> NotifyToOpponentYouUseEnergyBoostCardResponse;
    async fn notify_to_opponent_you_use_item_instant_death(&mut self, notify_to_opponent_you_use_item_card_request: NotifyToOpponentYouUseItemInstantDeathRequest) -> NotifyToOpponentYouUseItemCardResponse;
    async fn notify_to_opponent_you_use_item_instant_death_alternatives(&mut self, notify_to_opponent_you_use_item_instant_death_alternatives_request: NotifyToOpponentYouUseItemInstantDeathAlternativesRequest) -> NotifyToOpponentYouUseItemInstantDeathAlternativesResponse;
    async fn notify_to_opponent_you_use_draw_support_card(&mut self, notify_to_opponent_you_use_draw_support_card_request: NotifyToOpponentYouUseDrawSupportCardRequest) -> NotifyToOpponentYouUseDrawSupportCardResponse;
    async fn notify_opponent_you_use_search_support_card(&mut self, notify_opponent_you_use_search_support_request: NotifyOpponentYouUseSearchSupportRequest) -> NotifyOpponentYouUseSupportCardResponse;
    async fn notify_opponent_you_use_field_energy_remove_support_card(&mut self, notify_to_opponent_you_use_field_energy_remove_support_card_request: NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest) -> NotifyOpponentYouUseSupportCardResponse;
    async fn notify_opponent_you_use_item_field_energy_increase_item_card(&mut self, notify_opponent_you_use_item_field_energy_increase_request: NotifyOpponentYouUseItemFieldEnergyIncreaseRequest) -> NotifyToOpponentYouUseItemCardResponse;
}
