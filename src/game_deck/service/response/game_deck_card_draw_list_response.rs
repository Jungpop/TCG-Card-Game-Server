use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckCardDrawListResponse {
    draw_card_list: Vec<i32>,
}

impl GameDeckCardDrawListResponse {
    pub fn new(draw_card_list: Vec<i32>) -> Self {
        GameDeckCardDrawListResponse { draw_card_list }
    }
}
