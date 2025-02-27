use std::println;
use async_trait::async_trait;
use crate::game_field_unit::service::request::acquire_unit_attack_point_request::AcquireUnitAttackPointRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;

use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_field_unit::service::request::apply_catastrophic_damage_to_field_unit_request::ApplyCatastrophicDamageToFieldUnitRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_instant_death_to_target_unit_index_request::ApplyInstantDeathToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::apply_passive_skill_list_request::ApplyPassiveSkillListRequest;
use crate::game_field_unit::service::request::apply_status_effect_damage_iteratively_request::ApplyStatusEffectDamageIterativelyRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_special_energy_to_unit_index_request::AttachSpecialEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attack_target_unit_with_extra_effect_request::AttackTargetUnitWithExtraEffectRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_field_unit::service::request::check_turn_action_request::CheckTurnActionRequest;
use crate::game_field_unit::service::request::detach_multiple_energy_from_field_unit_request::DetachMultipleEnergyFromFieldUnitRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::find_active_skill_usage_unit_id_by_index_request::FindActiveSkillUsageUnitIdByIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_game_field_unit_card_of_account_uique_id_request::GetGameFieldUnitCardOfAccountUniqueIdRequest;
use crate::game_field_unit::service::response::acquire_unit_attack_point_response::AcquireUnitAttackPointResponse;
use crate::game_field_unit::service::response::acquire_unit_extra_effect_response::AcquireUnitExtraEffectResponse;

use crate::game_field_unit::service::response::add_unit_to_game_field_response::AddUnitToGameFieldResponse;
use crate::game_field_unit::service::response::apply_catastrophic_damage_to_field_unit_response::ApplyCatastrophicDamageToFieldUnitResponse;
use crate::game_field_unit::service::response::apply_damage_to_target_unit_index_response::ApplyDamageToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::apply_instant_death_to_target_unit_index_response::ApplyInstantDeathToTargetUnitIndexResponse;
use crate::game_field_unit::service::response::apply_passive_skill_list_response::ApplyPassiveSkillListResponse;
use crate::game_field_unit::service::response::apply_status_effect_damage_iteratively_response::ApplyStatusEffectDamageIterativelyResponse;
use crate::game_field_unit::service::response::attach_single_energy_to_unit_index_response::AttachSingleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_unit_index_response::AttachMultipleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_special_energy_to_unit_index_response::AttachSpecialEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attack_target_unit_with_extra_effect_response::AttackTargetUnitWithExtraEffectResponse;
use crate::game_field_unit::service::response::judge_death_of_unit_response::JudgeDeathOfUnitResponse;
use crate::game_field_unit::service::response::check_turn_action_response::CheckTurnActionResponse;
use crate::game_field_unit::service::response::detach_multiple_energy_from_field_unit_response::DetachMultipleEnergyFromFieldUnitResponse;
use crate::game_field_unit::service::response::execute_turn_action_response::ExecuteTurnActionResponse;
use crate::game_field_unit::service::response::find_active_skill_usage_unit_id_by_index_response::FindActiveSkillUsageUnitIdByIndexResponse;
use crate::game_field_unit::service::response::find_target_unit_id_by_index_response::FindTargetUnitIdByIndexResponse;
use crate::game_field_unit::service::response::get_current_attached_energy_of_field_unit_by_index_response::GetCurrentAttachedEnergyOfFieldUnitByIndexResponse;
use crate::game_field_unit::service::response::get_current_health_point_of_field_unit_by_index_response::GetCurrentHealthPointOfFieldUnitByIndexResponse;
use crate::game_field_unit::service::response::get_game_field_unit_card_of_account_uique_id_response::GetGameFieldUnitCardOfAccountUniqueIdResponse;

