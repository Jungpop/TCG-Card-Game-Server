use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;

pub struct SupportCard_20_Function;

impl GameCardSupportHandler for SupportCard_20_Function {
    unsafe fn generate_support_card_effect_summary(&self) -> GameCardSupportEffect {
        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 0);
        game_card_support_effect.set_need_to_draw_card_count(2);

        game_card_support_effect
    }
}