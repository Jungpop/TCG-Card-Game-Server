use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

pub trait GameFieldUnitRepository {
    fn create_game_field_unit_object(&mut self, account_unique_id: i32) -> bool;
    fn add_unit_to_game_field(&mut self,
                              account_unique_id: i32,
                              unit_card_number: i32,
                              unit_race: RaceEnum,
                              unit_grade: GradeEnum,
                              unit_attack_point: i32,
                              unit_health_point: i32,
                              unit_attack_required_energy: i32,
                              first_passive_skill: bool,
                              second_passive_skill: bool,
                              third_passive_skill: bool) -> i32;
    fn attach_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_enum: RaceEnum, quantity: i32);
    fn attach_multiple_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_number: i32, quantity: i32) -> bool;
    fn find_unit_by_id(&self, account_unique_id: i32, unit_card_number: i32) -> Option<&GameFieldUnitCard>;
    fn find_indexed_unit(&self, account_unique_id: i32, unit_card_index: i32) -> Option<&GameFieldUnitCard>;
    fn attach_multiple_energy_to_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32) -> bool;
    fn increase_max_health_of_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: usize, amount: i32) -> bool;

    fn find_target_unit_id_by_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32) -> i32;
    fn apply_damage_to_target_unit_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32, damage: i32) -> bool;
    fn apply_instant_death_to_target_unit_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32) -> bool;
    fn check_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool;
    fn reset_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool;
    fn execute_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool;
    fn judge_death_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> i32;
    fn attach_special_energy_to_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32, status_effect_list: Vec<StatusEffect>) -> bool;
    fn apply_harmful_status_effect_damage_iteratively(&mut self, account_unique_id: i32) -> bool;
    fn impose_harmful_state_to_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        harmful_state: ExtraStatusEffect,
    ) -> bool;
    fn acquire_unit_attack_point(
        &mut self,
        account_unique_id: i32,
        attacker_unit_index: i32
    ) -> i32;
    fn acquire_unit_extra_effect_by_index(
        &mut self,
        account_unique_id: i32,
        attacker_unit_index: i32
    ) -> &Vec<ExtraStatusEffect>;
    fn attack_target_unit_with_extra_effect(
        &mut self,
        opponent_unique_id: i32,
        opponent_unit_index: i32,
        damage: i32,
        extra_status_effect_list: Vec<ExtraStatusEffect>
    ) -> bool;
    fn apply_damage_to_every_unit(
        &mut self,
        opponent_unique_id: i32,
        damage: i32,
    ) -> bool;
    fn impose_extra_effect_state_to_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_index: i32,
        extra_effect_state: SummaryPassiveSkillEffect,
    ) -> bool;
    fn detach_multiple_energy_from_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        race_enum: RaceEnum,
        quantity: i32) -> bool;
    fn set_field_unit_deployed_round(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        current_round_value: i32) -> bool;
    fn get_field_unit_deployed_round(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32) -> i32;
}