#[async_trait]
pub trait GameFieldUnitService {
    async fn add_unit_to_game_field(&mut self, add_unit_to_game_field_request: AddUnitToGameFieldRequest) -> AddUnitToGameFieldResponse;
    async fn attach_energy_to_field_unit_index(&mut self, attach_energy_to_unit_index_request: AttachSingleEnergyToUnitIndexRequest) -> AttachSingleEnergyToUnitIndexResponse;
    async fn attach_multiple_energy_to_field_unit_index(&mut self, attach_multiple_energy_to_unit_index_request: AttachMultipleEnergyToUnitIndexRequest) -> AttachMultipleEnergyToUnitIndexResponse;
    async fn find_target_unit_id_by_index(&mut self, find_target_unit_id_by_index_request: FindTargetUnitIdByIndexRequest) -> FindTargetUnitIdByIndexResponse;
    async fn apply_damage_to_target_unit_index(&mut self, apply_damage_to_target_unit_index_response: ApplyDamageToTargetUnitIndexRequest) -> ApplyDamageToTargetUnitIndexResponse;
    async fn apply_instant_death_to_target_unit_index(&mut self, apply_instant_death_to_target_unit_index_request: ApplyInstantDeathToTargetUnitIndexRequest) -> ApplyInstantDeathToTargetUnitIndexResponse;
    async fn judge_death_of_unit(&mut self, check_is_unit_alive_request: JudgeDeathOfUnitRequest) -> JudgeDeathOfUnitResponse;
    async fn check_turn_action(&mut self, check_turn_action_request: CheckTurnActionRequest) -> CheckTurnActionResponse;
    async fn execute_turn_action(&mut self, execute_turn_action_request: ExecuteTurnActionRequest) -> ExecuteTurnActionResponse;
    async fn reset_turn_action(&mut self, check_turn_action_request: CheckTurnActionRequest) -> CheckTurnActionResponse;
    async fn get_current_health_point_of_field_unit_by_index(&self, get_current_health_point_of_field_unit_by_index_request: GetCurrentHealthPointOfFieldUnitByIndexRequest) -> GetCurrentHealthPointOfFieldUnitByIndexResponse;

    async fn attach_special_energy_to_field_unit_index(&mut self, attach_special_energy_to_unit_index_request: AttachSpecialEnergyToUnitIndexRequest) -> AttachSpecialEnergyToUnitIndexResponse;
    async fn find_active_skill_usage_unit_id_by_index(&mut self, find_active_skill_usage_unit_id_by_index_request: FindActiveSkillUsageUnitIdByIndexRequest) -> FindActiveSkillUsageUnitIdByIndexResponse;
    async fn apply_status_effect_damage_iteratively(&mut self, apply_status_effect_damage_iteratively_request: ApplyStatusEffectDamageIterativelyRequest) -> ApplyStatusEffectDamageIterativelyResponse;
    async fn acquire_unit_attack_point(&mut self, acquire_unit_attack_point_request: AcquireUnitAttackPointRequest) -> AcquireUnitAttackPointResponse;
    async fn acquire_unit_extra_effect(&mut self, acquire_unit_extra_effect_request: AcquireUnitExtraEffectRequest) -> AcquireUnitExtraEffectResponse;
    async fn attack_target_unit_with_extra_effect(&mut self, attack_target_unit_with_extra_effect_request: AttackTargetUnitWithExtraEffectRequest) -> AttackTargetUnitWithExtraEffectResponse;
    async fn apply_passive_skill_list(&mut self, apply_passive_skill_list_request: ApplyPassiveSkillListRequest) -> ApplyPassiveSkillListResponse;
    async fn apply_catastrophic_damage_to_field_unit(&mut self, apply_catastrophic_damage_to_field_unit: ApplyCatastrophicDamageToFieldUnitRequest) -> ApplyCatastrophicDamageToFieldUnitResponse;
    async fn detach_multiple_energy_from_field_unit(&mut self, detach_multiple_energy_from_field_unit_request: DetachMultipleEnergyFromFieldUnitRequest) -> DetachMultipleEnergyFromFieldUnitResponse;
    async fn get_current_attached_energy_of_field_unit_by_index(&mut self, get_current_attached_energy_of_field_unit_by_index_request: GetCurrentAttachedEnergyOfFieldUnitByIndexRequest) -> GetCurrentAttachedEnergyOfFieldUnitByIndexResponse;
    async fn get_game_field_unit_card_of_account_unique_id(&mut self, get_game_field_unit_map_to_service_request: GetGameFieldUnitCardOfAccountUniqueIdRequest) -> GetGameFieldUnitCardOfAccountUniqueIdResponse;
}
