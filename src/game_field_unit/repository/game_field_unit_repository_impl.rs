use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;

use crate::game_field_unit::entity::game_field_unit::GameFieldUnit;
use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;

pub struct GameFieldUnitRepositoryImpl {
    game_field_unit_map: IndexMap<i32, GameFieldUnit>,
}

impl GameFieldUnitRepositoryImpl {
    pub fn new() -> Self {
        GameFieldUnitRepositoryImpl {
            game_field_unit_map: IndexMap::new(),
        }
    }

    pub fn get_game_field_unit_map(&mut self) -> &mut IndexMap<i32, GameFieldUnit> {
        &mut self.game_field_unit_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameFieldUnitRepository for GameFieldUnitRepositoryImpl {
    fn create_game_field_unit_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameFieldUnitRepositoryImpl: create_game_field_unit_object()");

        let new_game_field_unit_map = GameFieldUnit::new();
        self.game_field_unit_map.insert(account_unique_id, new_game_field_unit_map);

        true
    }

    // TODO: unit 스펙 관련 사항이 필요함
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
                              third_passive_skill: bool) -> i32 {

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let unit_index = game_field_unit.add_unit_to_game_field(
                GameFieldUnitCard::new(
                    unit_card_number,
                    unit_race,
                    unit_grade,
                    unit_attack_point,
                    unit_health_point,
                    unit_attack_required_energy,
                    first_passive_skill,
                    second_passive_skill,
                    third_passive_skill,
                    true));

            unit_index
        } else {
            -1
        }
    }

    // TODO: 수량 1개 (enum 관련 사항을 어떻게 처리 할 것인가 고찰이 필요함)
    fn attach_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_enum: RaceEnum, quantity: i32) {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let race = RaceEnumValue::from(race_enum as i32);
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);
        }
    }

    // TODO: 여러 개 (enum 관련 사항을 어떻게 처리 할 것인가 고찰이 필요함)
    fn attach_multiple_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_number: i32, quantity: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let race = RaceEnumValue::from(race_number);
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);

            return true
        }

        return false
    }

    fn find_unit_by_id(&self, account_unique_id: i32, unit_card_number: i32) -> Option<&GameFieldUnitCard> {
        if let Some(game_field_unit) = self.game_field_unit_map.get(&account_unique_id) {
            game_field_unit.find_unit_by_id(unit_card_number)
        } else {
            None
        }
    }

    fn find_indexed_unit(&self, account_unique_id: i32, unit_card_index: i32) -> Option<&GameFieldUnitCard> {
        if let Some(game_field_unit) = self.game_field_unit_map.get(&account_unique_id) {
            Some(game_field_unit.find_unit_by_index(unit_card_index as usize))
        } else {
            None
        }
    }

    fn attach_multiple_energy_to_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32) -> bool {
        println!("GameFieldUnitRepositoryImpl: attach_multiple_energy_to_indexed_unit()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_energy_to_indexed_unit(unit_card_index as usize, RaceEnumValue::from(race_enum as i32), quantity);
            return true
        }

        return false
    }

    fn increase_max_health_of_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: usize, amount: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.increase_max_health_of_indexed_unit(unit_card_index, amount);
            true
        } else {
            false
        }
    }

    fn find_target_unit_id_by_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32) -> i32 {
        if let Some(opponent_game_field_unit) = self.game_field_unit_map.get(&opponent_unique_id) {
            if (0..opponent_game_field_unit.get_all_unit_list_in_game_field().len() as i32).contains(&opponent_target_unit_index) {
                let target_unit_card = &opponent_game_field_unit.get_all_unit_list_in_game_field()[opponent_target_unit_index as usize];

                return target_unit_card.get_card();
            }
        }
        -1
    }

    fn apply_damage_to_target_unit_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32, damage: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&opponent_unique_id) {
            let target_unit_index = opponent_target_unit_index as usize;

            if target_unit_index < game_field_unit.get_all_unit_list_in_game_field().len() {
                game_field_unit.apply_damage_to_indexed_unit(target_unit_index, damage);
                return true;
            }
        }

        false
    }

    fn apply_instant_death_to_target_unit_index(&mut self, opponent_unique_id: i32, opponent_target_unit_index: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&opponent_unique_id) {
            let target_unit_index = opponent_target_unit_index as usize;

            if target_unit_index < game_field_unit.get_all_unit_list_in_game_field().len() {
                game_field_unit.apply_death_to_indexed_unit(target_unit_index);
                return true;
            }
        }

        false
    }

    fn judge_death_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> i32 {
        println!("GameFieldUnitRepositoryImpl: judge_death_of_unit()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let unit_card_index = unit_card_index as usize;

            if unit_card_index < game_field_unit.get_all_unit_list_in_game_field().len() {
                let maybe_dead_unit_id = game_field_unit.judge_death_of_unit(unit_card_index);
                if game_field_unit.check_unit_alive(unit_card_index) == false {
                    return maybe_dead_unit_id;
                }
            }
        }

        -1
    }
    fn check_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool {
        let mut has_already_taken_action: bool = false;

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let unit_card_index = unit_card_index as usize;

            has_already_taken_action = game_field_unit.check_turn_action_of_unit(unit_card_index);
        }

        has_already_taken_action
    }

    fn execute_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let unit_card_index = unit_card_index as usize;
            game_field_unit.execute_turn_action_of_unit(unit_card_index);
            return true
        }

        false
    }
    fn reset_turn_action_of_unit(&mut self, account_unique_id: i32, unit_card_index: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let unit_card_index = unit_card_index as usize;
            game_field_unit.reset_turn_action_of_unit(unit_card_index);
            return true
        }

        false
    }


    fn attach_special_energy_to_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32, status_effect_list: Vec<StatusEffect>) -> bool {
        println!("GameFieldUnitRepositoryImpl: attach_special_energy_to_indexed_unit()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_special_energy_to_indexed_unit(
                unit_card_index as usize,
                RaceEnumValue::from(race_enum as i32),
                quantity,
                status_effect_list);

            return true
        }

        return false
    }

    // TODO: Game Field Unit이 너무 거대해지고 있음 (그러나 당장 고려 할 수 없는 상황임)
    fn apply_harmful_status_effect_damage_iteratively(&mut self, account_unique_id: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.apply_status_effect_damage_iteratively();
            true
        } else {
            false
        }
    }

    fn impose_harmful_state_to_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        harmful_state: ExtraStatusEffect,
    ) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.impose_harmful_state_to_indexed_unit(unit_card_index as usize, harmful_state);
            true
        } else {
            false
        }
    }

    fn acquire_unit_attack_point(
        &mut self,
        account_unique_id: i32,
        attacker_unit_index: i32
    ) -> i32 {
        let indexed_unit_reference = self.find_indexed_unit(account_unique_id, attacker_unit_index).unwrap();
        return indexed_unit_reference.get_unit_attack_point();
    }

    fn acquire_unit_extra_effect_by_index(
        &mut self,
        account_unique_id: i32,
        attacker_unit_index: i32
    ) -> &Vec<ExtraStatusEffect> {
        let indexed_unit_reference = self.find_indexed_unit(account_unique_id, attacker_unit_index).unwrap();
        return indexed_unit_reference.get_extra_status_effect_list();
    }

    fn attack_target_unit_with_extra_effect(
        &mut self,
        opponent_unique_id: i32,
        opponent_unit_index: i32,
        damage: i32,
        extra_status_effect_list: Vec<ExtraStatusEffect>
    ) -> bool {
        if let Some(mut indexed_unit_reference) = self.find_indexed_unit(opponent_unique_id, opponent_unit_index) {
            let mut mutable_indexed_unit_reference = indexed_unit_reference.to_owned(); // Assuming GameFieldUnitCard is Clone
            mutable_indexed_unit_reference.apply_damage(damage);
            mutable_indexed_unit_reference.impose_harmful_state_list(extra_status_effect_list);

            return true;
        }

        false
    }

    fn apply_damage_to_every_unit(
        &mut self,
        opponent_unique_id: i32,
        damage: i32,
    ) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&opponent_unique_id) {
            let all_units = game_field_unit.get_all_field_unit_list_mut();

            for unit in all_units.iter_mut() {
                unit.apply_damage(damage);
            }

            true
        } else {
            false
        }
    }

    fn impose_extra_effect_state_to_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_index: i32,
        extra_effect_state: SummaryPassiveSkillEffect,
    ) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.impose_extra_effect_state_to_indexed_unit(unit_index as usize, extra_effect_state);
            true
        } else {
            false
        }
    }

    fn detach_multiple_energy_from_indexed_unit(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        race_enum: RaceEnum,
        quantity: i32) -> bool {
        println!("GameFieldUnitRepositoryImpl: detach_multiple_energy_from_indexed_unit()");

        return if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.detach_energy_from_unit(
                unit_card_index as usize,
                RaceEnumValue::from(race_enum as i32),
                quantity);
            true
        } else {
            false
        }
    }

    fn set_field_unit_deployed_round(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32,
        current_round_value: i32) -> bool {

        println!("GameFieldUnitRepositoryImpl: set_field_unit_deployed_round()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.set_unit_deployed_round(unit_card_index as usize, current_round_value);
            return true
        }

        false
    }

    fn get_field_unit_deployed_round(
        &mut self,
        account_unique_id: i32,
        unit_card_index: i32) -> i32 {

        println!("GameFieldUnitRepositoryImpl: get_field_unit_deployed_round()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            return game_field_unit.get_unit_deployed_round(unit_card_index as usize);
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_field_unit_object() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        let result = game_field_unit_repository.create_game_field_unit_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameFieldUnitRepositoryImpl::get_instance();

        let mut lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.get_game_field_unit_map().len(), 0);
    }

    #[tokio::test]
    async fn test_add_unit_to_game_field() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        // assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_attach_energy_to_game_field_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Human,
            GradeEnum::Legend,
            35,
            30,
            1,
            false,
            false,
            false);

        println!("Initial state: {:?}", game_field_unit_repository.get_game_field_unit_map());

        // let race = RaceEnumValue::Undead;
        let race_enum = RaceEnum::Undead;
        let race = RaceEnumValue::from(race_enum as i32);
        let quantity = 1;
        game_field_unit_repository.attach_energy_to_game_field_unit(1, unit_card_number, race_enum, quantity);
        println!("After attaching energy: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let game_field_unit = game_field_unit_repository.get_game_field_unit_map().get(&1).unwrap();
        let attached_energy = game_field_unit.get_all_unit_list_in_game_field()[0].
            get_attached_energy().get_energy_quantity(&race);
        assert_eq!(attached_energy, Some(&quantity));
    }

    #[tokio::test]
    async fn test_find_unit_by_id() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Human,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            6,
            RaceEnum::Undead,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            2,
            RaceEnum::Trent,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            13,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        // assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let found_unit = game_field_unit_repository.find_unit_by_id(1, 2);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_some());

        let found_unit = found_unit.unwrap();
        assert_eq!(found_unit.get_card(), 2);

        let found_unit = game_field_unit_repository.find_unit_by_id(1, 12312);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_none());
    }

    #[tokio::test]
    async fn test_increase_max_health_of_indexed_unit_in_repository() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let current_max_health = game_field_unit_repository.get_game_field_unit_map()[&1]
            .get_all_unit_list_in_game_field()[0]
            .get_unit_health_point()
            .get_max_health_point();

        let increase_amount = 10;
        game_field_unit_repository.increase_max_health_of_indexed_unit(1, 0, increase_amount);

        let updated_max_health = game_field_unit_repository.get_game_field_unit_map()[&1]
            .get_all_unit_list_in_game_field()[0]
            .get_unit_health_point()
            .get_max_health_point();

        assert_eq!(updated_max_health, current_max_health + increase_amount);
    }

    #[tokio::test]
    async fn test_find_indexed_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Human,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            6,
            RaceEnum::Undead,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            2,
            RaceEnum::Trent,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            13,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let found_indexed_unit = game_field_unit_repository.find_indexed_unit(1,3);
        println!("{:?}", found_indexed_unit)
    }

    #[tokio::test]
    async fn test_find_target_unit_id_by_index() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let opponent_unique_id = 2;
        let opponent_target_unit_index = 1;

        let result = game_field_unit_repository.find_target_unit_id_by_index(opponent_unique_id, opponent_target_unit_index);
        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
        assert_eq!(result, -1);

        game_field_unit_repository.create_game_field_unit_object(2);
        game_field_unit_repository.add_unit_to_game_field(
            2,
            34,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let opponent_unique_id = 2;
        let opponent_target_unit_index = 0;

        let result = game_field_unit_repository.find_target_unit_id_by_index(opponent_unique_id, opponent_target_unit_index);
        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
        assert_eq!(result, 34);
    }

    #[tokio::test]
    async fn test_apply_damage_to_target_unit_index() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        game_field_unit_repository.add_unit_to_game_field(
            1,
            42,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false,
        );

        game_field_unit_repository.add_unit_to_game_field(
            1,
            42,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false,
        );

        game_field_unit_repository.add_unit_to_game_field(
            1,
            42,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false,
        );

        game_field_unit_repository.add_unit_to_game_field(
            1,
            42,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false,
        );

        game_field_unit_repository.add_unit_to_game_field(
            1,
            42,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false,
        );

        println!("Initial state: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let opponent_unique_id = 1;
        let opponent_target_unit_index = 3;
        let damage = 10;

        // Act
        let result = game_field_unit_repository.apply_damage_to_target_unit_index(
            opponent_unique_id,
            opponent_target_unit_index,
            damage,
        );

        assert!(result);

        println!("Final state: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_apply_damage_to_every_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        // Add multiple units to the game field
        for _ in 0..5 {
            game_field_unit_repository.add_unit_to_game_field(
                1,
                42,
                RaceEnum::Chaos,
                GradeEnum::Legend,
                35,
                30,
                2,
                false,
                false,
                false,
            );
        }

        println!("Initial state: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let opponent_unique_id = 1;
        let damage = 30;

        // Act
        let result = game_field_unit_repository.apply_damage_to_every_unit(opponent_unique_id, damage);

        assert!(result);


        let is_alive = game_field_unit_repository.judge_death_of_unit(opponent_unique_id, 1);

        println!("{}", is_alive);

        let game_field_unit_map = game_field_unit_repository.get_game_field_unit_map();
        for (_, game_field_unit) in game_field_unit_map.iter() {
            for unit in game_field_unit.get_all_unit_list_in_game_field().iter() {
                assert_eq!(unit.get_unit_health_point().get_current_health_point(), 00);
            }
        }


        println!("Final state: {:?}", game_field_unit_repository.get_game_field_unit_map());



    }

    #[test]
    fn test_detach_multiple_energy_from_indexed_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        // Add multiple units to the game field
        for _ in 0..3 {
            game_field_unit_repository.add_unit_to_game_field(
                1,
                42,
                RaceEnum::Chaos,
                GradeEnum::Legend,
                35,
                30,
                2,
                false,
                false,
                false,
            );
        }

        // Attach energy
        game_field_unit_repository.attach_multiple_energy_to_indexed_unit(
            1,
            1,
            RaceEnum::Chaos,
            3
        );

        game_field_unit_repository.attach_multiple_energy_to_indexed_unit(
            1,
            2,
            RaceEnum::Chaos,
            4
        );

        // Detach energy
        game_field_unit_repository.detach_multiple_energy_from_indexed_unit(
            1,
            1,
            RaceEnum::Chaos,
            2
        );

        game_field_unit_repository.detach_multiple_energy_from_indexed_unit(
            1,
            2,
            RaceEnum::Chaos,
            5
        );

        println!("{:?}", game_field_unit_repository.get_game_field_unit_map().get(&1))
    }

}
