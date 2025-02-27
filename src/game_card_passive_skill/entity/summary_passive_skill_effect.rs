use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;

#[derive(Debug, Clone, PartialEq)]
pub struct SummaryPassiveSkillEffect {
    passive_skill_type: PassiveSkillType,
    skill_damage: i32,
}

impl SummaryPassiveSkillEffect {
    pub fn new(passive_skill_type: PassiveSkillType,
               skill_damage: i32) -> Self {

        SummaryPassiveSkillEffect {
            passive_skill_type,
            skill_damage,
        }
    }

    pub fn get_passive_skill_type(&self) -> &PassiveSkillType {
        &self.passive_skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }
}
