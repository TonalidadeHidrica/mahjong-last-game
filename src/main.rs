use mahjong_last_game::{
    FinalGameState, GameConfig,
    agari_scores::{child_rong, child_tsumo, parent_rong, parent_tsumo},
};

fn main() {
    FinalGameState::new([-230, 331, 452, -573], [159, 349, 199, 298], 0, 0)
        .calc(GameConfig::m_league());
}
